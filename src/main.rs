use std::collections::HashMap;
use structopt::StructOpt;
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
}

fn main() {
    let args = Cli::from_args();
    let infile = std::fs::read_to_string(&args.infile)
    .expect("could not read file");
    let mut frequency: HashMap<&str, u32> = HashMap::new();
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
        if val == &1 {
            for seq in seqreads.lines() {
                if seq.contains(key) {
                println!("{}", seq)
                }
            }
        }
    }
}
