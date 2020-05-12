#!/usr/bin/env python3
"""Import hash from the rir dynamic lib."""

LIB_SYMLINK = "pyironring.so"

import os
import sys

# Pass the shared object as argument
if len(sys.argv) == 2:
    if os.path.lexists(LIB_SYMLINK):
        os.remove(LIB_SYMLINK)
    os.symlink(sys.argv[1], LIB_SYMLINK)
try:
    import pyironring
except ImportError:
    exit("PyIronring not found, pass the shared lib as parameter for dev.")


def name_hashes():
    path = "/chr/c0000.anibnd.dcx"
    hash_u32 = pyironring.name_hashes.hash(path)
    print('Hash for path "{}" is {:X}'.format(path, hash_u32))

    hash_u32_str = pyironring.name_hashes.hash_as_string(hash_u32)
    print("Formatted by Rust: {}".format(hash_u32_str))

    hm = pyironring.name_hashes.load_name_map("../../res/namefile.txt")
    print("Hash map for DS1 names: {}".format(hm)[:100] + "...")
    try:
        pyironring.name_hashes.load_name_map("nonexistent.txt")
    except FileNotFoundError:
        print("Received FileNotFoundError exception.")


if __name__ == "__main__":
    name_hashes()

    # Clean dev symlink.
    if os.path.lexists(LIB_SYMLINK):
        os.remove(LIB_SYMLINK)
