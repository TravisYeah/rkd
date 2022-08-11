# RKD

RKD is a delta encoding library.

## Getting Started

add dependency to `cargo.toml`

```config
rk_delta = { git = "https://github.com/TravisYeah/rkpb", tag = "v0.1.0" }
```

See examples of usage in the [tests folder](tests/lib.rs).

## Testing

```bash
cargo test
```

## Compression

Checkout and test the compression in the [compression folder](compression).

## Benchmarks

```bash
cargo bench
```

## Encoding

See encoding information [here](ENCODING.md). The encoding data format is inspired by VCDIFF [1].

## References

[1] D. Korn, et al. "The VCDIFF Generic Differencing and Compression Data Format", RFC 2119, June 2002.
