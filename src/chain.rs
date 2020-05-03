pub struct Chain {
    pub chain_id: String,
}

impl Chain {
    pub fn to_string(&self) -> String {
        return format!("Chain: {chain_id}", chain_id = self.chain_id,);
    }
}
