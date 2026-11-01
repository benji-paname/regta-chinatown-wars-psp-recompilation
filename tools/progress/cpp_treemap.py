"""Measure source-level PSP import mappings; never infer full-game completion."""
import hashlib
import html
import json
import math
import re
from collections import defaultdict
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
IMPORT_PATTERN=re.compile(r'\{\s*(0x[\dA-Fa-f]+)u?\s*,\s*"([^"]+)"\s*,\s*(0x[\dA-Fa-f]+)u?\s*,\s*"([^"]+)"\s*\}')


def parse_imports(text):
    section=text.split('const ImportEntry g_import_list[]',1)[1].split('};',1)[0]
    records=[{'stub':int(m[1],16),'library':m[2],'nid':int(m[3],16),'name':m[4]} for m in IMPORT_PATTERN.finditer(section)]
    if not records:raise ValueError('No import records parsed; leave published progress unchanged')
    return records


def providers(source):
    code=re.sub(r'/\*.*?\*/|//[^\n]*','',source,flags=re.S)
    found=set(re.findall(r'\bHLE\s*\(\s*(\w+)\s*,\s*(\w+)\s*\)',code))
    found.update(re.findall(r'install_hle_by_name\s*\(\s*"([^"]+)"\s*,\s*"([^"]+)"',code))
    return found


def category(library):
    if library.startswith('sceNet') or library=='sceWlanDrv':return 'Network'
    if library in {'sceSasCore','sceAtrac3plus','sceAudio'}:return 'Audio'
    if library in {'sceDisplay','sceGe_user'}:return 'Graphics & display'
    if library in {'IoFileMgrForUser','StdioForUser','sceUmdUser','scePspNpDrm_user'}:return 'Files & media'
    if library in {'ThreadManForUser','LoadExecForUser','ModuleMgrForUser','SysMemUserForUser','InterruptManager','Kernel_Library'}:return 'Kernel & threads'
    return 'Input & system'


def measure(records, definitions):
    # Match rt_init's last-record-per-stub map; four overlapping records are not
    # extra active services. Library+name is the actual HLE registry key.
    selected={r['stub']:r for r in records}
    if not selected:raise ValueError('Empty import inventory')
    counts=defaultdict(lambda:{'mapped':0,'total':0})
    for r in selected.values():
        c=counts[r['library']];c['total']+=1;c['mapped']+=((r['library'],r['name']) in definitions)
    groups=defaultdict(list)
    for lib,c in sorted(counts.items()):groups[category(lib)].append({'library':lib,**c})
    mapped=sum(c['mapped'] for c in counts.values());total=len(selected)
    return {'raw_records':len(records),'selected_imports':total,'mapped':mapped,'missing':total-mapped,
            'percent':round(mapped/total*100,2),'groups':dict(sorted(groups.items()))}


def split_rect(items,x,y,w,h):
    if len(items)==1:return [(items[0],x,y,w,h)]
    total=sum(a['total'] for a in items);acc=0;cut=1
    for i,item in enumerate(items[:-1],1):
        acc+=item['total'];cut=i
        if acc>=total/2:break
    first=items[:cut];second=items[cut:];ratio=sum(a['total'] for a in first)/total
    if w>=h:return split_rect(first,x,y,w*ratio,h)+split_rect(second,x+w*ratio,y,w*(1-ratio),h)
    return split_rect(first,x,y,w,h*ratio)+split_rect(second,x,y+h*ratio,w,h*(1-ratio))


