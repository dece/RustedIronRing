Rusted Iron Ring
================

Low-level library for exploring From Software games files.

This project is mainly to play with the Rust language, Nom parser, FFI, etc; if
you need an actually used and tested library, see [SoulsFormats][soulsformats]
(C#) or [soulstruct][soulstruct] (Python). The main target has been Dark Souls 1
PTDE, but checkout the features section below.

[soulsformats]: https://github.com/JKAnderson/SoulsFormats
[soulstruct]: https://github.com/Grimrukh/soulstruct



Usage
-----

The project contains 2 artefacts:

- `ironring`, a library with all the projects features implemented.
- `rir`, an executable to use main lib features from the CLI.

```
Rusted Iron Ring

USAGE:
    rir [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    bhd         Extracts BHD/BDT contents
    bhds        Extracts all BHD/BDT content (alphabetically) in a folder
    bhf         Extracts BHF/BDT contents
    bnd         Extracts BND contents
    dat         Extracts King's Field IV DAT contents
    dat-pack    Packs files in a King's Field IV DAT
    dcx         Extracts and decompress DCX data
    hash        Calculates hash for a string
    help        Prints this message or the help of the given subcommand(s)
    param       Parses PARAM contents
    paramdef    Prints PARAMDEF contents
```



Features
--------

### Format support

| Type     | Games | Features                                 |
|----------|-------|------------------------------------------|
| BHD5/BDT | DS1   | Load, extract                            |
| DCX      | DS1   | Load, extract, repack (untested)         |
| BND3     | DS1   | Load, extract                            |
| BHF3     | DS1   | Load, extract                            |
| DAT      | KF4   | Load, extract, repack                    |
| PARAMDEF | DS1   | Pretty-print                             |
| PARAM    | DS1   | Pretty-print, optionally with a PARAMDEF |

Formats typically found within DCX files can usually be decompressed on the fly.

Repacking is mostly not supported, maybe one day. It is not that useful when
using [UDSFM][udsfm] and [Yabber][yabber], but if you really need it you can
check out [SiegLib][sieglib].

[udsfm]: https://github.com/HotPocketRemix/UnpackDarkSoulsForModding
[yabber]: https://github.com/JKAnderson/Yabber
[sieglib]: https://github.com/Dece/DarkSoulsDev/tree/master/Programs/SiegLib

### Misc

- Encrypted archive name hasher.
- There is a demo Python binding for some `name_hashes` features in the
    `bindings/python` dir, that uses [PyO3][pyo3] and thus requires nightly
    rustc to build.
- There are a few scripts useful for some testing/modding tasks.

[pyo3]: https://pyo3.rs/



Credits
-------

TKGP and all the fat cats involved in the scene and the [wiki][smwiki].

[smwiki]: http://soulsmodding.wikidot.com/
