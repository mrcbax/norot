# NoRot [![License: LGPL v3.0](https://img.shields.io/badge/license-LGPL%20v3.0-blue.svg)](https://github.com/LogoiLab/norot/LICENSE)
A command line tool to prevent bit-rot of specific files.

Bit-rot is a problem for any storage device. Unfortunately there are no easy ways to prevent it for crucial data.

NoRot is for those of us without RAID arrays who would like to keep a couple critical files pristine.

How it works:
-

NoRot takes the file you tell it to protect, compresses it, and copies it to a directory you specify an amount of times(you also specify). It does this as often as you want. It then regularly compares the compressed version to the original using sha256 checksums. If they do not differ it will overwrite the original (as often as you specify). If they do differ it identifies if it is the original or the copy, if it is the original it replaces it with the copy. If the copy differs it attempts to use a different copy(if one exists). If that fails it compares the original to an older checksum for itself, if there is no change it reduplicates the copies.

If copies and the original differ from both eachother and themselves there is a major issue and you will be notified. Also, your storage probably just failed big time if this happens.

Development progression:
-

- [ ] Generic Linux
- [ ] Windows
- [ ] Mac OSX

Crates:
-

- [clap ![clap crates.io](https://img.shields.io/crates/v/clap.svg)](https://crates.io/crates/clap)
- [lz4-compress ![lz4-compress crates.io](https://img.shields.io/crates/v/lz4-compress.svg)](https://crates.io/crates/lz4-compress)
- [serde ![serde crates.io](https://img.shields.io/crates/v/serde.svg)](https://crates.io/crates/serde)
