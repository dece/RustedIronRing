#!/usr/bin/env python3
"""Import hash from the rir dynamic lib."""

import sys
from ctypes import cdll

lib = cdll.LoadLibrary(sys.argv[1])
#print(lib.name_hashes.hash("/chr/c0000.anibnd.dcx"))
