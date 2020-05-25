#!/usr/bin/env python3
"""Group KF4 sound files together in named dirs for later use w/ mkpsf2.

Some tools to handle sounds have issues with PS2 sounds split as HD/BD/SQ, so
grouping them with this script makes it easier to batch process them with mkspf2
so you can process them later as PSF2.

Example:

```bash
$ ./kf4-group-sounds.py workspace/dat/snd/ workspace/soundgroups
$ cd workspace/soundgroups
$ ls -lF
bgm000/
bgm001/
...
$ fd -t d -x wine mkpsf2 {}.psf2 {}
```
"""

import argparse
import glob
import os
import shutil
from pathlib import Path


def main():
    argparser = argparse.ArgumentParser()
    argparser.add_argument("snd_dir")
    argparser.add_argument("output_dir")
    args = argparser.parse_args()
    copy_files(Path(args.snd_dir), Path(args.output_dir))


def copy_files(snd_path, output_path):
    filenames = set([os.path.splitext(f)[0] for f in os.listdir(snd_path)])
    for filename in filenames:
        subfiles = glob.glob(str(snd_path / "{}.*".format(filename)))
        files_output_path = output_path / filename
        if not files_output_path.exists():
            files_output_path.mkdir(parents=True)
        for f in subfiles:
            shutil.copyfile(f, files_output_path / os.path.basename(f))


if __name__ == "__main__":
    main()
