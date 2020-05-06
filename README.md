# grep-table-converter (`gtc`)
[![Build Status](https://travis-ci.com/zeroclock/grep-into-table.svg?branch=master)](https://travis-ci.com/zeroclock/grep-into-table)

`gtc` is a cli utility to convert grep result to table (csv, markdown, textile) written in Rust.

``` sh
# grep
$ grep -rn 'grep' ./src | gtc -o output-grep.csv -m csv
# ripgrep
$ rg -n grep ./src | gtc -o output-ripgrep.csv -m csv
# input file
$ gtc -o output.csv -m markdown input.txt
```

*NOTE: make sure that input text contains line number! (using `-n` option)*

input:

```
./src/main.rs:1:extern crate grep_table_converter;
./src/main.rs:11:use grep_table_converter::generator::*;
./src/main.rs:12:use grep_table_converter::io::*;
./src/main.rs:15:#[structopt(name = "grep_table_converter")]
```

output:

[csv]
```
file_name,line_num,content
./src/main.rs,1,extern crate grep_table_converter;
./src/main.rs,11,use grep_table_converter::generator::*;
./src/main.rs,12,use grep_table_converter::io::*;
./src/main.rs,15,#[structopt(name = "grep_table_converter")]
```

[markdown]

```
| file_name | line_num | content |
| --- | --- | --- |
| ./src/main.rs | 1 | extern crate grep_table_converter; |
| ./src/main.rs | 11 | use grep_table_converter::generator::*; |
| ./src/main.rs | 12 | use grep_table_converter::io::*; |
| ./src/main.rs | 15 | #[structopt(name = "grep_table_converter")] |
```

[textile]

```
|file_name|line_num|content|
|./src/main.rs|1|extern crate grep_table_converter;|
|./src/main.rs|11|use grep_table_converter::generator::*;|
|./src/main.rs|12|use grep_table_converter::io::*;|
|./src/main.rs|15|#[structopt(name = "grep_table_converter")]|
```

## Getting Started

### Installing

Cargo:

``` sh
$ cargo install grep-table-converter
```

MacOS:
Windows:
Linux:

TODO

### Convertable format

```
[file_path]:[line_number]:[code]
```

### Parameters

Required arguments:

```
-m --mode	Convert mode (csv, markdown, textile)
```

Optional arguments:

| Parameter                 | Default       | Description   |	
| :------------------------ |:-------------:| :-------------|
| -o --output-filename | output_`datetime`.`csv/md/textile` | output filename |
| input-filename | | input filename instead of stdin |

## Build and test

``` sh
# Build
$ cargo build

# Test
$ cargo test
```

## Contributing

Please feel free to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
