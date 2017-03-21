extern crate chipper;
extern crate clap;

use clap::{App, Arg};

fn main()
{

    let args = App::new("Chipper")
        .about("Runs chip-8 programs from the programs directory")
        .version("0.1.0")
        .author("Kyle Galloway")
        .arg(Arg::with_name("file")
                 .help("the program file to use; i.e. programs/<file>")
                 .index(1)
                 .required(true)
                 .short("f")
                 .long("file"))
        .get_matches();

    let program = format!("programs/{}", args.value_of("file").unwrap());

    chipper::main(program);
}
