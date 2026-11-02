# Static PSP code-generation diagnostics

**99.99% displayed — 34,800/34,801 indexed PSP routines.**

![Static PSP translation coverage](assets/cpp-codegen-progress.svg)

## Technical diagnostic only

For this secondary diagnostic only, 100% means that every indexed routine meets the generated-body/object-compilation criterion. **It is not completed playable recompilation.** Overall readiness is defined in [CPP_PROGRESS.md](CPP_PROGRESS.md). This diagnostic uses original-routine-size weighting for the European PSP executable and all 166 overlays:

`100 × sum(original byte size of counted routines) / sum(original byte size of all indexed routines)`

Current numerator **12,307,572 original PSP bytes**; denominator **12,307,576 original PSP bytes**. This is **99.99996750% before display truncation**. The display truncates down to two decimals, so unresolved work never rounds up to 100.00%.

A routine counts only when its generated C++ body exists, its source SHA256 matches the compiled recipe, its Windows AMD64 COFF object exists and matches its recorded SHA256, and neither its original body nor hook wrapper contains an `rt_unhandled` operation. The hook's `_orig` body and wrapper count as one original routine. Runtime helpers, HLE declarations, tests and Rust code do not contribute to the numerator. 250/250 generated units passed the source/object checks in this measurement; no new compilation or game run was performed.

The original-size reference is `end - start` from the existing European function analysis, not C++ line count or x64 object size. These intervals exactly partition the executable/overlay code-size inventory. Analysis boundaries can include embedded data; this is coverage of that indexed scope, not a proof that every boundary is a real routine or every dynamic callback is known. The reference has 34,801 routine candidates; Android's 25,988-routine denominator is not used for PSP.

## Remaining excluded candidate and practical limits

Excluded candidates: mapapp — 4 original bytes (unsupported operation). Earlier static analysis strongly suggests that the mapapp candidate is embedded data misidentified as code. It remains grey until the classification is formally resolved. No original data, generated code or runtime is changed by the progress generator.

This high percentage reflects **automatic static translation**, which can translate most indexed code before all PSP runtime services work. It is not a percentage of gameplay validated, faithful behavior, performance, campaign completion or public-release readiness. Only two missions have been owner-tested so far. See [STATUS.md](STATUS.md).

PSP service source bindings are [another technical diagnostic](CPP_IMPORTS.md). Neither metric measures completed playable-port readiness.

## Method and provenance

