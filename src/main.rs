mod molecule;

fn main() {
    let filename: &'static str = "./dat/3aid.pdb";

    let molecule: molecule::Molecule = molecule::Molecule::from_file(filename);
    println!("{}", molecule.to_string());
    println!("{}", molecule.get_stats());
}
