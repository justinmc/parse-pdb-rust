pub struct Atom {
    pub serial: i32,
    pub name: String,
    pub alt_loc: String,
    pub res_name: String,
    pub chain_id: String,
    pub res_seq: i32,
    pub i_code: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub occupancy: f64,
    pub temp_factor: f64,
    pub element: String,
    pub charge: String,
}

impl Atom {
    pub fn from_string(pdb_line: String) -> Atom {
        return Atom {
            serial: pdb_line[6..11].trim().parse::<i32>().unwrap(),
            name: pdb_line[12..16].trim().to_string(),
            alt_loc: pdb_line[16..17].trim().to_string(),
            res_name: pdb_line[17..20].trim().to_string(),
            chain_id: pdb_line[21..22].trim().to_string(),
            res_seq: pdb_line[22..26].trim().parse::<i32>().unwrap(),
            i_code: pdb_line[26..27].trim().to_string(),
            x: pdb_line[30..38].trim().parse::<f64>().unwrap(),
            y: pdb_line[38..46].trim().parse::<f64>().unwrap(),
            z: pdb_line[46..54].trim().parse::<f64>().unwrap(),
            occupancy: pdb_line[54..60].trim().parse::<f64>().unwrap(),
            temp_factor: pdb_line[60..66].trim().parse::<f64>().unwrap(),
            element: pdb_line[76..78].trim().to_string(),
            charge: pdb_line[78..80].trim().to_string(),
        };
    }

    pub fn to_string(&self) -> String {
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