The arithmetic uses the same original-routine-size weighting principle documented in [CTW-Native's progress script](https://github.com/official-kryo-to/ctw-native/blob/108320b137c3ca748d697838f86c8919c12b3f2c/scripts/progress.py). CTW-Native identifies routines in its port source by name references. This PSP diagnostic identifies automatically generated bodies and compiled objects. The source-name and generated-body criteria differ, so these percentages are not comparable port-completion scores. No Android counts, routine sizes, addresses or completion claims are transplanted.

Regenerate locally: `python tools/progress/cpp_status.py`. Implementation: tools/progress/cpp_recomp_progress.py. The original function inventory, source translations and objects remain private. Only aggregate module counts/byte totals, hashes and a generated visualization are published in [cpp-codegen-progress.json](assets/cpp-codegen-progress.json).

| PSP module | Original bytes | Recompiled bytes | Routines recompiled |
| --- | ---: | ---: | ---: |
| main | 3,346,064 | 3,346,064 | 12,222/12,222 |
| ammozoneapp | 42,464 | 42,464 | 81/81 |
| autosavequeryapp | 4,272 | 4,272 | 19/19 |
| bik_b01 | 130,828 | 130,828 | 329/329 |
| bik_b02 | 84,132 | 84,132 | 210/210 |
| bik_b03 | 78,724 | 78,724 | 193/193 |
| bik_b04 | 104,180 | 104,180 | 272/272 |
| bombdisposalapp | 16,744 | 16,744 | 32/32 |
| bombplantingapp | 8,296 | 8,296 | 30/30 |
| bugplantingapp | 6,912 | 6,912 | 24/24 |
| carbombfittingapp | 8,880 | 8,880 | 27/27 |
| caryard | 19,636 | 19,636 | 50/50 |
| charviewer | 2,284 | 2,284 | 11/11 |
| chinesedelivery | 52,020 | 52,020 | 149/149 |
| contactsapp | 5,776 | 5,776 | 21/21 |
| controlconfigapp | 9,960 | 9,960 | 35/35 |
| deatharrestapp | 1,296 | 1,296 | 12/12 |
| deedapp | 3,552 | 3,552 | 29/29 |
| desktopex | 10,432 | 10,432 | 28/28 |
| dlcnotifyapp | 1,104 | 1,104 | 11/11 |
| drugvanslashapp | 8,936 | 8,936 | 39/39 |
| dummyapp | 3,432 | 3,432 | 26/26 |
| dumpsterweaponstashapp | 39,480 | 39,480 | 99/99 |
| emailapp | 35,304 | 35,304 | 64/64 |
| endgame | 4,604 | 4,604 | 22/22 |
| enginesabotage | 16,824 | 16,824 | 42/42 |
| environmentsetups | 32,684 | 32,684 | 130/130 |
| explodingpolicevansapp | 12,048 | 12,048 | 35/35 |
| firefighter | 79,332 | 79,332 | 127/127 |
| friendlistapp | 11,160 | 11,160 | 32/32 |
| hes_a01 | 51,052 | 51,052 | 161/161 |
| hes_a02 | 47,932 | 47,932 | 141/141 |
| hes_a03 | 68,916 | 68,916 | 176/176 |
| hes_a04 | 58,396 | 58,396 | 175/175 |
| hes_b01 | 109,548 | 109,548 | 223/223 |
| hes_b02 | 63,660 | 63,660 | 178/178 |
| hes_b03 | 154,980 | 154,980 | 318/318 |
| hes_b04 | 57,628 | 57,628 | 168/168 |
| hes_b05 | 36,164 | 36,164 | 104/104 |
| hes_c01 | 158,068 | 158,068 | 428/428 |
| hes_c01b | 129,116 | 129,116 | 345/345 |
| hobotron | 38,340 | 38,340 | 93/93 |
| hostscreenpspapp | 19,856 | 19,856 | 76/76 |
| hostselectpspapp | 5,568 | 5,568 | 16/16 |
| hotwiringapp | 27,112 | 27,112 | 101/101 |
| hudapp | 49,456 | 49,456 | 140/140 |
| internettradingapp_psp | 15,208 | 15,208 | 46/46 |
| jao_a01 | 52,948 | 52,948 | 152/152 |
| jao_a02 | 59,220 | 59,220 | 156/156 |
| jao_a03 | 58,148 | 58,148 | 156/156 |
| jao_a04 | 175,676 | 175,676 | 397/397 |
| jao_a05 | 32,948 | 32,948 | 108/108 |
| jao_a06 | 70,524 | 70,524 | 269/269 |
| jao_a07 | 88,068 | 88,068 | 315/315 |
| jao_b01 | 81,996 | 81,996 | 188/188 |
| jao_b02 | 77,540 | 77,540 | 208/208 |
| jao_b03 | 70,268 | 70,268 | 156/156 |
| jao_b04 | 127,012 | 127,012 | 302/302 |
| jao_b05 | 57,284 | 57,284 | 186/186 |
| jao_b06 | 82,452 | 82,452 | 204/204 |
| jao_b07 | 93,556 | 93,556 | 222/222 |
| jao_c02 | 100,252 | 100,252 | 226/226 |
| jao_c02b | 87,876 | 87,876 | 222/222 |
| ken_a01 | 89,028 | 89,028 | 273/273 |
| ken_a02 | 136,748 | 136,748 | 406/406 |
| ken_a03 | 45,396 | 45,396 | 152/152 |
| ken_a04 | 52,748 | 52,748 | 157/157 |
| ken_a05 | 78,260 | 78,260 | 198/198 |
| ken_a06 | 131,820 | 131,820 | 263/263 |
| ken_a07 | 113,788 | 113,788 | 306/306 |
| ken_a08 | 92,124 | 92,124 | 244/244 |
| ken_a09 | 128,884 | 128,884 | 286/286 |
| ken_a10 | 77,452 | 77,452 | 208/208 |
| ken_a11 | 80,692 | 80,692 | 223/223 |
| ken_a12 | 28,988 | 28,988 | 93/93 |
| ken_b01 | 80,260 | 80,260 | 217/217 |
| ken_b02 | 152,460 | 152,460 | 347/347 |
| kor_b01 | 79,708 | 79,708 | 186/186 |
| kor_b02 | 148,916 | 148,916 | 308/308 |
| kor_b03 | 119,316 | 119,316 | 242/242 |
| kor_b04 | 123,636 | 123,636 | 261/261 |
| laptopapp | 8,080 | 8,080 | 25/25 |
| liftgatesmashapp | 13,832 | 13,832 | 50/50 |
| magneticcraneapp | 10,072 | 10,072 | 33/33 |
| mapapp | 59,144 | 59,140 | 75/76 |
| mel_a01 | 152,276 | 152,276 | 405/405 |
| mel_a02 | 167,484 | 167,484 | 502/502 |
| mel_a03 | 162,428 | 162,428 | 518/518 |
| messengerapp | 6,136 | 6,136 | 28/28 |
| mob_b01 | 51,468 | 51,468 | 148/148 |
| mob_b02 | 126,932 | 126,932 | 298/298 |
| mob_b03 | 126,876 | 126,876 | 284/284 |
| mob_b04 | 140,916 | 140,916 | 324/324 |
| molotovmakingapp | 30,176 | 30,176 | 54/54 |
| mpstatsapp | 1,904 | 1,904 | 15/15 |
| packagerun | 87,732 | 87,732 | 175/175 |
| paramedic | 32,380 | 32,380 | 104/104 |
| pausebriefapp | 5,048 | 5,048 | 19/19 |
| pausegameapp | 3,576 | 3,576 | 15/15 |
| pausestatsapp | 33,200 | 33,200 | 78/78 |
| pcjplayground01 | 22,284 | 22,284 | 58/58 |
| pcjplayground02 | 22,156 | 22,156 | 58/58 |
| pcjplayground03 | 22,228 | 22,228 | 58/58 |
| pcjplayground04 | 22,276 | 22,276 | 58/58 |
| pcjplayground05 | 22,236 | 22,236 | 58/58 |
| pcjplayground06 | 22,244 | 22,244 | 58/58 |
| pcjplayground07 | 22,284 | 22,284 | 58/58 |
| pcjplayground08 | 22,284 | 22,284 | 58/58 |
| pcjplayground09 | 22,292 | 22,292 | 58/58 |
| pcjplayground10 | 22,284 | 22,284 | 58/58 |
| pcjplayground11 | 22,276 | 22,276 | 58/58 |
| quicktradingapp | 6,792 | 6,792 | 29/29 |
| races_algon | 61,100 | 61,100 | 159/159 |
| races_east | 60,356 | 60,356 | 159/159 |
| races_north | 59,348 | 59,348 | 159/159 |
| radioapp | 11,720 | 11,720 | 42/42 |
| rampage | 56,548 | 56,548 | 131/131 |
| replaylevelapp | 23,384 | 23,384 | 54/54 |
| restartengineapp | 8,120 | 8,120 | 36/36 |
| rifleconstructionapp | 14,752 | 14,752 | 48/48 |
| rnd_ch01 | 46,612 | 46,612 | 110/110 |
| rnd_ch02 | 60,860 | 60,860 | 144/144 |
| rnd_ch03 | 44,652 | 44,652 | 124/124 |
| rnd_ch04 | 56,124 | 56,124 | 132/132 |
| rnd_ch05 | 46,972 | 46,972 | 106/106 |
| rnd_ch06 | 49,708 | 49,708 | 111/111 |
| rnd_ch07 | 66,356 | 66,356 | 151/151 |
| rnd_ch08 | 14,348 | 14,348 | 43/43 |
| safecrackingapp | 13,776 | 13,776 | 36/36 |
| safehouseapp | 88,848 | 88,848 | 206/206 |
| safehousesalehandler | 8,876 | 8,876 | 27/27 |
| scratchcardapp | 24,224 | 24,224 | 60/60 |
| scratchcards | 10,604 | 10,604 | 30/30 |
| scriptinit | 13,076 | 13,076 | 12/12 |
| searchingbodiesapp | 11,008 | 11,008 | 47/47 |
| settingsapp | 11,848 | 11,848 | 39/39 |
| shootingrange | 119,660 | 119,660 | 180/180 |
| singleplayerraceselectapp | 6,920 | 6,920 | 22/22 |
| sinkingcarapp | 27,760 | 27,760 | 66/66 |
| smashelectroniclockapp | 16,656 | 16,656 | 34/34 |
| tankfield | 35,076 | 35,076 | 89/89 |
| tattooingapp | 34,240 | 34,240 | 49/49 |
| tattooparlour | 9,172 | 9,172 | 31/31 |
| taxi | 31,756 | 31,756 | 75/75 |
| timetrial01 | 66,876 | 66,876 | 147/147 |
| timetrial02 | 58,612 | 58,612 | 133/133 |
| timetrial03 | 71,972 | 71,972 | 157/157 |
| timetrial04 | 63,444 | 63,444 | 145/145 |
| timetrial05 | 62,996 | 62,996 | 145/145 |
| tradestatsapp | 47,528 | 47,528 | 75/75 |
| turret | 38,388 | 38,388 | 97/97 |
| vancuttingapp | 12,136 | 12,136 | 38/38 |
| vigilante | 53,492 | 53,492 | 102/102 |
| vigilanteapp | 7,576 | 7,576 | 23/23 |
| warehouseoj | 58,804 | 58,804 | 130/130 |
| winchingapp | 7,656 | 7,656 | 23/23 |
| xin_01 | 35,324 | 35,324 | 107/107 |
| xin_02 | 22,652 | 22,652 | 71/71 |
| zho_a01 | 92,572 | 92,572 | 242/242 |
| zho_a02 | 72,116 | 72,116 | 181/181 |
| zho_a03 | 60,132 | 60,132 | 137/137 |
| zho_a04 | 43,828 | 43,828 | 116/116 |
| zho_b01 | 90,172 | 90,172 | 227/227 |
| zho_b02 | 56,772 | 56,772 | 168/168 |
| zho_b03 | 74,764 | 74,764 | 192/192 |
| zho_b04 | 69,532 | 69,532 | 166/166 |
| zho_c01 | 176,124 | 176,124 | 322/322 |
