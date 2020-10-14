use structopt::StructOpt;
// use std::{fs::File};
// use std::io::{Write, BufReader, BufRead, Error};

// use noodles_bam;
// use noodles_sam;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// Sequence to search
    #[structopt(
        short="s",
        long="--sequence"
    )]
    sequence: String,

    /// Path to the file to read
    #[structopt(
    short="i", 
    long = "--infile",
    parse(from_os_str)
)]
    infile: std::path::PathBuf,

    // /// Path to the file to write
    // #[structopt(
    //     short="o", 
    //     long = "--outfile",
    //     parse(from_os_str)
    // )]
    //     path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let file = std::fs::read_to_string(&args.infile)
    .expect("could not read file");
    for read in file.lines() {
        if read.contains(&args.sequence) {
                let readiter: Vec<_> = read.split_whitespace()
                    .into_iter()
                    .map(|s| s.parse::<String>())
                    .filter_map(Result::ok)
                    .collect();
                let dupqnames = readiter.into_iter().nth(0).unwrap();
                println!("{}", dupqnames);
        }
    }
}

    // let split = line.split_whitespace();
    // let seqvec = split.collect::<Vec<&str>>();
    // println!("{:?}", split);


                // let qnamecounts: HashMap<&str, i32> = dupqnames.iter().cloned().collect();
                // // use the values stored in map
                // let nondupnames: Vec<_> = dupqnames.into_iter().map(String::from);
                // nondupqnames.sort_by();
                // nondupqnames.dedup();
                // let mut frequency: HashMap<&str, u32> = HashMap::new();
                // let freqiter: Vec<_> = read
                    // .into_iter()
                    // .map(|s| s.parse::<String>())
                    // .filter_map(Result::ok)
                // let nondupqnames = &dupqnames.;
                // let test = nondupqnames.sort()

// fn get_qnames() {
//     let readiter: Vec<_> = read.split_whitespace()
//     .into_iter()
//     .map(|s| s.parse::<String>())
//     .filter_map(Result::ok)
//     .collect();
//     readiter.into_iter().nth(0);
// }

// use std::collections::HashMap;
// fn sequence_frequency(){
//     let mut frequency: HashMap<&str, u32> = HashMap::new();
//     for read in file.lines() {
//         if read.contains(&args.sequence) {
//     *frequency.entry(&read).or_insert(0) += 1;
// }
// println!("{:?}", frequency);
// }
// }