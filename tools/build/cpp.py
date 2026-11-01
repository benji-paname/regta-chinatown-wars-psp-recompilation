"""Local C++20 build. Private generated game code is never staged or distributed.

The validated local recipe lives in ignored work/cpp-mainline/build-config.json.
No downloads, game execution, save-directory writes or automatic Playtest update.
"""
import argparse, datetime, hashlib, json, os, re, shutil, subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

def digest(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def record(directory, name, command, cwd=None, env=None, timeout=180):
    proc = subprocess.run(list(map(str, command)), cwd=cwd or ROOT, env=env,
                          capture_output=True, text=True, encoding='utf-8', errors='replace', timeout=timeout)
    log = proc.stdout + proc.stderr
    (directory / (name + '.log')).write_text(log, encoding='utf-8')
    result = {'command':list(map(str, command)), 'cwd':str(cwd or ROOT), 'exit_code':proc.returncode,
              'warnings':len(re.findall(r'\bwarning:', log)), 'errors':len(re.findall(r'\berror:', log))}
    (directory / (name + '.json')).write_text(json.dumps(result, indent=2), encoding='utf-8')
    print(f'{name}: exit={proc.returncode}, warnings={result["warnings"]}, errors={result["errors"]}', flush=True)
    if proc.returncode:
        raise RuntimeError(log)
    return log

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--test', action='store_true')
    parser.add_argument('--runtime', action='store_true', help='Compile all 29 canonical runtime components and link locally')
    parser.add_argument('--full-game', action='store_true', help='Also rebuild 250 private generated translation units')
    args = parser.parse_args()
    config_file = ROOT / 'work/cpp-mainline/build-config.json'
    if not config_file.is_file():
        raise SystemExit('Private local build recipe is missing. See docs/BUILD_CPP.md; no game files are downloaded.')
    config = json.loads(config_file.read_text(encoding='utf-8'))
    expand = lambda text: str(text).replace('${ROOT}', str(ROOT))
    compiler = Path(config['compiler'])
    assert digest(compiler) == config['compiler_sha256'], 'Compiler differs from validated recipe'
    assert len(config['units']) == 279 and sum(u['generated'] for u in config['units']) == 250
    for header, sha in config['headers'].items():
        if digest(ROOT / header) != sha:
            raise SystemExit('Shared header changed: revalidate generated-code ABI and refresh the private recipe explicitly: '+header)
    for unit in config['units']:
        if not (ROOT / unit['source']).is_file():raise SystemExit('Missing local source: '+unit['source'])
        for flag in unit['flags']:
            if flag.startswith('-I') and not Path(expand(flag[2:])).is_dir():raise SystemExit('Missing include directory: '+expand(flag[2:]))
    if args.check:
        for unit in config['units']:
            assert digest(ROOT/unit['object']) == unit['object_sha256'], unit['object']
        print('279 sources/objects and headers checked; 250 generated/private, 29 runtime. No execution.')
        return
    if not args.test and not args.runtime and not args.full_game:args.runtime=True
    if args.full_game:args.runtime=True
    destination = ROOT / 'work/cpp-mainline' / datetime.datetime.now().strftime('build-%Y%m%d-%H%M%S')
    destination.mkdir(parents=True, exist_ok=False)
    objects=[]
    for unit in config['units']:
        source=ROOT/unit['source']; old=ROOT/unit['object']
        build = args.full_game or (args.runtime and not unit['generated'])
        if build:
            obj=destination/(unit['name']+'.obj')
            record(destination, 'compile-'+unit['name'], [compiler, *map(expand,unit['flags']),source,'-o',obj])
            record(destination, 'coff-'+unit['name'], [compiler.parent/'llvm-readobj.exe','--file-headers','--coff-directives',obj])
        else:
            assert digest(old)==unit['object_sha256'], 'Cached object changed: '+str(old)
            assert digest(source)==unit['source_sha256'], 'Source changed but not selected for rebuild: '+str(source)
            obj=old
        objects.append(obj)
    def link(name, inputs, test=False):
        exe=destination/(name+'.exe')
        options=[expand(s) for s in config['link_options']]
        if test:options=[s.replace('/ENTRY:mainCRTStartup','/ENTRY:wmainCRTStartup') for s in options]
        response=destination/(name+'.rsp')
        response.write_text('\n'.join([*options,f'/OUT:"{exe}"',f'/MAP:"{destination/(name+".map")}"',*[f'"{p}"' for p in inputs]])+'\n',encoding='utf-8')
        record(destination,'link-'+name,[compiler.parent/'lld-link.exe','@'+str(response)])
        return exe
    game = link('ctw_native',objects) if args.runtime else None
    tests=None
    if args.test:
        kernel=next(u for u in config['units'] if u['name']=='kernel_thread')
        fixture=ROOT/'tests/kernel/mutex_scheduler.cpp';obj=destination/'kernel_tests.obj'
        record(destination,'compile-tests',[compiler,*map(expand,kernel['flags']),fixture,'-o',obj])
        inputs=[obj if u['name']=='kernel_thread' else p for u,p in zip(config['units'],objects)]
        exe=link('kernel_tests',inputs,True)
        for dll in (ROOT/'outputs/ReGTA-CTW-Playtest').glob('*.dll'):shutil.copy2(dll,destination/dll.name)
        env={k:v for k,v in os.environ.items() if not k.startswith('CTW_')};env['CTW_LOG']='all'
        log=record(destination,'kernel-tests',[exe],destination,env,30)
        assert 'checks=35 failures=0 known_PSP_differences=0' in log, 'Incomplete kernel tests; inspect log'
        tests={'passed':35,'failed':0,'known_differences_in_covered_tests':0}
    result={'directory':str(destination.relative_to(ROOT)),'runtime_units_compiled':29 if args.runtime else 0,
            'generated_units_compiled':250 if args.full_game else 0,'tests':tests,
            'game':str(game.relative_to(ROOT)) if game else None,'game_sha256':digest(game) if game else None,
            'playtest_updated':False,'game_executed':False}
    (destination/'result.json').write_text(json.dumps(result,indent=2),encoding='utf-8')
    (ROOT/'work/cpp-mainline/latest-build.json').write_text(json.dumps(result,indent=2),encoding='utf-8')
    print(json.dumps(result,indent=2))

if __name__=='__main__':main()
