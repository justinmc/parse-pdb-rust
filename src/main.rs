mod atom;
mod residue;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &'static str = "./dat/3aid.pdb";

    let molecule: Molecule = Molecule::from_file(filename);
    println!("{}", molecule.to_string());
}

// TODO Move to its own file, but I was having module directory problems...
pub struct Molecule {
    pub atoms: Vec<atom::Atom>,
    pub filename: String,
    pub residues: Vec<residue::Residue>,
}

impl Molecule {
    pub fn from_file(filename: &str) -> Molecule {
        let mut atoms: Vec<atom::Atom> = Vec::new();
        let mut residues: Vec<residue::Residue> = Vec::new();

        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(ip) = line {
                    let identifier = &ip[0..6];

                    match identifier {
                        "ATOM  " => {
                            // TODO error handling if data is bad?
                            atoms.push(atom::Atom::from_string(ip));
                        }
                        "SEQRES" => {
                            let new_residues: Vec<residue::Residue> =
                                residue::Residue::parse_seq_res_entry(ip);
                            for residue in new_residues {
                                residues.push(residue);
                            }
                            // TODO add to chains.
                        }
                        _ => {
                            // These lines are not relevant with the current functionality.
                        }
                    }
                }
            }
        }
        return Molecule {
            atoms,
            filename: filename.to_string(),
            residues,
        };
    }

    pub fn to_string(&self) -> String {
        return format!(
            "Molecule {filename} with {num_atoms} atoms and {num_residues} residues",
            filename = self.filename,
            num_atoms = self.atoms.len(),
            num_residues = self.residues.len(),
        );
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
