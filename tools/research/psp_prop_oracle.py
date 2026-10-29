"""Offline Europe PSP kinematics oracle. Never linked to the Rust runtime.
Run from repository root after installing unicorn into work/physics-oracle-deps.
Requires the user's decrypted work/psp-analysis/CTW-PSP.elf; outputs synthetic
input/output cases only, under ignored target/. No ROM/code is distributed.
"""
import sys,struct
from pathlib import Path
sys.path.insert(0,'work/physics-oracle-deps')
from unicorn import *
from unicorn.mips_const import *
b=Path('work/psp-analysis/CTW-PSP.elf').read_bytes()
import hashlib
assert hashlib.sha256(b).hexdigest() == '9eaf850f09d743e5ecfc8d1c6a0a8c290399142319b1ef3a732a03d7278bffe0', 'Wrong PSP build'
u=Uc(UC_ARCH_MIPS,UC_MODE_MIPS32|UC_MODE_LITTLE_ENDIAN)
u.mem_map(0x08800000,0x1000000)
phoff=struct.unpack_from('<I',b,28)[0];size,count=struct.unpack_from('<HH',b,42)

for i in range(count):
 typ,off,va,pa,fs,ms,fl,al=struct.unpack_from('<8I',b,phoff+i*size)
 if typ==1:

  data=bytearray(b[off:off+fs])
  # Allegrex MADD encoding -> standard MIPS32 MADD; same signed HI:LO semantics.
  if fl&1:
   for j in range(0,len(data)-3,4):
    w=struct.unpack_from('<I',data,j)[0]
    if w>>26==0 and w&63==28:struct.pack_into('<I',data,j,0x70000000|(w&0x03ffffc0))
  u.mem_write(va,bytes(data))

import random,json
rng=random.Random(0x13470101);cases=[]
for index in range(512):
 obj=0x09000000;u.mem_write(obj,bytes(0x400))
 mass=rng.choice([0,410,2048,4096,8192]);inv=0 if not mass else (1<<24)//mass
 frozen=index%17==0
 matrix=[rng.randrange(-8192,8193) for _ in range(9)]
 v=[rng.randrange(-300000,300001) for _ in range(3)]
 av=[rng.randrange(-32768,32769) for _ in range(3)]
 f=[rng.randrange(-2000000,2000001) for _ in range(3)] if index%5 else [0]*3
 torque=[rng.randrange(-100000,100001) for _ in range(3)] if index%7 else [0]*3
 acc=[31,-37,43]
 for off,values in [(0x6c,v),(0xa0,av),(0xac,acc),(0xb8,f),(0xc4,torque),(0x15c,[mass,inv])]:u.mem_write(obj+off,struct.pack('<'+'i'*len(values),*values))
 u.mem_write(obj+0x9b,bytes([frozen]));u.mem_write(obj+0x124,struct.pack('<9h',*matrix))
 u.reg_write(UC_MIPS_REG_A0,obj);u.reg_write(UC_MIPS_REG_SP,0x09110000);u.reg_write(UC_MIPS_REG_RA,0x09200000)
 u.emu_start(0x089b5960,0x09200000,count=10000)
 assert u.reg_read(UC_MIPS_REG_PC)==0x09200000
 output={key:list(struct.unpack('<3i',u.mem_read(obj+off,12))) for key,off in [('velocity',0x6c),('angular',0xa0),('acceleration',0xac),('force',0xb8),('torque',0xc4)]}
 cases.append(dict(mass=mass,inverse_mass=inv,disabled=frozen,matrix=matrix,velocity=v,angular=av,acceleration=acc,force=f,torque=torque,expected=output))
Path('target/psp-prop-oracle.json').write_text(json.dumps(cases))
print('512 original PSP kinematics executions completed; target/psp-prop-oracle.json')
