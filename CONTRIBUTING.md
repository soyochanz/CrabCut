# Contributing to CrabCut

CrabCut is an independent fork of Concat, distributed under AGPL-3.0-or-later.
Please submit issues and pull requests to https://github.com/soyochanz/CrabCut.
The upstream contributor agreement is preserved for historical reference in CLA.md;
it is not required for contributions submitted to this fork.

Build instructions are in src/README.md. Run from src:

```sh
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
```

For interface changes also run `python3 scripts/locales.py --check` from the root.
Keep upstream copyright and SPDX notices. Describe new features and test results.
