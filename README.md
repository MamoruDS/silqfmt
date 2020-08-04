# silqfmt

A simple formatter for Silq

## Installation

### Prebuilt binaries

Prebuilt binaries are provided in the [releases](https://github.com/MamoruDS/silqfmt/releases) page.

## Usage

```shell
silqfmt -i sample.slq -o output.slq
```

### CLI Options

```
USAGE:
    silqfmt [FLAGS] [OPTIONS] --input <input-file>

FLAGS:
        --hardtab    Using hardtabs instead of spaces
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input-file>      Path of input file
    -o, --output <output-file>    Path of output file
        --tab-size <tab-size>     Size of indent [default: 4]
```

## License

MIT License
copyright © 2020 MamoruDS
