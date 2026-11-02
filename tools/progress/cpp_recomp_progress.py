"""Original PSP recompilation coverage, weighted by original routine byte size.

Private analysis/generated code/objects are inputs only. No game payload is exported.
"""
import hashlib
import html
import json
import re
import struct
from decimal import Decimal, ROUND_DOWN
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
DEFINITION=re.compile(r'\bvoid\s+((?:F|O\d{3})_[0-9A-F]{8}(?:_orig)?)\s*\(CPUState\s*\*\s*c\)\s*\{')


def digest(path):
    with path.open('rb') as stream:return hashlib.file_digest(stream,'sha256').hexdigest()


def definitions(text):
    # Slice actual generated definitions, ignoring forward declarations. Generated
    # compiler input is pinned by its SHA256; never count a prototype as a port.
    matches=list(DEFINITION.finditer(text));out={}
    for i,m in enumerate(matches):
        end=matches[i+1].start() if i+1<len(matches) else len(text)
        body=text[m.end():end]
        if m[1] in out:raise ValueError('Duplicate generated definition: '+m[1])
        out[m[1]]={'unhandled':bool(re.search(r'\brt_unhandled\s*\(',body))}
    return out


def classify(name):
    if name=='main':return 'Main executable'
    if re.match(r'^(bik|hes|jao|ken|kor|mel|mob|xin|zho|rnd)_',name) or name in {'endgame','scriptinit'}:return 'Mission overlays'
    if any(word in name for word in ['hotwir','sinking','tattoo','scratch','crack','bomb','molotov','rifle','disposal','planting','sabotage','winching','vancutting','liftgate','lock','dumpster','bodies','crane','drugvan','policevans']):return 'Minigame overlays'
    if any(word in name for word in ['race','taxi','firefighter','paramedic','vigilante','rampage','timetrial','warehouseoj','pcjplay','tankfield','turret','packagerun','delivery','shootingrange']):return 'Activities & races'
    if name.endswith('app') or name in {'desktopex','safehousesalehandler','tattooparlour','scratchcards'}:return 'UI & other apps'
    return 'Other overlays'


def summarise(routines):
    if not routines:raise ValueError('Empty original-code inventory')
    total=sum(r['bytes'] for r in routines);done=sum(r['bytes'] for r in routines if r['recompiled'])
    if total<=0 or any(r['bytes']<=0 for r in routines):raise ValueError('Invalid original routine size')
    raw=Decimal(done)*100/Decimal(total)
    # Never round unresolved work up to 100.00%.
    display=format(raw.quantize(Decimal('.01'),rounding=ROUND_DOWN),'.2f')
    return {'bytes':total,'recompiled_bytes':done,'remaining_bytes':total-done,'routines':len(routines),
            'recompiled_routines':sum(r['recompiled'] for r in routines),'percent':float(raw),'display_percent':display}


