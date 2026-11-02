"""Fail closed on private payloads in a Git tree and new reachable push objects.

This is a technical publication gate, not a legal clearance. Explicit source and
brand provenance review remains required. Never deletes or changes local files.
"""
import argparse, hashlib, json, re, subprocess, sys
from pathlib import Path, PurePosixPath
ROOT=Path(__file__).resolve().parents[2]
PRIVATE_PARTS={'work','target','private','extracted','generated','ms0','savedata','savestates','logs','node_modules'}
FORBIDDEN={'.iso','.cso','.nds','.apk','.elf','.exe','.dll','.bin','.pak','.wad','.toc','.bik','.at3','.at9','.dat','.sav','.ctwstate','.sfo','.pbp','.prx','.gim','.gxt','.mp4','.mp3','.wav','.ogg','.flac','.mkv','.avi','.webm','.bmp','.ppm','.dds','.tga','.zip','.7z','.obj','.o','.lib','.a','.pdb','.ttf','.otf','.woff','.woff2'}

def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT)

def inspect(path, data, branding):
    p=PurePosixPath(path); lower=path.lower(); problems=[]
    if set(s.lower() for s in p.parts)&PRIVATE_PARTS:problems.append('private directory')
    if p.suffix.lower() in FORBIDDEN:problems.append('private/binary/media extension')
    if re.search(r'(^|/)(recomp_\d+|recomp_ovl[^/]*|image)\.',lower):problems.append('generated game content')
    if len(data)>5_000_000:problems.append('large unreviewed blob')
    if data.startswith((b'MZ',b'\x7fELF',b'PK\x03\x04',b'RIFF',b'OggS',b'BIK')) or data[32769:32774]==b'CD001':problems.append('binary/game/archive signature')
    media=p.suffix.lower() in {'.png','.jpg','.jpeg','.webp','.gif','.svg'}
    if media and p.name not in {'progress.svg','badge-overall.svg','badge-routines.svg','badge-cpp.svg','cpp-progress.svg','badge-cpp-imports.svg','badge-cpp-recomp.svg','badge-cpp-routines.svg','cpp-imports.svg','cpp-codegen-progress.svg','badge-playable.svg'}:
        if branding.get(path)!=hashlib.sha256(data).hexdigest():problems.append('image not in reviewed branding allowlist')
    if re.search(rb'(ghp_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{30,}|-----BEGIN (?:RSA |OPENSSH )?PRIVATE KEY-----)',data):problems.append('possible secret')
    if p.suffix.lower()=='.svg' and re.search(rb'(?:href|src)\s*=\s*[\x22\x27]data:',data,re.I):problems.append('embedded image payload')
    return problems

def audit(tip, outgoing=False):
    branding=json.loads((ROOT/'docs/branding-publication.json').read_text(encoding='utf-8'))['files']
    records=[]
    for line in git('ls-tree','-rlz',tip).split(b'\0'):
        if not line:continue
        meta,name=line.split(b'\t',1);parts=meta.split()
        if parts[1]!=b'blob':continue
        records.append((parts[2].decode(),name.decode('utf-8')))
    if outgoing:
        for line in git('rev-list','--objects',tip,'--not','--remotes=origin').decode().splitlines():
            pair=line.split(' ',1)
            if len(pair)==2 and git('cat-file','-t',pair[0]).strip()==b'blob':records.append(tuple(pair))
    failures=[];seen=set()
    for sha,path in records:
        if (sha,path) in seen:continue
        seen.add((sha,path));data=git('cat-file','blob',sha)
        problems=inspect(path,data,branding)
        if problems:failures.append({'path':path,'blob':sha,'reasons':problems})
    return {'tip':tip,'outgoing_history_checked':outgoing,'blobs_checked':len(seen),'failures':failures}

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--tree',default='HEAD');p.add_argument('--outgoing',action='store_true');p.add_argument('--pre-push',action='store_true');p.add_argument('--report',type=Path)
    args=p.parse_args();results=[]
    if args.pre_push:
        for line in sys.stdin:
            local_ref,local_sha,remote_ref,remote_sha=line.split()
            if local_sha!='0'*40:results.append(audit(local_sha,True))
    else:results=[audit(args.tree,args.outgoing)]
    text=json.dumps(results,indent=2)
    if args.report:args.report.parent.mkdir(parents=True,exist_ok=True);args.report.write_text(text,encoding='utf-8')
    for r in results:
        print(f'Publication audit: {r["blobs_checked"]} blobs; {len(r["failures"])} blocked files')
        for f in r['failures']:print(f['path']+': '+', '.join(f['reasons']))
    raise SystemExit(1 if any(r['failures'] for r in results) else 0)

if __name__=='__main__':main()
