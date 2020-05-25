#!/usr/bin/env python3
"""Patch a file with another file at offset. Useful for dirty DAT patching."""

import argparse


def main():
    argparser = argparse.ArgumentParser()
    argparser.add_argument("archive", help="File to be modified")
    argparser.add_argument("file", help="File to patch in archive")
    argparser.add_argument("offset", help="Offset as hex")
    args = argparser.parse_args()

    archive_path, input_file_path = args.archive, args.file
    ofs = int(args.offset.lstrip("0x").rstrip("h"), 16)
    patch(archive_path, input_file_path, ofs)


def patch(archive_path, input_file_path, ofs):
    with open(input_file_path, "rb") as input_file:
        input_data = input_file.read()
    with open(archive_path, "rb+") as archive_file:
        archive_file.seek(ofs)
        archive_file.write(input_data)


if __name__ == "__main__":
    main()
