# RKD

RKD is a delta encoding library.

## Getting Started

add dependency to `cargo.toml`

```config
rk_delta = { git = "https://github.com/TravisYeah/rkpb", tag = "v0.1.0" }
```

See examples of usage in the [tests folder](tests/lib.rs).

## Compression

Checkout and test the compression in the [compression folder](compression).

## Benchmarks

```bash
cargo bench
```

## Encoding

See encoding information [here](ENCODING.md).
