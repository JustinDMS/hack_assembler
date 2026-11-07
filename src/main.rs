mod code;
mod parser;
mod symbol_table;

use parser::{Parser, CommandType};
use symbol_table::SymbolTable;

fn main() {
    let Some(input_path) = std::env::args().nth(1) else {
        eprintln!("Usage: hack_assembler <file.asm>");
        return;
    };
    let output_path = std::path::Path::new(&input_path).with_extension("hack");
    let mut parser = Parser::new(&input_path).expect("could not open file");
    let mut output = String::new();
    let mut symbols = SymbolTable::new();
    let mut rom_address: u16 = 0;

    while parser.has_more_commands() {
        parser.advance();
        match parser.command_type() {
            CommandType::A | CommandType::C => rom_address += 1,
            CommandType::L => symbols.add_entry(parser.symbol(), rom_address),
        }
    }

    parser.reset();
    let mut next_variable: u16 = 16;

    while parser.has_more_commands() {
        parser.advance();
        match parser.command_type() {
            CommandType::A => {
                let symbol = parser.symbol();
                let value = if symbol.starts_with(|c: char| c.is_ascii_digit()) {
                    symbol.parse::<u16>().expect("A instruction value is not a number")
                } else if symbols.contains(symbol) {
                    symbols.get_address(symbol)
                } else {
                    let address = next_variable;
                    symbols.add_entry(symbol, address);
                    next_variable += 1;
                    address
                };
                
                output.push_str( &format!("{:016b}\n", value) );
            },
            CommandType::L => {},
            CommandType::C => {
                let comp_bits = code::comp(parser.comp()).expect("invalid comp");
                let dest_bits = code::dest(parser.dest()).expect("invalid dest");
                let jump_bits = code::jump(parser.jump()).expect("invalid jump");
                output.push_str(&format!("111{comp_bits}{dest_bits}{jump_bits}\n"));
            },
        }
    }

    std::fs::write(&output_path, output).expect("could not write output file");
    println!("Wrote {}", output_path.display());
}
