
# knipp

Generate some words.

frira  
knipp  
toofa  
ajaqc  
vtapu

```
USAGE:
    knipp [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -r, --random     Use a completely random sequence
    -V, --version    Prints version information

OPTIONS:
    -f, --format <format>    Specify formats separated by semicolon
    -l, --length <length>    Generate with a specific length
    -n, --number <number>    Number of words to generate
```

## e.g.

```
$ knipp -f cvcc
bork
$ knipp -f 'vccvc;ccvcvv;vc' -n 4
ur
ebpuh
nxcei
ig
```

## Disclaimer

This program can potentially generate offensive words. If you want to add a
filter, feel free to make a pull request.

