Rusted Iron Ring
================

Low-level library for exploring From Software games files. Currently only
supports Dark Souls 1 (PTDE).

This project is mainly to play with the Rust language, Nom parser, FFI, etc; if
you need an actually used and tested library, see [SoulsFormats][soulsformats].

[soulsformats]: https://github.com/JKAnderson/SoulsFormats



Usage
-----

The project contains 2 artefacts:

- `librir`, a library containing all the parsing/unpacking features implemented.
- `ironring`, an executable to use main lib features from the CLI.

The goal is to make the lib compatible with FFI tools such as Python's ctypes,
to ship a dynamic lib accessible for any language to easily script tasks and
ideas, but we're not there yet.

Ironring usage:

```
Iron Ring

USAGE:
    ironring [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    bhd     Extracts BHD/BDT contents
    bhds    Extracts all BHD/BDT content (alphabetically) in a folder
    bnd     Extracts BND contents
    dcx     Extracts and decompress DCX data
    hash    Calculates hash for a string
    help    Prints this message or the help of the given subcommand(s)
```



Features
--------

- BHD5 / BDT: extraction on disk.
- DCX: decompression on disk.
- BND (v3): extraction on disk or in memory.

Repacking is not supported, maybe one day. It is not that useful when using
[UDSFM][udsfm] and [Yabber][yabber], but if you really need it you can check out
[SiegLib][sieglib].

[udsfm]: https://github.com/HotPocketRemix/UnpackDarkSoulsForModding
[yabber]: https://github.com/JKAnderson/Yabber
[sieglib]: https://github.com/Dece/DarkSoulsDev/tree/master/Programs/SiegLib



Credits
-------

All the fat cats involved in the scene and the [wiki][smwiki].

[smwiki]: http://soulsmodding.wikidot.com/
