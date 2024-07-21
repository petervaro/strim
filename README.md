# `strim`

[![status-badge](https://ci.codeberg.org/api/badges/13539/status.svg)][badge]

This crate provides a single procedural macro that removes all blank lines and
leading and trailing whitespace from a string or byte-string literal.

## Rationale

Rust's string (and byte-string) literals are very convenient to use when it
comes to breaking them into multiple lines by utilising the `\` character, e.g.

```rust
let string = "\
    Hello,\
    World!\
";
assert_eq!(string, "Hello,World!");
```

However when one is _forced_ to work with raw-literals, the `\` character cannot
be employed anymore to un-break and unindent (i.e. _trim_) the lines.  This is
the main use-case where the `strim::trim` macro shines.

```rust
use strim::trim;

let string = trim!(r#"
    "Hello,
     World!"
"#);
assert_eq!(string, "\"Hello,World!\"");
```

## Installation

```toml
[dependencies]
strim = "~0.2.0"
```

Or use:

```bash
$ cargo add strim@~0.2.0
```

## Bug Reports and Feature Requests

If you find something that doesn't work as expected and you wish to report it,
or if you would like to submit a feature request, please do both of these in the
'issues' section of the [original repository on Codeberg][repo].

## Development and Contribution

The simplest and quickest way to see the result of your changes is to use the
`tests` project included in this repository, i.e.

```bash
$ cd tests/
$ cargo test
```

All invalid cases must produce easy to read, correct, and properly referring
compile errors, i.e. they should underline and complain about the relevant bits
which are incorrect.  The tests which result in such compile errors are all
hidden under the `compile-errors` feature.

```bash
$ cd tests/
$ cargo test --features compile-errors
```

## License

Copyright &copy;2024 Peter Varo

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but **without
any warranty**; without even the implied warranty of
**merchantability** or **fitness for a particular purpose**.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses>.

<!-- LINKS -->
[badge]: https://ci.codeberg.org/repos/13539
[repo]: https://codeberg.org/petervaro/strim
