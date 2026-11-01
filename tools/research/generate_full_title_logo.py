"""Compose the full-title primary logo using private fonts; preserve existing CTW variants."""
from pathlib import Path
import argparse, hashlib
from fontTools.pens.recordingPen import RecordingPen, replayRecording
from fontTools.ttLib import TTFont
from generate_brand_logos import outline, paint
from fontTools.pens.boundsPen import BoundsPen
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.transformPen import TransformPen
from fontTools.svgLib.path import parse_path
from xml.sax.saxutils import escape
import xml.etree.ElementTree as ET

ROOT=Path(__file__).resolve().parents[2]

class CoverGlyph:
    # The supplied historical font is the engraved version. Keep its real outer
    # contours and letter counters; remove only its decorative inline highlights
    # to obtain the solid red lettering seen on the cover. The font is untouched.
    def __init__(self, glyph, character):
        self.width=glyph.width
        recording=RecordingPen();glyph.draw(recording)
        contours=[];current=[]
        for command in recording.value:
            current.append(command)
            if command[0] in {'closePath','endPath'}:contours.append(current);current=[]
        keep={0,*{'A':{3},'O':{2},'R':{3}}.get(character,set())}
        self.commands=[command for i,contour in enumerate(contours) if i in keep for command in contour]
    def draw(self, pen):replayRecording(self.commands,pen)



def title_path(font, text, cx, top, width, height, bookends=False):
    glyphs=font.getGlyphSet();cmap=font.getBestCmap();placed=[];advance=0
    for i,ch in enumerate(text):
        glyph=CoverGlyph(glyphs[cmap[ord(ch)]],ch)
        scale=1.22 if bookends and i in {0,len(text)-1} else 1
        placed.append((glyph,scale,advance));advance+=glyph.width*scale-15
    bounds=BoundsPen(glyphs)
    for glyph,k,x in placed:glyph.draw(TransformPen(bounds,(k,0,0,k,x,0)))
    x0,y0,x1,y1=bounds.bounds
    scale=min(width/(x1-x0),height/(y1-y0));left=cx-(x1-x0)*scale/2
    pen=SVGPathPen(glyphs)
    for glyph,k,x in placed:glyph.draw(TransformPen(pen,(scale*k,0,0,-scale*k,left+(x-x0)*scale,top+y1*scale)))
    return pen.getCommands()


def ink(path):
    return (f'<path d="{path}" fill="#080808" stroke="#080808" stroke-width="15" stroke-linejoin="round" transform="translate(3 4)"/>'
            f'<path d="{path}" fill="#ffda22" stroke="#ffda22" stroke-width="6" stroke-linejoin="round"/>'
            f'<path d="{path}" fill="#d90000"/>')


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--bruce-mikita',type=Path,required=True)
    ap.add_argument('--pricedown',type=Path,required=True)
    ap.add_argument('--regta-source',type=Path,default=ROOT/'docs/assets/logo.svg')
    ap.add_argument('--output',type=Path,default=ROOT/'docs/assets')
    args=ap.parse_args()
    assert hashlib.sha256(args.bruce_mikita.read_bytes()).hexdigest()=='6c6700ee98153ca00d58284b294f196aed291141ba1dad4915efb559cbf602f2', 'This contour selection requires the documented BRUCM___.TTF copy'
    font=TTFont(args.bruce_mikita)
    subtitle_font=TTFont(args.pricedown)
    source=ET.parse(args.regta_source).getroot()
    regta_paths=[e for e in source if e.tag.endswith('path')][:2]
    assert len(regta_paths)==2 and all(e.get('d') for e in regta_paths)
    # Reuse exactly the approved ReGTA contours, including its lowercase e.
    regta=''.join(ET.tostring(e,encoding='unicode').replace('ns0:','').replace(':ns0','') for e in regta_paths)
    china=title_path(font,'CHINATOWN',450,263,770,133,True)
    wars=title_path(font,'WARS',450,401,320,82)
    subtitle=paint(outline(subtitle_font,'PSP RECOMPILATION',450,525,790,82))
    svg=f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 900 625" width="900" height="625" role="img">
<title>ReGTA Chinatown Wars — PSP Recompilation</title><desc>Independent project wordmark. Approved ReGTA outlines and Bruce Mikita full-title lettering, red with yellow and black outlines. No font software embedded.</desc>
{regta}{ink(china)}{ink(wars)}
{subtitle}
</svg>'''
    args.output.mkdir(parents=True,exist_ok=True)
    for suffix in ['', '-transparent']:(args.output/f'logo-chinatown-wars{suffix}.svg').write_text(svg,encoding='utf-8')
    black=svg.replace('<title>','<rect width="900" height="625" fill="#000000"/><title>',1)
    (args.output/'logo-chinatown-wars-black.svg').write_text(black,encoding='utf-8')
    print('Full-title main logo generated; existing CTW logos unchanged.')

if __name__=='__main__':main()
