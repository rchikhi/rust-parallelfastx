# Rust-parallelfastx

A truly parallel parser for FASTA/FASTQ files.

## Principle

The input file is memory-mapped then virtually split into N chunks. Each chunk is fed to a regular FASTA/FASTQ parser (here, the excellent https://github.com/markschl/seq_io library).

## Rationale

Virtually all other "multithreaded" FASTA/FASTQ parsers typically use only one thread to parse the file, then they feed the parsed sequences to threads. If your disk is fast enough (> 2 GB/s) that parsing the file becomes a CPU bottleneck, then you might benefit from this library as the parsing is truly multithreaded.

## How to use

see `src/main.rs`, should be self explanatory.

## Related work

Inspiration for this repository is the amazing `fastlwc-mt` tool from https://github.com/expr-fi/fastlwc which does multi-threaded line counting faster than `wc`.

A related work on parallel FASTX parsing, I was not aware of at the time of development, is: https://github.com/natir/in_place_fastx

## Caveat

Input file needs to be seekable, which rules out all compression methods except blocked ones, which currently aren't supported by this library, but could be in principle.

## Author

Rayan Chikhi, 2022
