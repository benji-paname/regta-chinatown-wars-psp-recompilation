# SPDX-License-Identifier: GPL-3.0-only
# Copyright (C) 2026 ReGTA CTW Project & Benji Paname
"""Native Rust source inventory and status; no inferred original-game coverage."""
import argparse
import hashlib
import html
import json
import math
import re
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
SOURCE_DIR = ROOT / 'legacy/rust/src'

RUST_MODULE_MAP = {
    'handling.rs': ['CEngine', 'CTyre', 'cWheeledVehicle'],
    'traffic.rs': ['cWanderRoads', 'cAISpline', 'cDrivingRecovery'],
    'camera.rs': ['cCamera', 'cBaseCam', 'cFollowPedCam', 'cCam'],
    'car_camera.rs': ['cFollowCarCam2'],
    'vehicle.rs': [
        'cVehicle', 'cWheeledVehicle', 'cCar', 'cBike', 'Vehicle', 'VehicleType', 'cSeats',
        'cTrafficLight', 'cTrafficLightManager', 'cTrafficManager', 'cWanderRoads', 'cCarController',
        'cPayNSpray', 'cPayNSprayManager',
        'cBoat', 'cAIBoat', 'cAmbientBoatManager', 'cBoatEngine', 'cBoatWakeEmitter', 'cBoatSprayEmitter',
        'cCarInWater', 'cPoliceBoat',
        'cSkidmark', 'cSkidmarkManager', 'cCarGen', 'cCarGenManager',
        'cVehicleDoor', 'cDoor', 'cDoorManager', 'cRollOutOfVehicle', 'cTank', 'cParticleEmitterSkidSmoke'
    ],
    'ped.rs': ['cPed', 'cPlayer', 'cPopulationManager', 'cPedSprite', 'cAnimation'],
    'actors.rs': ['cAIGang', 'cAITaskHub', 'cSimpleMover', 'cPlayerOnFoot', 'cPlayerBoat', 'cLeaveWater', 'cPlayerPickup', 'cBodySearch'],
    'vault.rs': ['cTaskVault', 'cSimpleTrail'],
    'roll.rs': ['cTaskRoll', 'cOneShotAnimationTask'],
    'weapons.rs': [
        'cWeapon', 'cWeaponManager', 'cBulletBasedWeapon', 'cAK47', 'cTazer', 'cSensorCone',
        'cBaseProjectile', 'cGrenade', 'cMolotov', 'cMolotovManager', 'cRocket', 'cRocketLauncher',
        'cAIRocket', 'cFlameThrower', 'cSniperRifle', 'cPlayerAsSniper', 'cSniperCam',
        'cBulletTrace', 'cBulletTraces', 'cParticleEmitterFlameThrower',
        'cBaseWeaponPickup', 'cPickupBat', 'cPickupChainsaw', 'cPickupColt', 'cPickupDesertEagle',
        'cPickupDualPistol', 'cPickupFlameThrower', 'cPickupGrenade', 'cPickupMinigun', 'cPickupMolotov',
        'cPickupMp5', 'cPickupNiteStick', 'cPickupPistol', 'cPickupProximityMine', 'cPickupRocketLauncher',
        'cPickupShotgun', 'cPickupSniperRifle', 'cPickupSword', 'cPickupTazer', 'cPickupUzi',
        'cPickupAk47', 'cPickupDoubleBarrelShot', 'cPickupSawnOffShot', 'cPickupFlashBang',
        'cPickupHealth', 'cPickupArmour', 'cPickupBribe', 'cPickupMoney', 'cPickupManager',
        'cBasePickup', 'cPickupGen', 'cPickupGeneric', 'cPickupLuckLion', 'cPickupRandomStuff',
        'cPickupListener', 'cCollectedAmbientPickupStore',
        'cHudWeaponSelect', 'cWeaponSlotIcon'
    ],
    'explosions.rs': ['Explosion', 'cBaseExplosion', 'FadeExplosion'],
    'collision.rs': ['CCollision', 'CCollisionBox', 'CCollisionMesh'],
    'world.rs': ['WorldImpl', 'cWorldSector', 'Area', 'cPhysicalIntegrator', 'cPhysical'],
    'props.rs': ['cDynamicProp', 'cZoneManager'],
    'psp_prop_physics.rs': ['cPhysical'],
    'pavement.rs': ['cLocalAreaKnowledge', 'cRoadNetwork'],
    'ped_paths.rs': ['cPedRoute', 'cPedPath'],
    'hud.rs': ['Gui', 'HUDText', 'cMiniHud', 'cInWaterHelpText'],
    'gpu.rs': ['cWaterRenderBlock', 'cParticleEmitterWaterSplash', 'cPickupEffectRenderable'],
    'radar.rs': ['cRadar', 'cRadarBlip', 'cTurfMap'],
    'gps.rs': ['cGPS', 'cRouteManager'],
    'pda.rs': ['PDAImpl', 'cPda', 'cEmailApp', 'cPhoneApp', 'cIPhonePad', 'cContextHelp', 'cContainerWnd', 'cWnd', 'cListBox'],
    'audio.rs': ['cAudio', 'cAudioManager', 'cAudioBase', 'SoundImpl', 'cSoundEvents'],
    'radio.rs': ['cRadioApp', 'cSoundStream', 'cAudioBaseOAL'],
    'timecycle.rs': ['cTimeCycle', 'cCycleElement'],
    'weather.rs': ['cWeather', 'cTimeCycle', 'cLightManager', 'Fire', 'cFire', 'cParticleEmitterFire'],
    'races.rs': ['CRaces_Algon', 'CRaces_East', 'CRaces_North', 'cRace', 'cCheckpointTT'],
    'collectibles.rs': ['CRampage', 'CTurretRampage', 'cSecurityCamera'],
    'safehouse.rs': ['cSafehouse', 'cStash', 'cWardrobe', 'cSafehouseApp', 'cSafehouseManager', 'cSafehouseInfo', 'cSafehouseHandler', 'cSafehouseSaleHandler'],
    'drugs.rs': ['cDrugSystem', 'cDealer', 'cTrade', 'cBaseTradeApp', 'cTradeApp', 'cTradeQuantity', 'cTradeItem', 'cTradeManager', 'cTradeStorageApp', 'cAmbDealer', 'cDealerInfo'],
    'jobs.rs': ['cOddJob', 'Taxi', 'cParamedic', 'cFleeOnFoot'],
    'police.rs': [
        'cWantedManager', 'cWantedRating', 'cPoliceChase', 'cMonitorWantedLevel',
        'cPoliceHeli', 'cHeliSearchLight', 'cHelicopterManager', 'cRoadBlock', 'cSpikeStrip'
    ],
    'stats.rs': ['cStatsManager', 'StatsImpl', 'PopulationProfile'],
    'save.rs': ['SaveGameImpl', 'cSaveGameMgr'],
    'missions.rs': ['CScriptInit', 'CScriptMain', 'ScriptHnd', 'cMission', 'cActStructure', 'cCutscene_Intro', 'cCutscene_Outro', 'cCutscene_AtKennys', 'cDeathArrestApp'],
    'minigames.rs': [
        'cHotWiringApp', 'cScratchCardApp', 'cShootingRange', 'cTattooingApp',
        'cSinkingCarApp', 'cSafeCrackingApp', 'cDumpsterWeaponStashApp', 'cCarBombFittingApp',
        'cMolotovApp', 'cRifleConstructionApp', 'cSearchingBodiesApp',
        'cAmmozoneApp', 'cMapApp'
    ],
}