def treemap(data):
    elements=['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1100 690" role="img" aria-labelledby="title desc">',
              '<title id="title">PSP import source mappings — '+f'{data["percent"]:.2f}%</title>',
              '<desc id="desc">Green means an exact library and name provider exists in the C++ source. Grey means absent. This is not a percentage of gameplay completed or proof of correct behavior.</desc>',
              '<rect width="1100" height="690" rx="10" fill="#15191f"/>',
              '<text x="22" y="35" fill="#f2c35b" font-size="20" font-family="Arial,sans-serif" font-weight="bold">PSP import source mappings</text>',
              f'<text x="1078" y="35" text-anchor="end" fill="#edf6ec" font-size="22" font-family="Arial,sans-serif">{data["percent"]:.2f}% · {data["mapped"]} / {data["selected_imports"]}</text>']
    groups=[{'name':name,'total':sum(l['total'] for l in libs),'libraries':libs} for name,libs in data['groups'].items()]
    for group,x,y,w,h in split_rect(sorted(groups,key=lambda a:-a['total']),18,56,1064,566):
        mapped=sum(l['mapped'] for l in group['libraries']);total=group['total'];label=f'{group["name"]} · {mapped/total*100:.1f}%'
        elements+=[f'<rect x="{x:.2f}" y="{y:.2f}" width="{w:.2f}" height="{h:.2f}" fill="#1b2027" stroke="#818b96" stroke-width="1"/>',f'<text x="{x+7:.2f}" y="{y+18:.2f}" fill="#f2f3f4" font-family="Arial,sans-serif" font-size="12" font-weight="bold">{html.escape(label)}</text>']
        for lib,lx,ly,lw,lh in split_rect(sorted(group['libraries'],key=lambda a:-a['total']),x+4,y+26,w-8,h-30):
            name=lib['library'];title=f'{name}: {lib["mapped"]}/{lib["total"]} source mappings'
            elements.append(f'<rect x="{lx:.2f}" y="{ly:.2f}" width="{lw:.2f}" height="{lh:.2f}" fill="#252b33" stroke="#15191f"><title>{html.escape(title)}</title></rect>')
            label_height=17 if lw>48 and lh>34 else 0
            if label_height:
                short=name if len(name)*5.6<lw-8 else name[:max(2,int((lw-14)/5.6))]+'…'
                elements.append(f'<text x="{lx+4:.2f}" y="{ly+12:.2f}" fill="#cbd2da" font-family="Arial,sans-serif" font-size="10">{html.escape(short)}</text>')
            inner_w=lw-5;inner_h=lh-label_height-5;n=lib['total'];columns=max(1,math.ceil(math.sqrt(n*inner_w/inner_h)));rows=math.ceil(n/columns)
            cw=inner_w/columns;ch=inner_h/rows
            for i in range(n):
                state='mapped' if i<lib['mapped'] else 'missing';color='#45ad59' if state=='mapped' else '#555c66'
                elements.append(f'<rect class="{state}" x="{lx+2+(i%columns)*cw:.2f}" y="{ly+label_height+2+(i//columns)*ch:.2f}" width="{max(.2,cw-1):.2f}" height="{max(.2,ch-1):.2f}" fill="{color}"><title>{html.escape(title)} · {state}</title></rect>')
    elements+=['<rect x="22" y="642" width="13" height="13" fill="#45ad59"/><text x="43" y="653" fill="#dbe2e8" font-family="Arial,sans-serif" font-size="13">Source provider found</text>',
               '<rect x="227" y="642" width="13" height="13" fill="#555c66"/><text x="248" y="653" fill="#dbe2e8" font-family="Arial,sans-serif" font-size="13">Missing source provider</text>',
               '<text x="22" y="675" fill="#a4afba" font-family="Arial,sans-serif" font-size="12">Each tile is one selected PSP import. A mapping may be partial or a stub; green does not certify gameplay fidelity.</text></svg>']
    return '\n'.join(elements)+'\n'


