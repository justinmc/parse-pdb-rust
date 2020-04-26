mod atom;
mod residue;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &'static str = "./dat/3aid.pdb";

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                parse_line(ip);
            }
        }
    }
}

fn parse_line(line: String) {
    let identifier = &line[0..6];

    // TODO save atoms, res, chains in data structure and output stats or something.
    match identifier {
        "ATOM  " => {
            // TODO error handling if data is bad?
            let atom = atom::Atom::new(line);
            println!("{}", atom.to_string());
        }
        "SEQRES" => {
            let residues: Vec<residue::Residue> = residue::Residue::parse_seq_res_entry(line);
            for residue in residues {
                println!("{}", residue.to_string());
            }
            // TODO add to chains.
        }
        _ => {
            // These lines are not relevant with the current functionality.
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