GREEN = '#3fa34d'
GREY = '#4d5257'
DARK = '#16191c'
INK = '#e6e8ea'
FONT = 'Verdana,DejaVu Sans,sans-serif'


def squarify(values, x, y, w, h):
    """Bruls et al. squarified treemap."""
    items = [(v, i) for i, v in enumerate(values) if v > 0]
    total = sum(v for v, _ in items)
    if not items or w <= 0 or h <= 0:
        return [None] * len(values)
    scale = w * h / total
    items = [(v * scale, i) for v, i in items]
    out = [None] * len(values)
    while items:
        short = min(w, h)
        row, rest = [items[0]], items[1:]

        def worst(r):
            s = sum(a for a, _ in r)
            return max(max(short * short * a / (s * s), (s * s) / (short * short * a)) for a, _ in r)

        while rest and worst(row + [rest[0]]) <= worst(row):
            row.append(rest.pop(0))
        s = sum(a for a, _ in row)
        if w >= h:   # column on left
            cw = s / h
            yy = y
            for a, i in row:
                out[i] = (x, yy, cw, a / cw)
                yy += a / cw
            x, w = x + cw, w - cw
        else:        # row along top
            rh = s / w
            xx = x
            for a, i in row:
                out[i] = (xx, y, a / rh, rh)
                xx += a / rh
            y, h = y + rh, h - rh
        items = rest
    return out


