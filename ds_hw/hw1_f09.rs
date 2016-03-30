//http://www.cs.rpi.edu/academics/courses/fall09/ds/hw/01_text_justification/hw.pdf

use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, Write};

enum Align { 
    Left, 
    Right, 
    Full,
}

fn format(lines: Vec<String>, align: Align) -> Vec<String> {
    Vec::<String>::new()
}

fn split_up(f_in: &File, width: u32) -> Vec<String> {
    let f_in_reader = BufReader::new(f_in);
    for line in f_in_reader.lines(){
        println!("{}", line.unwrap());
    }
    Vec::<String>::new()
}

fn write_out(mut f_out: &File, lines: Vec<String>) {
    let lorem_ipsum = "LOREM IPSUM\n".to_string();
    if let Err(e) = f_out.write_all(lorem_ipsum.as_bytes()){
        //don't want to use a match statement,
        // because Ok(_) should do nothing
        println!("failed to write to output file: {}", Error::description(&e));
    }
    
}

fn main() {
    //Arg parsing:
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 { 
        panic!("Usage: ./a.out input output width alignment"); 
    }
    let align = match args[4].as_ref(){
        "flush_left"    => Align::Left,
        "flush_right"   => Align::Right,
        "full_justify"  => Align::Full,
        _   => panic!("Alignment must be 'flush_left', 'flush_right', or 'full_justify'"), 
    };
    let width: u32 = match args[3].trim().parse(){
        Ok(n) => n,
        _   => panic!("Width must be a positive integer"),
    };

    ////file io
    let f_in = match File::open(&args[1]) {
        Err(e) => panic!("Failed to open input file {}: {}", 
                         args[1], Error::description(&e)),
        Ok(f)  => f,
    };
    let fn_out= Path::new(&args[2]);
    let mut f_out = match File::create(&fn_out){
        Err(e)  => panic!("failed to create file {}: {}", 
                          fn_out.display(), Error::description(&e)),
        Ok(f)   => f,
    };

    //puttin' in work
    let mut lines = split_up(&f_in, width);
    lines = format(lines, align);
    write_out(&f_out, lines);

}
