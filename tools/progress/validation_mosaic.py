"""Evidence-backed scenario mosaic, independent of automatic code generation."""
import html
import textwrap

COLORS={'observed':'#3fa34d','tested':'#3fa34d','partial':'#bd8738','pending':'#a83d43','issue':'#cf4149'}
LABELS={'observed':'OBSERVED','tested':'TESTED','partial':'PARTIAL','pending':'OPEN','issue':'ISSUE'}


def validate(groups):
    seen=set()
    if len(groups)!=9:raise ValueError('Expected nine scenario groups')
    for group in groups:
        if not group.get('name') or len(group.get('checks',[]))!=6:raise ValueError('Each group needs six explicit checks')
        for check in group['checks']:
            if check.get('id') in seen or not check.get('id'):raise ValueError('Missing or duplicate scenario id')
            seen.add(check['id'])
            if check.get('state') not in COLORS or not check.get('label') or not check.get('evidence'):raise ValueError('Scenario state, label and evidence required')
            if check['state'] in {'observed','tested'} and not check.get('source'):raise ValueError('Green scenarios require an evidence source')
    return groups


def render(groups):
    validate(groups)
    out=['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1100 850" role="img" aria-labelledby="title desc">',
         '<title id="title">Playable port &#8212; validation mosaic</title>',
         '<desc id="desc">Green: observed or tested within the stated scope. Amber: partial. Red: work or validation still open, including known issues. Cells are checklist items, not PSP routines or weighted work percentages. This expanding checklist is not the complete original-game scope.</desc>',
         '<rect width="1100" height="850" rx="6" fill="#14191e"/>',
         '<text x="18" y="32" fill="#f2c35b" font-size="23" font-weight="bold" font-family="Arial,sans-serif">Playable port &#8212; validation mosaic</text>',
         '<text x="18" y="56" fill="#d4dce3" font-size="13" font-family="Arial,sans-serif">Five story missions &amp; taxi activity owner-tested. Campaign, PSP services, fidelity and performance validation continue.</text>',
         '<text x="18" y="77" fill="#aebbc7" font-size="12" font-family="Arial,sans-serif">Each tile is a specific scenario or check &#8212; automatic code generation does not turn it green.</text>']
    for i,group in enumerate(groups):
        x=16+(i%3)*362;y=96+(i//3)*222
        out.extend([f'<rect x="{x}" y="{y}" width="352" height="212" fill="#202932" stroke="#596572"/>',
                    f'<text x="{x+8}" y="{y+22}" fill="#eef1f4" font-size="14" font-weight="bold" font-family="Arial,sans-serif">{html.escape(group["name"])}</text>'])
        for j,check in enumerate(group['checks']):
            cx=x+6+(j%2)*171;cy=y+33+(j//2)*58
            out.append(f'<rect class="{check["state"]}" x="{cx}" y="{cy}" width="169" height="56" fill="{COLORS[check["state"]]}" stroke="#14191e"><title>{html.escape(check["label"]+": "+check["evidence"])}</title></rect>')
            out.append(f'<text x="{cx+7}" y="{cy+12}" fill="#eff5f1" font-size="8" font-weight="bold" font-family="Arial,sans-serif">{LABELS[check["state"]]}</text>')
            for k,line in enumerate(textwrap.wrap(check['label'],width=23)):
                out.append(f'<text x="{cx+7}" y="{cy+29+k*13}" fill="#fff" font-size="11" font-family="Arial,sans-serif">{html.escape(line)}</text>')
    out.extend(['<rect x="18" y="782" width="12" height="12" fill="#3fa34d"/><text x="38" y="793" fill="#d4dce3" font-size="12" font-family="Arial,sans-serif">Observed / bounded test</text>',
                '<rect x="280" y="782" width="12" height="12" fill="#bd8738"/><text x="300" y="793" fill="#d4dce3" font-size="12" font-family="Arial,sans-serif">Partial</text>',
                '<rect x="420" y="782" width="12" height="12" fill="#a83d43"/><text x="440" y="793" fill="#d4dce3" font-size="12" font-family="Arial,sans-serif">Not finished / validation open</text>',
                '<text x="18" y="819" fill="#b6c1cb" font-size="12" font-family="Arial,sans-serif">Green applies only to each named check. Red does not assert a missing implementation; it also includes untested behavior.</text>',
                '<text x="18" y="839" fill="#b6c1cb" font-size="12" font-family="Arial,sans-serif">Expanding validation checklist, not a complete feature inventory. Tile areas do not measure remaining effort or overall completion.</text></svg>'])
    return '\n'.join(out)+'\n'
