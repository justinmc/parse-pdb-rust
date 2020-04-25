pub struct Residue {
    pub ser_num: i32,
    pub chain_id: String,
    pub num_res: i32,
    pub res_name: String,
}

impl Residue {
    pub fn parse_seq_res_entry(pdb_line: String) -> Vec<Residue> {
        let ser_num = pdb_line[7..10].trim().parse::<i32>().unwrap();
        let chain_id = pdb_line[11..12].trim().to_string();
        let num_res = pdb_line[13..17].trim().parse::<i32>().unwrap();
        let res_names_string = pdb_line[19..70].trim().to_string();

        let mut residues: Vec<Residue> = Vec::new();
        for res_name in Residue::parse_res_names(res_names_string) {
            residues.push(Residue {
                ser_num: ser_num,
                chain_id: chain_id.clone(),
                num_res: num_res,
                res_name: res_name,
            });
        }

        return residues;
    }

    pub fn to_string(&self) -> String {
        return format!(
            "Residue: {res_name}, {ser_num}, {chain_id}, {num_res}",
            res_name = self.res_name,
            ser_num = self.ser_num,
            chain_id = self.chain_id,
            num_res = self.num_res,
        );
    }

    fn parse_res_names(res_names_string: String) -> Vec<String> {
        let mut res_names: Vec<String> = Vec::new();
        for res_name in res_names_string.split_whitespace() {
            res_names.push(res_name.to_string());
        }
        return res_names;
    }
}
