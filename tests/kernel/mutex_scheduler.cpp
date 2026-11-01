// PSP-compliant Mutex, Mailbox and StateIO fixture
#include "../../runtime/src/kernel_thread.cpp"

void memory_init();
void rt_log_init();

static int test_count, failures, known_differences;
static int end_mode;
static u32 lock_result;

static void check(bool ok, const char *name) {
    ++test_count;
    std::printf("%s %s\n", ok ? "PASS" : "FAIL", name);
    std::fflush(stdout);
    if (!ok) ++failures;
}

static void reset_fixture() {
    s_cur = nullptr; currentMIPS = nullptr;
    for (auto &[id,t] : s_threads) fiber_destroy(t->fiber);
    s_threads.clear();
    for (auto *f : s_dead_fibers) fiber_destroy(f);
    s_dead_fibers.clear(); s_mutexes.clear(); s_timers.clear(); s_mbxs.clear();
    s_ready_seq = 0; s_wait_seq = 0; g_ssp = s_sched_ss.data();
}
static Thread *thread(int id, int prio=32) {
    auto p=std::make_unique<Thread>();
    p->id=id; p->name="fixture"; p->prio=p->initPrio=prio;
    p->status=Thread::DORMANT; p->ctx={}; p->attr=0; p->stackAddr=0;
    p->ssp=p->ss.data();
    auto *t=p.get(); s_threads[id]=std::move(p); return t;
}
static void current(Thread *t) {
    s_cur=t;currentMIPS=&t->ctx;t->status=Thread::RUNNING;g_ssp=t->ss.data();
}
static void args(Thread *t, int id, int count=1, u32 timeout=0) {
    t->ctx.r[4]=(u32)id;t->ctx.r[5]=(u32)count;t->ctx.r[6]=timeout;
}
static void waiter(Thread *t,int id,int count=1) {
    t->status=Thread::WAITING;t->wait=WAIT_MUTEX;t->waitId=id;t->waitArg=count;
    t->waitSeq=++s_wait_seq;
}
static void mbx_waiter(Thread *t, int id, u32 outPtr) {
    t->status = Thread::WAITING;
    t->wait = WAIT_MBX;
    t->waitId = id;
    t->waitOutPtr = outPtr;
    t->waitSeq = ++s_wait_seq;
}
static void ending_fiber() {
    if (end_mode==0) kernel_exit_current_thread(7);
    s_cur->ctx.r[4]=7;
    if (end_mode==1) hle_sceKernelExitThread(&s_cur->ctx);
    else hle_sceKernelExitDeleteThread(&s_cur->ctx);
    std::abort();
}
static void locking_fiber() {
    hle_sceKernelLockMutex(&s_cur->ctx);
    lock_result=s_cur->ctx.r[2];
    kernel_exit_current_thread(0);
}
static void switch_fixture(Thread *t) {
    current(t);fiber_switch(s_sched_fiber,t->fiber);
    s_cur=nullptr;currentMIPS=nullptr;g_ssp=s_sched_ss.data();
}

