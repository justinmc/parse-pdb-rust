mod atom;
mod chain;
mod residue;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &'static str = "./dat/3aid.pdb";

    let molecule: Molecule = Molecule::from_file(filename);
    println!("{}", molecule.to_string());
    println!("{}", molecule.get_stats());
}

// TODO Move to its own file, but I was having module directory problems...
pub struct Molecule {
    pub atoms: Vec<atom::Atom>,
    pub chains: HashMap<String, chain::Chain>,
    pub filename: String,
    pub residues: Vec<residue::Residue>,
}

impl Molecule {
    pub fn from_file(filename: &str) -> Molecule {
        let mut atoms: Vec<atom::Atom> = Vec::new();
        let mut residues: Vec<residue::Residue> = Vec::new();
        let mut chains: HashMap<String, chain::Chain> = HashMap::new();

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
                                if !chains.contains_key(&residue.chain_id) {
                                    chains.insert(
                                        residue.chain_id.clone(),
                                        chain::Chain {
                                            chain_id: residue.chain_id.clone(),
                                        },
                                    );
                                }
                                residues.push(residue);
                            }
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
            chains,
            filename: filename.to_string(),
            residues,
        };
    }

    pub fn get_stats(&self) -> String {
        return format!(
            "Molecule {filename} with {num_atoms} atoms and {num_residues} residues and {num_chains} chains",
            filename = self.filename,
            num_atoms = self.atoms.len(),
            num_chains = self.chains.len(),
            num_residues = self.residues.len(),
            );
    }

    pub fn to_string(&self) -> String {
        let mut string: String = String::new();
        for atom in &self.atoms {
            string = format!("{}\n{}", string, atom.to_string());
        }
        for residue in &self.residues {
            string = format!("{}\n{}", string, residue.to_string());
        }
        for (_, chain) in &self.chains {
            string = format!("{}\n{}", string, chain.to_string());
        }
        return string;
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
