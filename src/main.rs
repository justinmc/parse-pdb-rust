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

struct Atom {
    serial: i32,
    name: String,
    alt_loc: String,
    res_name: String,
    chain_id: String,
    res_seq: i32,
    i_code: String,
    x: f64,
    y: f64,
    z: f64,
    occupancy: f64,
    temp_factor: f64,
    element: String,
    charge: String,
}

impl Atom {
    fn to_string(&self) -> String {
        return format!(
            "Atom: {name}, {serial}, {alt_loc}, {res_name}, {chain_id}, {res_seq}, {i_code}, {x}, {y}, {z}, {occupancy}, {temp_factor}, {element}, {charge}",
            name = self.name,
            serial = self.serial,
            alt_loc = self.alt_loc,
            res_name = self.res_name,
            chain_id = self.chain_id,
            res_seq = self.res_seq,
            i_code = self.i_code,
            x = self.x,
            y = self.y,
            z = self.z,
            occupancy = self.occupancy,
            temp_factor = self.temp_factor,
            element = self.element,
            charge = self.charge,
        );
    }
}

fn parse_line(line: String) {
    if &line[0..6] == ATOM_NAME {
        // TODO error handling if data is bad?
        let atom = Atom {
            serial: line[6..11].trim().parse::<i32>().unwrap(),
            name: line[12..16].trim().to_string(),
            alt_loc: line[16..17].trim().to_string(),
            res_name: line[17..20].trim().to_string(),
            chain_id: line[21..22].trim().to_string(),
            res_seq: line[22..26].trim().parse::<i32>().unwrap(),
            i_code: line[26..27].trim().to_string(),
            x: line[30..38].trim().parse::<f64>().unwrap(),
            y: line[38..46].trim().parse::<f64>().unwrap(),
            z: line[46..54].trim().parse::<f64>().unwrap(),
            occupancy: line[54..60].trim().parse::<f64>().unwrap(),
            temp_factor: line[60..66].trim().parse::<f64>().unwrap(),
            element: line[76..78].trim().to_string(),
            charge: line[78..80].trim().to_string(),
        };
        println!("{}", atom.to_string());
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