def generate(root=ROOT):
    table=root/'work/cpp-recomp-feasibility/build/gen/recomp_table.cpp'
    records=parse_imports(table.read_text());definitions=set();sources=[]
    for p in sorted((root/'runtime').rglob('*.cpp')):
        raw=p.read_bytes();definitions.update(providers(raw.decode('utf-8-sig')))
        sources.append({'file':p.relative_to(root).as_posix(),'sha256':hashlib.sha256(raw).hexdigest()})
    data=measure(records,definitions)
    data['method']='Selected PSP imports with exact (library, name) source providers; not game completion or implementation correctness'
    data['evidence']={'private_import_table_sha256':hashlib.sha256(table.read_bytes()).hexdigest(),'runtime_sources':sources}
    assets=root/'docs/assets'
    (assets/'cpp-progress.svg').write_text(treemap(data),encoding='utf-8')
    (assets/'cpp-progress.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
    badge=f'<svg xmlns="http://www.w3.org/2000/svg" width="292" height="24" role="img" aria-label="PSP import source mappings {data["percent"]:.2f}%"><rect width="292" height="24" rx="4" fill="#24492c"/><text x="146" y="16" text-anchor="middle" fill="#e7f6e7" font-size="11" font-family="Verdana,sans-serif">PSP import mappings · {data["percent"]:.2f}% ({data["mapped"]}/{data["selected_imports"]})</text></svg>\n'
    (assets/'badge-cpp-imports.svg').write_text(badge,encoding='utf-8')
    readme=root/'README.md'
    metric=f"<!-- CPP_IMPORT_PROGRESS_START -->\n![PSP import mappings](docs/assets/badge-cpp-imports.svg)\n\n**{data['percent']:.2f}% of selected PSP imports have exact source providers ({data['mapped']}/{data['selected_imports']}).** Green blocks show source mappings; grey blocks show missing providers. Mappings can be incomplete or stubs: this is not a percentage of the game finished or a gameplay-fidelity certificate.\n\n[![PSP import source mappings](docs/assets/cpp-progress.svg)](docs/CPP_PROGRESS.md)\n<!-- CPP_IMPORT_PROGRESS_END -->"
    if readme.exists():
        text=readme.read_text(encoding='utf-8-sig')
        text=re.sub(r'<!-- CPP_IMPORT_PROGRESS_START -->.*?<!-- CPP_IMPORT_PROGRESS_END -->',lambda _:metric,text,flags=re.S)
        readme.write_text(text,encoding='utf-8')
    site=root/'docs/index.html'
    if site.exists():
        text=site.read_text(encoding='utf-8-sig')
        text=re.sub(r'(<strong data-import-percent>)[^<]+',lambda m:m[1]+f"{data['percent']:.2f}%",text)
        text=re.sub(r'(<code data-import-count>)[^<]+',lambda m:m[1]+f"{data['mapped']} / {data['selected_imports']}",text)
        site.write_text(text,encoding='utf-8')
    rows=['| PSP library | Source mappings | Percent |','| --- | ---: | ---: |']
    for libs in data['groups'].values():
        for lib in libs:rows.append(f"| {lib['library']} | {lib['mapped']}/{lib['total']} | {lib['mapped']/lib['total']*100:.2f}% |")
    report=f"# C++ progress: PSP import source mappings\n\n**{data['percent']:.2f}% — {data['mapped']} of {data['selected_imports']} selected PSP imports.**\n\n![PSP import source mappings](assets/cpp-progress.svg)\n\nThe percentage measures exact `(library, name)` providers in the current canonical C++ source. Each tile is one selected import: green when a provider declaration exists, grey when absent. Providers may be partial or stubs. This metric does not prove correct behavior, multiplayer availability or completion of any percentage of the game.\n\nThe private European import inventory contains {data['raw_records']} records; the same last-record-per-stub selection used by the runtime yields {data['selected_imports']} active entries. No original import table, NID list, generated code or game data is published here. Groups and aggregate counts are public research metadata. Runtime sources and private input hashes are recorded in [cpp-progress.json](assets/cpp-progress.json).\n\nIndependent evidence: 29 runtime components compiled, 250 generated objects reused, 35 kernel checks passed; Benjamin reports two missions completed. These measurements are not combined into a game-completion score. See [STATUS.md](STATUS.md).\n\n"+'\n'.join(rows)+"\n\nRegenerate in the prepared workspace: `python tools/progress/cpp_status.py`. The generator parses current source registrations and private imports; it never reads Rust metrics. See tools/progress/cpp_treemap.py for the method.\n"
    (root/'docs/CPP_PROGRESS.md').write_text(report,encoding='utf-8')
    print(f'PSP import source mappings: {data["mapped"]}/{data["selected_imports"]} = {data["percent"]:.2f}%')
    return data

if __name__=='__main__':generate()
