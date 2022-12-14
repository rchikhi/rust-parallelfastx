use memmap2::Mmap;
use seq_io::BaseRecord;

pub fn parallel_fastx<F>(filename: &str, nb_threads: usize, task: F)
    where F: Send + Sync + Fn(&[u8],&str)
{
    let f = std::fs::File::open(filename).expect("Error: file not found");
    let mmap = unsafe { Mmap::map(&f).expect(&format!("Error mapping file {}", filename)) };
    mmap.advise(memmap2::Advice::Sequential).unwrap(); // 15% perf gain with this

    // Determine chunks 
    let len_file = mmap.len();
    let mut start_pos : Vec<usize> = Vec::new();
    let mut end_pos : Vec<usize> = Vec::new();
    let mut mmap_chunk = Vec::new();
    let mut big_enough_file = true;
    start_pos.push(0);
    for i in 1..nb_threads
    {
	let mut start = i*len_file/nb_threads;
	// adjust starting position of chunk to align with fastq or fasta 
	while (start < len_file) && (! ((mmap[start] == '@' as u8 || mmap[start] == '>' as u8) && (mmap[start-1] == '\n' as u8 || mmap[start-1] == '\r' as u8))) { start += 1};
        if start >= len_file - 1 { big_enough_file = false; };
	end_pos.push(start-1);
	start_pos.push(start);
    }
    end_pos.push(len_file-1);
    for i in 1..nb_threads{
        if start_pos[i] == start_pos[i-1] {
            big_enough_file = false;
            break;
        }
    }
    if ! big_enough_file {
        println!("File too 'small' to be parsed using rust-parallelfastx. Falling back to 1 thread.");
        start_pos[0] = 0;
        end_pos[0] = len_file;
    }
    let nb_threads = if big_enough_file { nb_threads } else {1};

    for i in 0..nb_threads {
	mmap_chunk.push(mmap[start_pos[i]..end_pos[i]].as_ptr() as usize);
    }

    // Start FASTX parsing threads
    std::thread::scope(|scope|  {// since rust 1.63
        let task = &task;
        let mut threads = vec![];
        for i in 0..nb_threads {
            // the things rust make us do..
            let start_pos_i = start_pos[i];
            let end_pos_i = end_pos[i];
            let mmap_chunk_i = mmap_chunk[i];		
            threads.push(scope.spawn(move || {
                unsafe{
                    let mut reader = seq_io::fastx::Reader::new(std::slice::from_raw_parts(mmap_chunk_i as *const u8,end_pos_i-start_pos_i));
                    while let Some(result) = reader.next() {
                        let rec = result.unwrap();
                        let seq_str = rec.seq(); 
                        let seq_id = rec.id().unwrap().to_string();
                        task(&seq_str,&seq_id); 
                    }
                }
            }));
        }
        for thread in threads {
            let _ = thread.join();
        }
    });

}


