# Practical Cryptography for Developers (in Rust)

This repository contains examples of how to use various Rust libraries to perform cryptographic operations safely and with purpose.

The primary purposes of cryptography are:

- Confidentiality
- Integrity
- Non-repudiation
- Authentication

The examples in this repository depend on the [sodiumoxide](https://crates.io/crates/sodiumoxide) and [sha3](https://crates.io/crates/sha3) crates.

This is inspired by:

- [aeden/pc4d-go](https://github.com/aeden/pc4d-go)
- [aeden/pc4d-ruby](https://github.com/aeden/pc4d-ruby)

See also:

- [dmikusa/pc4d-java](https://github.com/dmikusa/pc4d-java)
- [dmikusa/pc4d-python](https://github.com/dmikusa/pc4d-python)

## Examples

To run the examples...

- [Install Rust](https://rustup.rs/).
- Run `cargo build` from the project directory. This will build the example binaries.

You should now be able to run the examples, for example `target/debug/encrypt`, or try running `./test.sh` which runs all four examples and validates they are working correctly.

### Confidentiality

Symmetric examples: `encrypt` & `decrypt`

Asymmetric examples: `pkencrypt` & `pkdecrypt`

Run either `encrypt` or `pkencrypt` with the appropriate arguments and the final output should show the command required to decrypt.

### Integrity

Hash example: `hash`

Run `hash` with the same input argument and you should get the same output argument every time.

Asymmetric example: `pksign` & `pkverify`

Run `pksign` with the appropriate arguments and the final output should show the command required to verify.

### Non-repudiation

Asymmetric examples: `pksign` & `pkverify`

Run `pksign` with the appropriate arguments and the final output should show the command required to verify.

### Authentication

Symmetric example: `auth` & `verify`

Run `auth` with the appropriate arguments and the final output should show the command required to verify.
