"""Update only the local game executable; retain the previous binary and all user data."""
import argparse
import hashlib
import json
import os
import shutil
import struct
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PLAYTEST = ROOT / 'outputs/ReGTA-CTW-Playtest'


def digest(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def validate_pe(path):
    with path.open('rb') as stream:
        if stream.read(2) != b'MZ':
            raise ValueError('Source is not a Windows executable')
        stream.seek(60)
        offset = struct.unpack('<I', stream.read(4))[0]
        stream.seek(offset)
        if stream.read(6) != b'PE\x00\x00\x64\x86':
            raise ValueError('Source is not PE AMD64')


def update(source, destination=PLAYTEST, check=False):
    source, destination = Path(source).resolve(), Path(destination).resolve()
    target = destination / 'ctw_game.exe'
    if not target.is_file():
        raise FileNotFoundError('Existing Playtest game executable required')
    validate_pe(source)
    new_hash, old_hash = digest(source), digest(target)
    result = {'source': str(source), 'destination': str(target), 'sha256': new_hash,
              'previous_sha256': old_hash, 'check_only': check, 'updated': False}
    if check or new_hash == old_hash:
        return result
    # Do not edit metadata, preferences, DLLs, assets or saves. This receipt is separate.
    backups = destination / 'binary-backups'
    backups.mkdir(exist_ok=True)
    backup = backups / (old_hash + '.exe')
    if backup.exists():
        if digest(backup) != old_hash:
            raise ValueError('Existing backup failed integrity check')
    else:
        with target.open('rb') as src, backup.open('xb') as dst:
            shutil.copyfileobj(src, dst)
        if digest(backup) != old_hash:
            raise ValueError('Backup integrity check failed')
    fd, temporary = tempfile.mkstemp(prefix='.ctw-update-', suffix='.tmp', dir=destination)
    os.close(fd)
    temporary = Path(temporary)
    try:
        shutil.copyfile(source, temporary)
        if digest(temporary) != new_hash or digest(target) != old_hash:
            raise ValueError('Concurrent change or copy integrity failure')
        os.replace(temporary, target)
    finally:
        temporary.unlink(missing_ok=True)
    result.update(updated=True, backup=str(backup))
    (backups / (new_hash + '.json')).write_text(json.dumps(result, indent=2), encoding='utf-8')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', nargs='?', type=Path)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    source = args.source
    if source is None:
        build = json.loads((ROOT / 'work/cpp-mainline/latest-build.json').read_text())
        if build.get('tests', {}).get('failed') != 0 or not build.get('game'):
            raise ValueError('No validated local build')
        source = ROOT / build['game']
        if digest(source) != build['game_sha256']:
            raise ValueError('Validated build hash mismatch')
    print(json.dumps(update(source, check=args.check), indent=2))


if __name__ == '__main__':
    main()
