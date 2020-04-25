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
