use std::collections::HashMap;
use structopt::StructOpt;
// use std::io::{BufWriter, BufReader, BufRead, Error};

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
    // outfile: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let infile = std::fs::read_to_string(&args.infile)
    .expect("could not read file");
    // let outfile = std::fs::read_to_string(&args.outfile).expect("could not read file");
    // let out = File::create(&args.outfile);
    // let reader = BufReader::new(out);
    // let mut writer = BufWriter::new(std::io::stdout());
    // let out = File::open(out).unwrap();
    // let mut temp = String::new();
    let mut frequency: HashMap<&str, u32> = HashMap::new(); //frequency hashmap
    let mut seqreads = String::new();
    for read in infile.lines() {
        if read.starts_with('\u{0040}'){println!("{}", read);} else { //'\u{0040}'
            if read.contains(&args.sequence) {
            seqreads.push_str(read);
            let readiter = read.split_whitespace().nth(0).unwrap();
            *frequency.entry(&readiter).or_insert(0) += 1;
            }
        }
    }
    for (key, val) in frequency.iter() {
        if val == &2 { //only one occurrence, aka single occurrence over the chromosome
            for seq in seqreads.lines() {
                if seq.contains(key) {
                println!("{}", seq)
                }
            }
        }
    }
    // file.flush();
    // writer.flush().unwrap()
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