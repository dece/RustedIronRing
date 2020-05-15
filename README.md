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

- `ironring`, a library with all the parsing/unpacking features implemented.
- `rir`, an executable to use main lib features from the CLI.

The goal is to make the lib compatible with FFI tools such as Python's ctypes,
to ship a dynamic lib accessible for any language to easily script tasks and
ideas, but we're not there yet.

Ironring usage:

```
Rusted Iron Ring

USAGE:
    rir [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    bhd     Extracts BHD/BDT contents
    bhds    Extracts all BHD/BDT content (alphabetically) in a folder
    bhf     Extracts BHF/BDT contents
    bnd     Extracts BND contents
    dcx     Extracts and decompress DCX data
    hash    Calculates hash for a string
    help    Prints this message or the help of the given subcommand(s)
```



Features
--------

- BHD5 / BDT: extraction from disk to disk.
- DCX: decompression from disk to disk/memory.
- BND (v3): extraction from disk/memory to disk/memory, optionally decompress
    from DCX.
- BHF (v3): extraction from disk/memory to disk/memory.

Repacking is not supported, maybe one day. It is not that useful when using
[UDSFM][udsfm] and [Yabber][yabber], but if you really need it you can check out
[SiegLib][sieglib].

[udsfm]: https://github.com/HotPocketRemix/UnpackDarkSoulsForModding
[yabber]: https://github.com/JKAnderson/Yabber
[sieglib]: https://github.com/Dece/DarkSoulsDev/tree/master/Programs/SiegLib

There is a demo Python binding for some `name_hashes` features in the
`bindings/python` dir, that uses [PyO3][pyo3] and thus requires nightly rustc to
build.

[pyo3]: https://pyo3.rs/



Credits
-------

All the fat cats involved in the scene and the [wiki][smwiki].

[smwiki]: http://soulsmodding.wikidot.com/