def badge(label, value, colour):
    lw, vw = 7 * len(label) + 12, 7 * len(value) + 12
    w = lw + vw
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="20" role="img" '
        f'aria-label="{label}: {value}">'
        f'<rect width="{lw}" height="20" fill="#555"/>'
        f'<rect x="{lw}" width="{vw}" height="20" fill="{colour}"/>'
        f'<g fill="#fff" font-family="{FONT}" font-size="11" text-anchor="middle">'
        f'<text x="{lw / 2}" y="14">{label}</text>'
        f'<text x="{lw + vw / 2}" y="14">{value}</text>'
        f'</g></svg>\n'
    )


def mask_noncode(text):
    """Mask comments and string literals, keeping offsets for cfg item filtering."""
    token = re.compile(r'//[^\n]*|/\*|r(\#*)"|b?"(?:\\.|[^"\\])*"|b?\'(?:\\.|[^\'\\])\'', re.S)
    chars = list(text)
    at = 0
    while match := token.search(text, at):
        start, end = match.span()
        if match.group() == '/*':
            depth = 1
            while depth and end < len(text):
                nxt = re.search(r'/\*|\*/', text[end:])
                if not nxt:
                    end = len(text)
                    break
                depth += 1 if nxt.group() == '/*' else -1
                end += nxt.end()
        elif match.group().startswith('r'):
            close = '"' + match.group(1)
            found = text.find(close, end)
            end = len(text) if found < 0 else found + len(close)
        chars[start:end] = ['\n' if c == '\n' else ' ' for c in text[start:end]]
        at = end
    return ''.join(chars)


def source_functions(text):
    code = mask_noncode(text)
    # These are source declarations, including platform-specific branches, not
    # proof of compiled reachability or correspondence to an original routine.
    def count(s):
        definitions = 0
        for m in re.finditer(r'\bfn\s+[A-Za-z_][A-Za-z_0-9]*', s):
            # Array type semicolons are not declaration terminators.
            square = parens = 0
            for ch in s[m.end():]:
                if ch == '[': square += 1
                elif ch == ']': square -= 1
                elif ch == '(': parens += 1
                elif ch == ')': parens -= 1
                elif square == 0 and parens == 0 and ch in '{;':
                    definitions += ch == '{'
                    break
        return definitions
    all_functions = count(code)
    if re.search(r'#!\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]', code):
        return 0, all_functions
    chars = list(code)
    for m in re.finditer(r'#\s*\[\s*(?:cfg\s*\(\s*test\s*\)|test)\s*\]', code):
        opening = code.find('{', m.end())
        if opening < 0:
            continue
        depth, end = 1, opening + 1
        while depth and end < len(code):
            depth += (code[end] == '{') - (code[end] == '}')
            end += 1
        chars[m.start():end] = ' ' * (end - m.start())
    runtime = count(''.join(chars))
    return runtime, all_functions - runtime


GROUPS = {
    'Vehicles': 'handling traffic vehicle',
    'People': 'actors ped vault roll pavement ped_paths',
    'World and rendering': 'world collision props gpu shader texture model render camera',
    'HUD and navigation': 'gps hud radar pda ui',
    'Combat': 'weapons explosions',
    'Missions and activities': 'missions minigames races collectibles jobs drugs safehouse',
    'Audio and weather': 'audio radio weather',
    'Saves and police': 'stats save police',
}


def measure_rust(source_dir=SOURCE_DIR):
    groups = {}
    digest = hashlib.sha256()
    for path in sorted(source_dir.rglob('*.rs')):
        data = path.read_bytes()
        name = path.relative_to(source_dir).as_posix()
        digest.update(name.encode() + b'\0' + data)
        runtime, tests = source_functions(data.decode('utf-8'))
        group = next((g for g, modules in GROUPS.items() if path.stem in modules.split()), 'Runtime and tools')
        sub = groups.setdefault(group, {'name': group, 'runtimeFunctions': 0, 'testFunctions': 0, 'classes': []})
        sub['runtimeFunctions'] += runtime
        sub['testFunctions'] += tests
        sub['classes'].append({'name': name, 'runtimeFunctions': runtime, 'testFunctions': tests,
                              'relatedClasses': RUST_MODULE_MAP.get(path.name, [])})
    return {'schema': 2, 'measurement': 'native Rust function inventory; not original-code coverage',
            'pspParityVerified': False, 'pspCompletionPercent': None, 'sourceSha256': digest.hexdigest(),
            'runtimeFunctions': sum(s['runtimeFunctions'] for s in groups.values()),
            'testFunctions': sum(s['testFunctions'] for s in groups.values()),
            'modules': sum(len(s['classes']) for s in groups.values()),
            'subsystems': list(groups.values())}


