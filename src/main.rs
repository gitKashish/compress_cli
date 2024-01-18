extern crate flate2;

use flate2::{write::GzEncoder,
            Compression};
use std::{env::args,
        fs::{File,metadata},
        time::Instant};
use std::io::{copy, BufReader};

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let source_path = args().nth(1).unwrap();
    let target_path = args().nth(2).unwrap();
    let md = metadata(source_path.clone()).unwrap();
    if md.is_dir() {
        eprintln!("Error: Source path is a directory");
        return;
    }
    
    let source_file = File::open(source_path);
    let source_file = match source_file {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Problem opening the source file");
            return;
        }
    };

    let mut input = BufReader::new(source_file);

    let output = File::create(target_path).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
        );

    println!(
        "Target len: {:?}",
        output.metadata().unwrap().len()
        );

    println!(
        "Elapsed: {:?}",
        start.elapsed()
        );

}
