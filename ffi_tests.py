#!/usr/bin/env python3
"""Import hash from the rir dynamic lib."""

import ctypes
import sys


lib = ctypes.cdll.LoadLibrary(sys.argv[1])

s = ctypes.c_char_p(b"/chr/c0000.anibnd.dcx")
lib.nam_hash.restype = ctypes.c_uint32
h = lib.nam_hash(s)
print(hex(h))

lib.nam_hash_as_string.restype = ctypes.c_char_p
h_str = lib.nam_hash_as_string(h)
print(h_str)
