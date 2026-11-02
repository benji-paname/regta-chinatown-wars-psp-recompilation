"""Validate the public landing page's local links, translations and media policy."""
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import urlsplit
ROOT=Path(__file__).resolve().parents[2]
class Page(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True);self.ids=set();self.links=[];self.translated=0;self.languages=set();self.images=[]
    def handle_starttag(self,tag,attrs):
        a=dict(attrs)
        if 'id' in a:self.ids.add(a['id'])
        if 'data-en' in a:
            assert all(a.get('data-'+lang) for lang in ['en','fr','es']), 'Missing translation: '+str(a)
            self.translated+=1
        if 'data-language' in a:self.languages.add(a['data-language'])
        if tag=='html':assert a.get('lang')=='en', 'English must be the default'
        for k in ['href','src']:
            if k in a:self.links.append(a[k])
        if tag=='img':self.images.append(a['src'])
        assert tag not in ['video','audio','iframe'], 'Unapproved embedded media'
def main():
    page=Page();page.feed((ROOT/'docs/index.html').read_text(encoding='utf-8-sig'))
    for link in page.links:
        p=urlsplit(link)
        if p.scheme or p.netloc:continue
        if p.path:assert (ROOT/'docs'/p.path).is_file(), 'Missing resource: '+link
        if not p.path and p.fragment:assert p.fragment in page.ids, 'Broken anchor: '+link
    assert page.languages=={'en','fr','es'}
    assert page.translated>=40
    assert all(Path(image).name in {'favicon.svg','logo-chinatown-wars.svg','badge-cpp.svg','cpp-progress.svg','cpp-status-map.svg','cpp-code-map.svg','cpp-codegen-progress.svg','badge-cpp-recomp.svg','badge-cpp-routines.svg'} for image in page.images), 'Unapproved image'
    print(f'Site: {page.translated} translated elements; EN/FR/ES; local links and approved media checked')
if __name__=='__main__':main()
