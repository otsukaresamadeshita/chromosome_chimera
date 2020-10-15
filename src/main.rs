use std::collections::HashMap;
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
    // since there is only one occurrence per qname, we can use xor

fn main() {
    let args = Cli::from_args();
    let infile = std::fs::read_to_string(&args.infile)
    .expect("could not read file");
    let mut frequency: HashMap<&str, u32> = HashMap::new(); //frequency hashmap
    // let mut readhash: HashMap<&str, u32> = HashMap::new(); //qname and whole read hashmap   
    let mut seqreads = String::from("");
    // let mut seqreads = vec![];
    for read in infile.lines() {
        if read.contains(&args.sequence) {
            seqreads.push_str(read);
            let readiter = read.split_whitespace().nth(0).unwrap();
            *frequency.entry(&readiter).or_insert(0) += 1;
            // let readiter: Vec<_> = read.split_whitespace().nth(0)
                // .into_iter()
                // .map(|s| s.parse::<String>())
                // .filter_map(Result::ok)
                // .collect();
                // println!("{:?}", &readiter); 
                // let dupqnames = readiter.into_iter().nth(0).unwrap();
                // *frequency.entry(read).or_insert(0) += 1;
                // let dupqnames = readiter.into_iter().nth(0).unwrap();
        }
    }
    // seqreads.sort_unstable();
    for (key, val) in frequency.iter() {
        if val == &1 { //only one occurrence, aka single occurrence over the chromosome
            for seq in seqreads.lines() {
                if seq.contains(key) {
                    println!("{}", seq); 
                    // println!("{:?}", seqreads.find(key)); 
                
                }//grep the original file
            }
        }  
    }
    // println!("{}", seqreads); 
}


// fn get_seqreads_index(name: &String, array: &Vec<String>) -> Option<usize> {
//     match array.binary_search(name) {
//         Ok(index) => Some(index),
//         Err(_)    => None,
//     }
// }

// let needle = "list".into();
// let haystack: Vec<_> = vec!["some", "long", "list", "of", "strings"]
//     .into_iter()
//     .map(String::from)
//     .collect();
// if haystack.contains(&needle) {
//     println!("{}", needle);
// } else {
//     println!("not found");
// }

// fn get_qname(infile: String, args: String){
//     for read in infile.lines() {
//         if read.contains(&args.sequence) {
//             let readiter: Vec<_> = read.split_whitespace()
//                 .into_iter()
//                 .map(|s| s.parse::<String>())
//                 .filter_map(Result::ok)
//                 .collect();
//         }
//     }
// }

// fn hashmap_count(infile: String) -> &'static str {
//     for read in infile.lines() {
//         *frequency.entry(read).or_insert(0) += 1;
//     }
//     for (key, val) in frequency.iter() {
//         if val == &1 { //only one occurrence, aka single occurrence over the chromosome
//             // println!("key: {} val: {}", key, val);
//             println!("{}", key);
            
//         }
//     }
// }