def measure(root=ROOT):
    analysis_path=root/'work/cpp-recomp-feasibility/data/functions.json'
    recipe_path=root/'work/cpp-mainline/build-config.json'
    data=json.loads(analysis_path.read_text(encoding='utf-8'));recipe=json.loads(recipe_path.read_text(encoding='utf-8'))
    inventory={};units=[]
    for u in recipe['units']:
        if not u['generated']:continue
        source=root/u['source'];obj=root/u['object'];valid=False
        if source.is_file() and obj.is_file():
            with obj.open('rb') as stream:header=stream.read(20)
            amd64=len(header)==20 and struct.unpack_from('<H',header)[0]==0x8664 and struct.unpack_from('<H',header,2)[0]>0
            valid=amd64 and digest(source)==u['source_sha256'] and digest(obj)==u['object_sha256']
        units.append({'name':u['name'],'verified':valid})
        if source.is_file():
            for name,state in definitions(source.read_text(encoding='utf-8-sig')).items():
                if name in inventory:raise ValueError('Generated symbol repeated in different units')
                inventory[name]={**state,'compiled':valid}
    modules=[{'id':0,'name':'main','functions':data['functions'],'stats':data['stats']}]+data['overlays']
    routines=[];summaries=[];blocked=[]
    for module in modules:
        if 'same_code_as' in module:raise ValueError('Shared overlay code needs explicit deduplication rules')
        records=[];previous_end=None
        for f in sorted(module['functions'],key=lambda f:f['start']):
            if previous_end is not None and f['start']<previous_end:raise ValueError('Overlapping function inventory')
            previous_end=f['end']
            name=(f'O{module["id"]:03}_' if module['id'] else 'F_')+f'{f["start"]:08X}'
            state=inventory.get(name);original=inventory.get(name+'_orig',state)
            good=bool(state and original and state['compiled'] and original['compiled'] and not state['unhandled'] and not original['unhandled'])
            reason='translated and compiled' if good else ('unsupported operation' if original and original['unhandled'] else 'missing or unverified compiled body')
            record={'bytes':f['end']-f['start'],'recompiled':good,'module':module['name']}
            records.append(record)
            if not good:blocked.append({'module':module['name'],'bytes':record['bytes'],'reason':reason})
        summary=summarise(records)
        if summary['bytes']!=module['stats']['code_bytes']:raise ValueError('Original byte denominator has gaps; review analysis')
        summaries.append({'name':module['name'],'category':classify(module['name']),**summary})
        routines.extend(records)
    result=summarise(routines)
    result.update(metric='Original PSP code recompiled, weighted by original routine size',build='PSP Europe ULES-01347 v1.01',
                  scope='Indexed executable .text and all 166 analyzed code overlays; original analysis boundaries may include embedded data',
                  modules=summaries,blocked=blocked,overlay_count=len(data['overlays']),generated_units=len(units),verified_units=sum(u['verified'] for u in units),
                  evidence={'analysis_sha256':digest(analysis_path),'private_build_recipe_sha256':digest(recipe_path)},
                  limits=['Static translation and object compilation, not gameplay correctness or campaign completion',
                          'PSP service HLE coverage is a separate metric',
                          'One known unsupported candidate is suspected embedded data; it remains excluded pending formal code/data classification',
                          'Dynamic callback targets and analysis completeness are not proven by this percentage'],
                  rounding='Truncated to two decimal places; 100.00% only when no indexed bytes remain excluded')
    return result,routines

def rectangles(items,x,y,w,h):
    if len(items)==1:return [(items[0],x,y,w,h)]
    total=sum(item['weight'] for item in items);target=total/2;acc=0;cut=1
    for i,item in enumerate(items[:-1],1):
        acc+=item['weight'];cut=i
        if acc>=target:break
    first,second=items[:cut],items[cut:];ratio=sum(item['weight'] for item in first)/total
    if w>=h:return rectangles(first,x,y,w*ratio,h)+rectangles(second,x+w*ratio,y,w*(1-ratio),h)
    return rectangles(first,x,y,w,h*ratio)+rectangles(second,x,y+h*ratio,w,h*(1-ratio))


