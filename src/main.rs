#[macro_use]
extern crate nom;

use std::fs;

named!(
    parse_bytecode_stream,
    tag!([0xfa, 0xfa])
);

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let path = &args[1];
    let bytecode = fs::read(path)?;
    println!("{:?}", parse_bytecode_stream(&bytecode));
    Ok(())
}
