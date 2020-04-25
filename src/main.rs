mod atom;
mod residue;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static ATOM_NAME: &str = "ATOM  ";
static RESIDUE_NAME: &str = "SEQRES";

fn main() {
    let filename: &'static str = "./dat/3aid.pdb";

    println!("In file {}", filename);

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                parse_line(ip);
            }
        }
    }
}

fn parse_line(line: String) {
    // TODO use match instead of if/else
    if &line[0..6] == ATOM_NAME {
        // TODO error handling if data is bad?
        let atom = atom::Atom::new(line);
        println!("{}", atom.to_string());
    } else if &line[0..6] == RESIDUE_NAME {
        let residues: Vec<residue::Residue> = residue::Residue::parse_seq_res_entry(line);
        for residue in residues {
            println!("{}", residue.to_string());
        }

        // TODO add to chains.
    }
    // TODO chain
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
