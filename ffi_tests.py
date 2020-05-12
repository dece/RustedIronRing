#!/usr/bin/env python3
"""Import hash from the rir dynamic lib."""

import pyironring


path = "/chr/c0000.anibnd.dcx"
hash_u32 = pyironring.name_hashes.hash(path)
print('Hash for path "{}" is {:X}'.format(path, hash_u32))

hash_u32_str = pyironring.name_hashes.hash_as_string(hash_u32)
print("Formatted by Rust: {}".format(hash_u32_str))

hm = pyironring.name_hashes.load_name_map("res/namefile.txt")
#print("Hash map for DS1 names: {}".format(hm))
wrong_hm = pyironring.name_hashes.load_name_map("res/nonexistent.txt")
#print("Loading a non-existent names: {}".format(wrong_hm))