def treemap(result,routines):
    grouped={}
    for module in result['modules']:grouped.setdefault(module['category'],[]).append(module)
    groups=[{'name':name,'weight':sum(m['bytes'] for m in modules),'modules':modules} for name,modules in grouped.items()]
    by_module={}
    for routine in routines:by_module.setdefault(routine['module'],[]).append(routine)
    out=['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1100 730" role="img" aria-labelledby="title desc">',
         '<title id="title">Original PSP code recompiled: '+result['display_percent']+'%, weighted by original code size</title>',
         '<desc id="desc">Main executable and 166 European PSP overlays. Green: original routine has a matching compiled C++ body without a known unsupported operation. Grey: not yet counted. Static coverage only; runtime fidelity and PSP HLE services are separate.</desc>',
         '<style>.recompiled{fill:#3fa34d}.remaining{fill:#555b64}.recompiled,.remaining{stroke:#28332b;stroke-width:.18}</style><rect width="1100" height="730" fill="#16191c"/>',
         f'<text x="14" y="27" fill="#e6e8ea" font-size="16" font-family="Verdana,sans-serif">Original PSP code recompiled: {result["display_percent"]}%</text>',
         f'<text x="14" y="50" fill="#bac5cf" font-size="12" font-family="Verdana,sans-serif">{result["recompiled_routines"]:,} / {result["routines"]:,} routines · weighted by original PSP bytes · ELF + 166 overlays</text>']
    for group,x,y,w,h in rectangles(sorted(groups,key=lambda g:-g['weight']),12,65,1076,596):
        summary=summarise([r for m in group['modules'] for r in by_module[m['name']]])
        out.append(f'<rect x="{x:.2f}" y="{y:.2f}" width="{w:.2f}" height="{h:.2f}" fill="#24282d" stroke="#87909a"/>')
        out.append(f'<text x="{x+4:.2f}" y="{y+16:.2f}" fill="#fff" font-size="11" font-family="Verdana,sans-serif">{html.escape(group["name"])} {summary["display_percent"]}%</text>')
        modules=[dict(m,weight=m['bytes']) for m in group['modules']]
        for module,mx,my,mw,mh in rectangles(sorted(modules,key=lambda m:-m['weight']),x+2,y+22,w-4,h-24):
            out.append(f'<rect x="{mx:.2f}" y="{my:.2f}" width="{mw:.2f}" height="{mh:.2f}" fill="#343a42" stroke="#16191c"><title>{html.escape(module["name"])}: {module["recompiled_routines"]}/{module["routines"]} routines</title></rect>')
            header=14 if mw>45 and mh>28 else 0
            if header:
                name=module['name'] if module['name']!='main' else 'PSP main executable'
                capacity=max(2,int((mw-8)/5.2));name=name if len(name)<=capacity else name[:capacity-1]+'…'
                out.append(f'<text x="{mx+3:.2f}" y="{my+11:.2f}" fill="#e2e5e9" font-size="9" font-family="Verdana,sans-serif">{html.escape(name)}</text>')
            cells=[dict(r,weight=r['bytes']) for r in by_module[module['name']]]
            for routine,rx,ry,rw,rh in rectangles(sorted(cells,key=lambda r:-r['weight']),mx+.8,my+header+.8,mw-1.6,mh-header-1.6):
                state='recompiled' if routine['recompiled'] else 'remaining'
                out.append(f'<rect class="{state}" x="{rx:.2f}" y="{ry:.2f}" width="{rw:.3f}" height="{rh:.3f}"/>')
    out+=['<rect x="14" y="678" width="12" height="12" fill="#3fa34d"/><text x="34" y="689" fill="#d3dde5" font-size="12" font-family="Verdana,sans-serif">Recompiled + compiled</text>',
          '<rect x="240" y="678" width="12" height="12" fill="#555b64"/><text x="260" y="689" fill="#d3dde5" font-size="12" font-family="Verdana,sans-serif">Not yet counted</text>',
          f'<text x="14" y="713" fill="#bac5cf" font-size="12" font-family="Verdana,sans-serif">{result["routines"]-result["recompiled_routines"]} candidate excluded · static recompilation coverage, not a gameplay completion score.</text></svg>']
    return '\n'.join(out)+'\n'


def badge(label,value,width):
    return f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="24" role="img" aria-label="{html.escape(label+": "+value)}"><rect width="{width}" height="24" rx="3" fill="#2f6239"/><text x="{width/2}" y="16" text-anchor="middle" fill="#fff" font-size="11" font-family="Verdana,sans-serif">{html.escape(label+": "+value)}</text></svg>\n'


