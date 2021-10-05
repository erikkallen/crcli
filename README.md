# CRcli

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat)](https://github.com/erikkallen/crcli/blob/dev/LICENSE)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/erikkallen/crcli/Continuous%20integration)
![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/erikkallen/crcli?include_prereleases)
![Libraries.io dependency status for GitHub repo](https://img.shields.io/librariesio/github/erikkallen/crcli)

Commandline crc calculator

**TODO**
- [ ] Search for crc algorythm based on crc and file/hex string
- [ ] Print all possible crc's
- [ ] Package for debian, redhat and arch

Features:

- Calculate any type of crc for files, hex strings, text

## Examples

```shell
# CRC a file
crcli -t crc16 file.txt

# CRC a string of hex
crcli -t crc16_modbus --hex "34 56 34 76"

# CRC a hex string with custom seperator
crcli -t crc16 --hex "34, 56, 34, 76" -s ", "
```

**NOTE:** This requires a fairly recent version of rust, if you get an error like the one below please update your rust version

```sh
error[E0658]: arbitrary expressions in key-value attributes are unstable
 --> /home/erikkallen/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-3.0.0-beta.4/src/lib.rs:8:10
  |
8 | #![doc = include_str!("../README.md")]
  |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #78835 <https://github.com/rust-lang/rust/issues/78835> for more information
```

## Thanks to the creators of these awesome crates:
* [crc-any](https://github.com/magiclen/crc-any) by [magiclen](https://github.com/magiclen)
* [clap](https://github.com/clap-rs/clap) by [Kevin B. Knapp](https://github.com/kbknapp)
* [rust-hex](https://github.com/KokaKiwi/rust-hex) by [KokaKiwi](https://github.com/KokaKiwi)

## License

`CRcli` is distributed under the terms of the MIT license.

See the [LICENSE](LICENSE) file in this repository for more information.