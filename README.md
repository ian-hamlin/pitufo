# pitufo

A small application that will walk the given path and replace the content of every json file found.  The file will either be prettified or minified.

## Usage

```console
usage of pitufo

Options:
  --follow          follow symbolic links, the default is to not follow.
  --minify          minify the json, the default is to prettify.
  -m, --max-depth   set the maximum depth to recurse
  -p, --path        the path to search for json files.
  --help            display usage information
```

## Example

recursively walk the path and prettify or minify everything you can find

```console
  ./pitufo /GitHub/verb-data/json
```

```console
  ./pitufo /GitHub/verb-data/json --minify
```

prettify or minify everything you can find *directly* under the path

```console
  ./pitufo /GitHub/verb-data/json --max-depth 1
```

```console
  ./pitufo /GitHub/verb-data/json --minify  --max-depth 1
```

## Building

Install Rust as per the instructions here https://www.rust-lang.org/tools/install

```console
git clone https://github.com/ian-hamlin/pitufo.git
cd pitufo
cargo build --release
./target/release/pitufo path/to/json
```

## Installation

pitufo can be installed via cargo.

```console
cargo install pitufo
```

## Notes

pitufo will:

* silently ignore any path that it can not access.
* report files it can not change to stderr.
* not provide any feedback on success or progress.

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
