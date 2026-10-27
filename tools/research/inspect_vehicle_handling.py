#!/usr/bin/env python3
"""Inspect handling numbers directly from the user's PSP ISO; writes no game assets."""
import argparse
import json
import struct
import zlib


def vehicle_table(path):
    with open(path, 'rb') as iso:
        def read(offset, size):
            iso.seek(offset)
            data = iso.read(size)
            if len(data) != size:
                raise ValueError('Truncated ISO')
            return data
        def u32(data, off):
            return struct.unpack_from('<I', data, off)[0]
        root = read(16 * 2048, 2048)[156:190]
        lba, size = u32(root, 2), u32(root, 10)
        for name in ('PSP_GAME', 'USRDIR', 'WAD', 'GAME.PAK'):
            data = read(lba * 2048, size)
            at = 0
            found = None
            while at < len(data):
                length = data[at]
                if not length:
                    at = (at // 2048 + 1) * 2048
                    continue
                row = data[at:at + length]
                if row[33:33 + row[32]].split(b';')[0] == name.encode():
                    found = u32(row, 2), u32(row, 10)
                    break
                at += length
            if found is None:
                raise ValueError(f'Missing {name}')
            lba, size = found
        h = read(lba * 2048, 2048)
        assert h[:4] == b'20KP'
        thresholds = struct.unpack_from('<8I', h, 4)
        def offset(index):
            page = struct.unpack_from('<H', h, 44 + 2 * index)[0]
            page += 65536 * sum(index >= t for t in thresholds)
            return u32(h, 40) + 2048 * page
        data = read(lba * 2048 + offset(29), offset(30) - offset(29))
        if data[:4] == b'10SA':
            data = zlib.decompress(data[12:12 + u32(data, 4)])
        assert u32(data, 0) == 106
        return data


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('iso')
    parser.add_argument('--ids', default='0,2,7,27,89')
    args = parser.parse_args()
    data = vehicle_table(args.iso)
    for index in map(int, args.ids.split(',')):
        record = data[4 + index * 312:4 + (index + 1) * 312]
        shorts = {hex(off): struct.unpack_from('<h', record, off)[0]
                  for off in range(0x28, 0x4c, 2)}
        words = {hex(off): struct.unpack_from('<i', record, off)[0]
                 for off in range(0x50, 0x8c, 4)}
        print(json.dumps({'id': index, 'shorts': shorts, 'words': words}))


if __name__ == '__main__':
    main()