def treemap(result, width=1000, height=560):
    parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" font-family="{FONT}">',
             f'<rect width="{width}" height="{height}" fill="{DARK}"/>',
             f'<text x="8" y="18" font-size="13" fill="{INK}">Native Rust inventory: {result["runtimeFunctions"]} runtime functions | grey: tests/helpers | PSP completion: unmeasured</text>']
    size = lambda item: item['runtimeFunctions'] + item['testFunctions']
    subs = sorted(result['subsystems'], key=lambda s: -size(s))
    boxes = squarify([size(s) for s in subs], 2, 26, width-4, height-28)
    for sub, box in zip(subs, boxes):
        if not box: continue
        sx, sy, sw, sh = box
        modules = sorted(sub['classes'], key=lambda c: -size(c))
        cells = squarify([size(c) for c in modules], sx+1, sy+17, sw-2, max(1, sh-18))
        for c, cell in zip(modules, cells):
            if not cell: continue
            x, y, w, h = cell
            title = html.escape(f'{c["name"]}: {c["runtimeFunctions"]} runtime, {c["testFunctions"]} test/helper functions')
            parts.append(f'<g><title>{title}</title><rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" fill="{GREY}" stroke="{DARK}"/>')
            green_h = h * c['runtimeFunctions'] / max(1,size(c))
            parts.append(f'<rect x="{x:.1f}" y="{y+h-green_h:.1f}" width="{w:.1f}" height="{green_h:.1f}" fill="{GREEN}" stroke="{DARK}"/>')
            if w > len(c['name'])*5.5+5 and h > 15:
                parts.append(f'<text x="{x+3:.1f}" y="{y+12:.1f}" font-size="9" fill="{INK}">{html.escape(c["name"])}</text>')
            parts.append('</g>')
        parts.append(f'<rect x="{sx:.1f}" y="{sy:.1f}" width="{sw:.1f}" height="{sh:.1f}" fill="none" stroke="{INK}"/>')
        label = f'{sub["name"]} ({sub["runtimeFunctions"]})'
        if sw < len(label)*6.2: label = sub['name']
        if sw > len(label)*6.2:
            parts.append(f'<text x="{sx+4:.1f}" y="{sy+12:.1f}" font-size="11" fill="{INK}">{html.escape(label)}</text>')
    return '\n'.join(parts) + '\n</svg>\n'


NOTE = ('Green counts runtime function definitions in legacy/rust/src/**/*.rs; grey counts test and test-helper definitions. '
        'Area represents function count, not original binary size. No C++ code or Android routine totals enter these measurements. '
        'These source counts do not prove gameplay fidelity or a percentage of the PSP game completed. '
        'Related class names are research navigation hints, not verified routine matches.')


def table(result):
    rows = []
    for s in sorted(result['subsystems'], key=lambda s: -s['runtimeFunctions']):
        rows.append(f'<tr><td>{html.escape(s["name"])}</td><td>{len(s["classes"])}</td><td>{s["runtimeFunctions"]}</td><td>{s["testFunctions"]}</td></tr>')
    return '<table><thead><tr><th>Subsystem</th><th>Modules</th><th>Runtime functions</th><th>Test/helper functions</th></tr></thead><tbody>' + ''.join(rows) + '</tbody></table>'


def validation():
    path = ROOT / 'target/validation/build-report.json'
    if path.exists():
        report = json.loads(path.read_text(encoding='utf-8-sig'))
        tests = report.get('tests')
        if tests and tests['failed'] == 0 and report['warnings'] == 0:
            return tests['passed'], report['timestamp'][:10]
    return None, None


