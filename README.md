# icnsify

Convert PNGs to [`.icns`](https://en.wikipedia.org/wiki/Apple_Icon_Image_format).

## Installation

### Cargo

```sh
cargo install icnsify
```

### Nix

[Available through Nixpkgs](https://nixpkgs.dev/icnsify).

```
nix run nixpkgs#icnsify
```

## Usage

```
icnsify <IMAGE> [--output <IMAGE>]
```

Provide an input image to transform into a `.icns` file. The provided image must be a PNG with square/equal dimensions.

By default, the resulting `.icns` file will be written to the same path as the input file with the `.icns` extension (`foo.png` becomes `foo.icns`). An alternative output path can be provided with `--output`.

## License

[GPL-3.0](LICENSE)