def generate(root=ROOT):
    result,routines=measure(root);assets=root/'docs/assets'
    (assets/'cpp-progress.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
    (assets/'cpp-progress.svg').write_text(treemap(result,routines),encoding='utf-8')
    (assets/'badge-cpp-recomp.svg').write_text(badge('Original PSP code recompiled',result['display_percent']+'%',318),encoding='utf-8')
    (assets/'badge-cpp-routines.svg').write_text(badge('PSP routines',f"{result['recompiled_routines']} / {result['routines']}",236),encoding='utf-8')
    metric=f"<!-- CPP_RECOMP_PROGRESS_START -->\n![Original PSP code recompiled](docs/assets/badge-cpp-recomp.svg) ![PSP routines](docs/assets/badge-cpp-routines.svg)\n\n**Original PSP code recompiled: {result['display_percent']}% — {result['recompiled_routines']:,} / {result['routines']:,} indexed routines, weighted by original code size.** Green routines have native C++ bodies in verified compiled objects, without a known unsupported operation. This measures static recompilation of the indexed PSP executable and 166 overlays. Runtime services, correctness and campaign validation remain separate.\n\n[![Original PSP code recompilation treemap](docs/assets/cpp-progress.svg)](docs/CPP_PROGRESS.md)\n\n[PSP service bindings: a separate diagnostic](docs/CPP_IMPORTS.md).\n<!-- CPP_RECOMP_PROGRESS_END -->"
    readme=root/'README.md';text=readme.read_text(encoding='utf-8')
    text=re.sub(r'<!-- CPP_RECOMP_PROGRESS_START -->.*?<!-- CPP_RECOMP_PROGRESS_END -->',lambda _:metric,text,flags=re.S)
    readme.write_text(text,encoding='utf-8')
    site=root/'docs/index.html';text=site.read_text(encoding='utf-8')
    text=re.sub(r'(<strong data-recomp-percent>)[^<]+',lambda m:m[1]+result['display_percent']+'%',text)
    text=re.sub(r'(<code data-recomp-count>)[^<]+',lambda m:m[1]+f"{result['recompiled_routines']:,} / {result['routines']:,}",text)
    site.write_text(text,encoding='utf-8')
    rows=['| PSP module | Original bytes | Recompiled bytes | Routines recompiled |','| --- | ---: | ---: | ---: |']
    for m in result['modules']:rows.append(f"| {m['name']} | {m['bytes']:,} | {m['recompiled_bytes']:,} | {m['recompiled_routines']:,}/{m['routines']:,} |")
    report=f"""# Original PSP code recompilation progress

**{result['display_percent']}% displayed — {result['recompiled_routines']:,}/{result['routines']:,} indexed PSP routines.**

![Original PSP code recompiled](assets/cpp-progress.svg)

## Definition requested by Benjamin

100% means all code in the PSP reference inventory has been translated to native C++ and compiled with no known unsupported routine in that inventory. The calculation follows CTW-Native's original-size weighting principle, adapted to the European PSP executable and all 166 overlays:

`100 × sum(original byte size of counted routines) / sum(original byte size of all indexed routines)`

Current numerator **{result['recompiled_bytes']:,} original PSP bytes**; denominator **{result['bytes']:,} original PSP bytes**. This is **{result['percent']:.8f}% before display truncation**. The display truncates down to two decimals, so unresolved work never rounds up to 100.00%.

A routine counts only when its generated C++ body exists, its source SHA256 matches the compiled recipe, its Windows AMD64 COFF object exists and matches its recorded SHA256, and neither its original body nor hook wrapper contains an `rt_unhandled` operation. The hook's `_orig` body and wrapper count as one original routine. Runtime helpers, HLE declarations, tests and Rust code do not contribute to the numerator. {result['verified_units']}/{result['generated_units']} generated units passed the source/object checks in this measurement; no new compilation or game run was performed.

The original-size reference is `end - start` from the existing European function analysis, not C++ line count or x64 object size. These intervals exactly partition the executable/overlay code-size inventory. Analysis boundaries can include embedded data; this is coverage of that indexed scope, not a proof that every boundary is a real routine or every dynamic callback is known. The reference has {result['routines']:,} routine candidates; Android's 25,988-routine denominator is not used for PSP.

## Remaining excluded candidate and practical limits

One four-byte candidate in `mapapp` still contains a generated unsupported-operation trap. Earlier static analysis strongly suggests embedded data misidentified as code. It remains grey and excluded until the code/data classification is formally resolved; this task does not edit the original data, generated code or runtime to obtain 100%.

This high percentage reflects **automatic static translation**, which can translate most indexed code before all PSP runtime services work. It is not a percentage of gameplay validated, faithful behavior, performance, campaign completion or public-release readiness. Only two missions have been owner-tested so far. See [STATUS.md](STATUS.md).

The older **80.56% (203/252)** figure measures [PSP service source bindings](CPP_IMPORTS.md). It is a separate diagnostic and no longer the primary recompilation-progress figure.

## Method and provenance

The arithmetic uses the same original-routine-size weighting principle documented in [CTW-Native's progress script](https://github.com/official-kryo-to/ctw-native/blob/108320b137c3ca748d697838f86c8919c12b3f2c/scripts/progress.py). CTW-Native identifies manually ported routines by source names. This PSP project identifies compiled generated bodies by their native symbols and pinned build evidence. No Android counts, routine sizes, addresses or completion claims are transplanted.

Regenerate locally: `python tools/progress/cpp_status.py`. Implementation: tools/progress/cpp_recomp_progress.py. The original function inventory, source translations and objects remain private. Only aggregate module counts/byte totals, hashes and a generated visualization are published in [cpp-progress.json](assets/cpp-progress.json).

"""+'\n'.join(rows)+'\n'
    (root/'docs/CPP_PROGRESS.md').write_text(report,encoding='utf-8')
    print(f"Original PSP code recompiled: {result['display_percent']}%, {result['recompiled_routines']}/{result['routines']} routines by original bytes")
    return result

if __name__=='__main__':generate()