def write_all(result, out_assets, out_docs, sync_docs=False):
    # Historical metrics must never overwrite C++ mainline documentation.
    sync_docs = False
    out_assets.mkdir(parents=True, exist_ok=True)
    out_docs.mkdir(parents=True, exist_ok=True)
    passed, date = validation()
    result['lastLocalValidation'] = {'passed': passed, 'date': date}
    (out_assets/'progress.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
    (out_assets/'badge-overall.svg').write_text(badge('PSP fidelity','unverified','#777'),encoding='utf-8')
    (out_assets/'badge-routines.svg').write_text(badge('Rust runtime functions',str(result['runtimeFunctions']),GREEN),encoding='utf-8')
    (out_assets/'progress.svg').write_text(treemap(result),encoding='utf-8')
    page = '<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>ReGTA CTW â€” Historical Rust inventory</title><style>body{background:#16191c;color:#e6e8ea;font:16px/1.6 system-ui;max-width:1100px;margin:40px auto;padding:0 20px}a{color:#ffc45e}img{max-width:100%;height:auto}table{border-collapse:collapse;width:100%}td,th{text-align:left;padding:10px;border-bottom:1px solid #555}p{color:#bfc3c7}</style><a href="../index.html">Back to ReGTA CTW</a><h1>Historical Rust inventory</h1><p>' + NOTE + '</p><img src="assets/progress.svg" alt="Native Rust function inventory">' + table(result) + '<p>Raw data: <a href="assets/progress.json">progress.json</a>. Local validation: ' + (f'{passed} tests passed, zero compiler warnings ({date}).' if passed else 'not available on this machine.') + '</p></html>\n'
    (out_docs/'progress.html').write_text(page,encoding='utf-8')
    if sync_docs:
        readme = ROOT/'README.md'
        text = readme.read_text(encoding='utf-8')
        block = '<!-- BEGIN STATUS SECTION -->\n## Status\n\n[![PSP fidelity](docs/assets/badge-overall.svg)](docs/progress.html) [![Rust runtime functions](docs/assets/badge-routines.svg)](docs/progress.html)\n\n[![Native Rust inventory](docs/assets/progress.svg)](docs/progress.html)\n\n' + NOTE + '\n\n[Driving and traffic validation](docs/DRIVING_TRAFFIC_REPAIR.md).\n<!-- END STATUS SECTION -->'
        text = re.sub(r'<!-- BEGIN STATUS SECTION -->.*?<!-- END STATUS SECTION -->',lambda _:block,text,flags=re.S)
        if passed:
            text = re.sub(r'local_tests-\d+_passed',f'local_tests-{passed}_passed',text)
            text = re.sub(r'\*\*\d+ tests / 0 failures\*\*',f'**{passed} tests / 0 failures**',text)
            text = re.sub(r'Local validation on \*\*.*?\*\*',f'Local validation on **{date}: {passed} passed, 0 failed, 0 ignored, 0 compiler warnings**',text)
        readme.write_text(text,encoding='utf-8')
        index = ROOT/'docs/index.html'
        text = index.read_text(encoding='utf-8')
        block = '<!-- BEGIN STATUS SECTION -->\n<section id="status" class="content wrap"><div class="section-top"><h2 data-en="Historical Rust inventory" data-fr="Ã‰tat du code Rust natif" data-es="Estado del cÃ³digo Rust nativo">Historical Rust inventory</h2><p>' + NOTE + '</p></div><div class="status-badges"><img src="assets/badge-overall.svg" alt="PSP fidelity unverified"> <img src="assets/badge-routines.svg" alt="Rust runtime function count"></div><div class="treemap-card"><a href="progress.html"><img src="assets/progress.svg" width="1000" height="560" alt="Native Rust inventory"></a></div><div class="progress-table-wrap">' + table(result) + '</div><p><a href="progress.html">Measurement details</a> Â· <a href="DRIVING_TRAFFIC_REPAIR.md">Driving and traffic validation</a></p></section>\n<!-- END STATUS SECTION -->'
        text = re.sub(r'<!-- BEGIN STATUS SECTION -->.*?<!-- END STATUS SECTION -->',lambda _:block,text,flags=re.S)
        if passed:
            text = re.sub(r'(<strong[^>]*>)\d+/\d+(</strong>)',rf'\g<1>{passed}/{passed}\2',text)
            text = re.sub(r'(<strong>)\d+(</strong>\s*<span\s+data-en="Unit tests passed")',rf'\g<1>{passed}\2',text)
        index.write_text(text,encoding='utf-8')
    print(f'Native Rust inventory: {result["runtimeFunctions"]} runtime functions, {result["testFunctions"]} test/helper functions, {result["modules"]} modules. PSP completion unmeasured.')


def main():
    parser = argparse.ArgumentParser(description='Measure native Rust inventory, never inferred PSP completion.')
    parser.add_argument('--source-dir',type=Path,default=SOURCE_DIR)
    parser.add_argument('--out-assets',type=Path,default=ROOT/'docs/legacy/assets')
    parser.add_argument('--out-docs',type=Path,default=ROOT/'docs/legacy')
    parser.add_argument('--no-sync',action='store_true')
    args = parser.parse_args()
    write_all(measure_rust(args.source_dir),args.out_assets,args.out_docs,not args.no_sync)


if __name__ == '__main__':
    main()
