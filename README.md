# A simple bf compiler

## Usage

all commands

```
bf_compiler --help
Usage: bf_compiler [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>
  -m, --mode <MODE>              [default: comp] [possible values: int, comp]
  -s, --stack-size <STACK_SIZE>  [default: 30000]
  -t, --time
  -o, --output <OUTPUT>          [default: ./out.o]
  -h, --help                     Print help
  -V, --version                  Print version
```

compile and execute a bf file
```
bf_compiler -i ./hanoi.bf -o ./hanoi.out
ld ./hanoi.out -o ./hanoi
./hanoi
```

interpret a bf file
```
bf_compiler --mode int -i ./hanoi.bf
```

## Building from source

you need to have cargo to build this project
```
cargo build --release
```
the binary should be in ./target/release/bf_compiler
