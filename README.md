# CRcli

Commandline crc calculator

**Work in progress**

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