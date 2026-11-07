use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u16>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut table: HashMap<String, u16> = HashMap::new();
        
        table.insert(String::from("SP"), 0);
        table.insert(String::from("LCL"), 1);
        table.insert(String::from("ARG"), 2);
        table.insert(String::from("THIS"), 3);
        table.insert(String::from("THAT"), 4);
        table.insert(String::from("SCREEN"), 16384);
        table.insert(String::from("KBD"), 24576);

        for i in 0..16 {
            table.insert(
                format!("R{i}"),
                i
            );
        }

        SymbolTable { table }
    }

    pub fn add_entry(&mut self, symbol: &str, address: u16) {
        self.table.insert(
            symbol.to_string(),
            address
        );
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> u16 {
        self.table.get(symbol).copied().expect("symbol is not in table")
    }
}
