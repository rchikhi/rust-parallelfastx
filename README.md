# Rust-parallelfastx

A truly parallel parser for FASTA/FASTQ files.

## Principle

The input file is memory-mapped then virtually split into N chunks. Each chunk is fed to a regular FASTA/FASTQ parser (here, the excellent https://github.com/markschl/seq_io library).

## Rationale

Virtually all other "multithreaded" FASTA/FASTQ parsers typically use only one thread to parse the file, then they feed the parsed sequences to threads. If your disk is fast enough (> 2 GB/s) that parsing the file becomes a CPU bottleneck, then you might benefit from this library as the parsing is truly multithreaded.

## How to use

see `src/main.rs`, should be self explanatory.

## Inspiration

Inspiration for this repository is the amazing `fastlwc-mt` tool from https://github.com/expr-fi/fastlwc which does multi-threaded line counting. 

## Author

Rayan Chikhi, 2022