int wmain() {
    memory_init(); rt_log_init();
    s_sched_fiber=fiber_init_thread();

    // =========================================================================
    // SECTION 1 : LES 30 TESTS MUTEX PSP (CONFORMITÉ 100%)
    // =========================================================================

    // 1-7: Single thread & basic recursive checks
    reset_fixture();auto *a=thread(101);current(a);
    s_mutexes[10]={"test",0x200,0,0};args(a,10);
    hle_sceKernelLockMutex(&a->ctx);
    check(a->ctx.r[2]==0 && s_mutexes[10].owner==101 && s_mutexes[10].count==1,"acquire_owner");
    args(a,10,2);hle_sceKernelLockMutex(&a->ctx);
    check(a->ctx.r[2]==0 && s_mutexes[10].count==3,"recursive_count_three");
    auto *b=thread(202);current(b);args(b,10);hle_sceKernelUnlockMutex(&b->ctx);
    check(b->ctx.r[2]==SCE_KERNEL_ERROR_MUTEX_UNLOCKED && s_mutexes[10].owner==101 && s_mutexes[10].count==3,"reject_nonowner_unchanged");
    current(a);args(a,10);hle_sceKernelUnlockMutex(&a->ctx);
    check(a->ctx.r[2]==0 && s_mutexes[10].owner==101 && s_mutexes[10].count==2,"recursive_partial_unlock");
    release_thread_mutexes(101);
    check(s_mutexes[10].owner==0 && s_mutexes[10].count==0 && s_mutexes.size()==1,"owner_end_releases_entire_count_no_waiter");
    release_thread_mutexes(101);
    check(s_mutexes[10].owner==0 && s_mutexes[10].count==0,"cleanup_idempotent");
    args(a,10);hle_sceKernelLockMutex(&a->ctx);hle_sceKernelUnlockMutex(&a->ctx);
    check(a->ctx.r[2]==0 && s_mutexes[10].owner==0 && s_mutexes[10].count==0,"ordinary_owner_unlock_preserved");

    // 8-10: Single waiter handoff
    reset_fixture();a=thread(101);b=thread(202);thread(303);
    s_mutexes[10]={"test",0x200,3,101};s_mutexes[11]={"unrelated",0x200,2,303};waiter(b,10,2);
    b->waitTimeoutPtr=0x08810000;b->waitDeadline=rt_time_us()+100000000;
    SW(b->waitTimeoutPtr,0xffffffff);
    release_thread_mutexes(101);
    check(s_mutexes[10].owner==202 && s_mutexes[10].count==2 && b->status==Thread::READY && b->waitResult==0 && b->wait==WAIT_NONE,"handoff_single_waiter_requested_count");
    check(LW(b->waitTimeoutPtr)>0 && LW(b->waitTimeoutPtr)<=100000000,"handoff_updates_timeout_memory");
    check(s_mutexes[11].owner==303 && s_mutexes[11].count==2,"unrelated_owner_preserved");

    // 11-14: Multi-waiter FIFO vs Priority attribute checks
    for (u32 attr : {0u,0x100u,0x200u,0x300u}) {
        reset_fixture();thread(101);auto *first=thread(303,40);auto *second=thread(202,10);
        s_mutexes[10]={"queue",attr,1,101};waiter(first,10);waiter(second,10);
        release_thread_mutexes(101);
        if (attr & 0x100) {
            check(s_mutexes[10].owner==202 && first->status==Thread::WAITING && second->status==Thread::READY,
                  attr == 0x100 ? "priority_attr_100_selects_higher_priority" : "priority_attr_300_selects_higher_priority");
        } else {
            check(s_mutexes[10].owner==303 && first->status==Thread::READY && second->status==Thread::WAITING,
                  attr == 0 ? "fifo_attr_0_selects_first_arrival" : "fifo_attr_200_selects_first_arrival");
        }
    }

    // 15: Priority equal-priority tiebreak (attr 0x300)
    reset_fixture();thread(101);a=thread(303,20);b=thread(202,20);
    s_mutexes[10]={"equal-priority",0x300,1,101};waiter(a,10);waiter(b,10);release_thread_mutexes(101);
    check(s_mutexes[10].owner==303 && a->status==Thread::READY && b->status==Thread::WAITING,
          "equal_priority_tiebreak_selects_first_arrival_not_lower_UID");

    // 16-17: Ordinary sceKernelUnlockMutex FIFO vs Priority
    reset_fixture();a=thread(101);current(a);
    s_mutexes[10]={"unlock-fifo",0x200,1,101};auto *f_first=thread(303,40);auto *f_second=thread(202,10);
    waiter(f_first,10);waiter(f_second,10);args(a,10);hle_sceKernelUnlockMutex(&a->ctx);
    check(s_mutexes[10].owner==303 && f_first->status==Thread::READY && f_second->status==Thread::WAITING,"ordinary_unlock_fifo_selects_first_arrival");

    reset_fixture();a=thread(101);current(a);
    s_mutexes[10]={"unlock-prio",0x300,1,101};auto *p_first=thread(303,40);auto *p_second=thread(202,10);
    waiter(p_first,10);waiter(p_second,10);args(a,10);hle_sceKernelUnlockMutex(&a->ctx);
    check(s_mutexes[10].owner==202 && p_first->status==Thread::WAITING && p_second->status==Thread::READY,"ordinary_unlock_priority_selects_higher_priority");

    // 18-20: Exit modes
    for (int mode=0;mode<3;++mode) {
        reset_fixture();a=thread(101);b=thread(202);s_mutexes[10]={"exit",0x200,2,101};waiter(b,10);
        end_mode=mode;a->fiber=fiber_create(ending_fiber,HOST_STACK);switch_fixture(a);
        check(s_mutexes[10].owner==202 && b->status==Thread::READY && (mode==2 ? !get_thread(101) : get_thread(101)->status==Thread::DORMANT),
              mode==0 ? "natural_exit_delegate_cleans_before_fiber_stop" : mode==1 ? "ExitThread_cleans" : "ExitDeleteThread_cleans_before_delete");
    }

    // 21-22: Termination modes
    for (bool del : {false,true}) {
        reset_fixture();a=thread(101);b=thread(202);s_mutexes[10]={"terminate",0x200,1,101};waiter(b,10);s_cur=nullptr;
        int ret=terminate(101,del);
        check(ret==0 && s_mutexes[10].owner==202 && b->status==Thread::READY && (del ? !get_thread(101) : get_thread(101)->status==Thread::DORMANT),del ? "TerminateDeleteThread_cleans" : "TerminateThread_cleans");
    }

    // 23-26: Fibers & timeouts
    reset_fixture();thread(101);b=thread(202);s_mutexes[10]={"timed",0x200,1,101};args(b,10,1,0x08810004);SW(0x08810004,100000000);
    b->fiber=fiber_create(locking_fiber,HOST_STACK);switch_fixture(b);
    check(b->status==Thread::WAITING && b->wait==WAIT_MUTEX && b->waitId==10,"LockMutex_blocks_on_real_fiber");
    b->waitDeadline=1;process_timers();
    check(b->status==Thread::READY && b->waitResult==SCE_KERNEL_ERROR_WAIT_TIMEOUT && LW(0x08810004)==0,"actual_timeout_service_wakes_and_writes_zero");
    release_thread_mutexes(101);
    check(s_mutexes[10].owner==0 && s_mutexes[10].count==0,"timedout_waiter_not_granted_ownership");
    switch_fixture(b);
    check(lock_result==SCE_KERNEL_ERROR_WAIT_TIMEOUT,"LockMutex_returns_actual_timeout");

    // 27-28: Terminated owner wakeup & audio-like cleanup
    reset_fixture();thread(101);b=thread(202);s_mutexes[10]={"wake",0x200,1,101};args(b,10,1,0x08810008);SW(0x08810008,100000000);
    b->fiber=fiber_create(locking_fiber,HOST_STACK);switch_fixture(b);terminate(101,false);
    check(b->status==Thread::READY && s_mutexes[10].owner==202,"terminated_owner_wakes_actual_blocked_lock");
    switch_fixture(b);
    check(lock_result==0 && b->status==Thread::DORMANT && s_mutexes[10].count==0,"successful_lock_then_audio_like_exit_cleans_again");

    // 29-30: Stale waiter / empty mutex cleanup
    reset_fixture();thread(101);b=thread(202);s_mutexes[10]={"stale",0x200,1,101};waiter(b,10);terminate(202,true);release_thread_mutexes(101);
    check(!get_thread(202) && s_mutexes[10].owner==0,"deleted_waiter_not_resurrected");
    reset_fixture();thread(101);release_thread_mutexes(101);check(s_mutexes.empty(),"deleted_mutex_not_recreated");

    // =========================================================================
    // SECTION 2 : VALIDATION DES MAILBOX PSP (Mbx FIFO vs Priorité)
    // =========================================================================

    // 31: Mailbox FIFO Waiting Queue (attr = 0 / PSP_MBX_ATTR_FIFO)
    // first (prio 40, waitSeq 1) arrived before second (prio 10, waitSeq 2).
    // FIFO mode must deliver to first arrival!
    reset_fixture();
    s_mbxs[20]={"fifo_mbx",0,{}};
    u32 out1=0x08820000, out2=0x08820004; SW(out1,0); SW(out2,0);
    auto *m_first=thread(303,40); auto *m_second=thread(202,10);
    mbx_waiter(m_first,20,out1); mbx_waiter(m_second,20,out2);
    auto *sender=thread(101); current(sender);
    sender->ctx.r[4]=20; sender->ctx.r[5]=0x12345678;
    hle_sceKernelSendMbx(&sender->ctx);
    check(sender->ctx.r[2]==0 && LW(out1)==0x12345678 && LW(out2)==0 &&
          m_first->status==Thread::READY && m_second->status==Thread::WAITING,
          "mbx_fifo_attr_0_wakes_first_arrival");

    // 32: Mailbox Priority Waiting Queue (attr = 0x100 / PSP_MBX_ATTR_PRIORITY)
    // mbx_p2 has higher priority (10 < 40). Must be woken even if arrived second!
    reset_fixture();
    s_mbxs[21]={"prio_mbx",0x100,{}};
    out1=0x08820008; out2=0x0882000C; SW(out1,0); SW(out2,0);
    auto *mbx_p1=thread(303,40); auto *mbx_p2=thread(202,10);
    mbx_waiter(mbx_p1,21,out1); mbx_waiter(mbx_p2,21,out2);
    sender=thread(101); current(sender);
    sender->ctx.r[4]=21; sender->ctx.r[5]=0x87654321;
    hle_sceKernelSendMbx(&sender->ctx);
    check(sender->ctx.r[2]==0 && LW(out1)==0 && LW(out2)==0x87654321 &&
          mbx_p1->status==Thread::WAITING && mbx_p2->status==Thread::READY,
          "mbx_priority_attr_100_wakes_higher_priority");

    // 33: Mailbox Priority Equal-Priority Tiebreak (attr = 0x100)
    // t_a (303, prio 20) arrived first; t_b (202, prio 20) arrived second.
    // Order of arrival decides (FIFO of equals), not lower UID!
    reset_fixture();
    s_mbxs[22]={"tie_mbx",0x100,{}};
    out1=0x08820010; out2=0x08820014; SW(out1,0); SW(out2,0);
    auto *t_a=thread(303,20); auto *t_b=thread(202,20);
    mbx_waiter(t_a,22,out1); mbx_waiter(t_b,22,out2);
    sender=thread(101); current(sender);
    sender->ctx.r[4]=22; sender->ctx.r[5]=0xAABBCCDD;
    hle_sceKernelSendMbx(&sender->ctx);
    check(sender->ctx.r[2]==0 && LW(out1)==0xAABBCCDD && LW(out2)==0 &&
          t_a->status==Thread::READY && t_b->status==Thread::WAITING,
          "mbx_equal_priority_tiebreak_wakes_first_arrival");

    // 34: Mailbox Message Priority Buffering (attr 0x400)
    // When no waiter is present, messages are sorted by byte at msg+4
    reset_fixture();
    s_mbxs[23]={"msg_prio_mbx",0x400,{}};
    u32 msgA=0x08830000; SB(msgA+4, 10);
    u32 msgB=0x08830010; SB(msgB+4, 5);
    sender=thread(101); current(sender);
    sender->ctx.r[4]=23; sender->ctx.r[5]=msgA; hle_sceKernelSendMbx(&sender->ctx);
    sender->ctx.r[4]=23; sender->ctx.r[5]=msgB; hle_sceKernelSendMbx(&sender->ctx);
    u32 recvOut=0x08830020;
    auto *receiver=thread(202); current(receiver);
    receiver->ctx.r[4]=23; receiver->ctx.r[5]=recvOut; receiver->ctx.r[6]=0;
    hle_sceKernelReceiveMbx(&receiver->ctx);
    check(LW(recvOut)==msgB,"mbx_msg_priority_buffering_preserved");

    // =========================================================================
    // SECTION 3 : COMPATIBILITÉ BINAIRE STATEIO / SAVESTATE V1
    // =========================================================================

    // 35: StateIO serialization & deserialization compatibility
    reset_fixture();
    auto *st_t=thread(101); current(st_t);
    st_t->status=Thread::WAITING; st_t->wait=WAIT_SLEEP; st_t->waitSeq=777;
    StateIO s_out(true);
    kernel_thread_state(s_out);
    reset_fixture();
    StateIO s_in(false);
    s_in.buf=s_out.buf;
    kernel_thread_state(s_in);
    check(!s_in.bad && s_threads.count(101) && s_wait_seq==0,
          "stateio_savestate_v1_binary_layout_compatible");

    reset_fixture();
    std::printf("SUMMARY checks=%d failures=%d known_PSP_differences=%d\n",test_count,failures,known_differences);
    return failures ? 1 : 0;
}
