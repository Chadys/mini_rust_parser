## Mini Rust parser

Really simple experiment in Rust to parse some json objects.
This tool needs an input file containing a json object on each line, including a field `type`.
An analysis result will be displayed as an aggregation of some infos for each type.

### Example file input:
```json
{"type":"error", "description": "foobar", "items":[1, 2]}
{"type": "success", "random": 5.0 }
{"type": "error", "status": "42"}
```

This project needs cargo to be built 
and the documentation uses [RFC 1946](https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links.html)
so you should use nightly to build it (`cargo +nightly doc`)

### Usage
After having correctly built the binary, you need to call it with
```shell script
./mini_rust_parser <INPUT_FILE>
```
For more info use:
```shell script
./mini_rust_parser --help
```
