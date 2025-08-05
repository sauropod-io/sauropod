# Contributing

## Development dependencies

- [Rust](https://www.rust-lang.org/tools/install) >= 1.85
- [Clang](https://clang.llvm.org/) >= 19
- [CMake](https://cmake.org/)
- OpenSSL
- [pre-commit](https://pre-commit.com/) >= 4.2.0
  - _Recommended_: Install the git hooks with `pre-commit install --install-hooks`

### Recommended Linker

Install the [`mold`](https://github.com/rui314/mold) linker and configure it for your default target for faster linking.

```toml
[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang"
rustflags = ["-C", "link-arg=--ld-path=/usr/bin/mold"]
```

## Running auto-fixes

Before pushing a PR make sure to run the automatic fixers:

```bash
make fix
```
