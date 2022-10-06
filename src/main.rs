#![feature(bench_black_box)]
use std::time::Instant;
use rust_parallelfastx::parallel_fastx;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let nb_threads = args[2].parse::<usize>().unwrap();
    let dummy = |_seq_str :&[u8], _seq_id: &str| { 
        //println!("{}",_seq_id.to_string()); 
        std::hint::black_box(1); 
    };
    println!("Parsing input FASTX file {} ({} threads)", filename, nb_threads);
    let start = Instant::now();
    parallel_fastx(filename, nb_threads, &dummy);
    let duration = start.elapsed();
    println!("FASTQ parsed in {:?}.", duration);
}
