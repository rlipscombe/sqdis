#[macro_use]
extern crate nom;

use nom::{le_u32, le_u64, le_u8};
use std::fs;

named!(
    parse_bytecode_stream<Closure>,
    do_parse!(tag!([0xfa, 0xfa]) >> closure: parse_closure_stream >> (closure))
);

#[derive(Debug)]
struct Closure {
    fun: FunctionProto,
}

named!(
    parse_closure_stream<Closure>,
    do_parse!(
        tag!("RIQS")
            >> _sizeof_char: le_u32
            >> _sizeof_int: le_u32
            >> _sizeof_float: le_u32
            >> fun: parse_function_proto
            >> tag!("LIAT")
            >> (Closure { fun })
    )
);

#[derive(Debug)]
struct FunctionProto {
    literals: Vec<String>,
    instructions: Vec<Instruction>,
}

named!(
    parse_function_proto<FunctionProto>,
    do_parse!(
        tag!("TRAP")
            >> _source_name: parse_object
            >> _function_name: parse_object
            >> tag!("TRAP")
            >> nliterals: le_u64
            >> nparameters: le_u64
            >> noutervalues: le_u64
            >> nlocalvarinfos: le_u64
            >> nlineinfos: le_u64
            >> ndefaultparams: le_u64
            >> ninstructions: le_u64
            >> nfunctions: le_u64
            >> tag!("TRAP")
            >> literals: count!(parse_literal, nliterals as usize)
            >> tag!("TRAP")
            >> count!(parse_parameter, nparameters as usize)
            >> tag!("TRAP")
            >> count!(parse_outer, noutervalues as usize)
            >> tag!("TRAP")
            >> count!(parse_local, nlocalvarinfos as usize)
            >> tag!("TRAP")
            >> count!(parse_lineinfo, nlineinfos as usize)
            >> tag!("TRAP")
            >> count!(parse_defaultparam, ndefaultparams as usize)
            >> tag!("TRAP")
            >> instructions: count!(parse_instruction, ninstructions as usize)
            >> tag!("TRAP")
            >> count!(parse_function_proto, nfunctions as usize)
            >> stack_size: le_u64
            >> is_generator: le_u8
            >> var_params: le_u64
            >> (FunctionProto {
                literals,
                instructions
            })
    )
);

named!(parse_literal<String>, do_parse!(s: parse_object >> (s)));
named!(parse_parameter<()>, do_parse!(parse_object >> (())));
named!(parse_outer<()>, do_parse!((())));

named!(
    parse_local<()>,
    do_parse!(parse_object >> pos: le_u64 >> start_op: le_u64 >> end_op: le_u64 >> (()))
);

named!(
    parse_lineinfo<()>,
    do_parse!(line: le_u64 >> op: le_u64 >> (()))
);

named!(parse_defaultparam<()>, do_parse!((())));

#[derive(Debug)]
struct Instruction {
    op: u8,
    arg0: u8,
    arg1: u32,
    arg2: u8,
    arg3: u8,
}

named!(
    parse_instruction<Instruction>,
    do_parse!(
        arg1: le_u32
            >> op: le_u8
            >> arg0: le_u8
            >> arg2: le_u8
            >> arg3: le_u8
            >> (Instruction {
                op,
                arg0,
                arg1,
                arg2,
                arg3
            })
    )
);

named!(
    parse_object<String>,
    do_parse!(
        s: switch!(le_u32,
        0x08000010 => call!(parse_string_object))
            >> (s)
    )
);

named!(
    parse_string_object<String>,
    do_parse!(
        length: le_u64 >> value: take!(length) >> (std::str::from_utf8(value).unwrap().to_string())
    )
);

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let path = &args[1];
    let bytecode = fs::read(path)?;
    println!("{:#?}", parse_bytecode_stream(&bytecode));
    Ok(())
}
