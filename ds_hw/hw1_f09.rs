//http://www.cs.rpi.edu/academics/courses/fall09/ds/hw/01_text_justification/hw.pdf

use std::env;
//use std::io::File;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

enum Align { 
    Left, 
    Right, 
    Full,
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
    let fn_in = &args[1];
    let fn_out= &args[2];
    let width: u32 = match args[3].trim().parse(){
        Ok(n) => n,
        _   => panic!("Width must be a positive integer"),
    };

    ////file io
    let f_in = match File::open(fn_in) {
        Err(e) => panic!("Failed to open input file {}: {}", 
                         fn_in, Error::description(&e)),
        Ok(f)  => f,
    };
    let f_in_reader = BufReader::new(f_in);
    for line in f_in_reader.lines(){
        println!("{}", line.unwrap());
    }

    let f_out= Path::new(fn_out);
    let mut f_out_ = match File::create(&f_out){
        Err(e)  => panic!("failed to create file {}: {}", 
                          f_out.display(), Error::description(&e)),
        Ok(f)   => f,
    };
    let lorem_ipsum = "LOREM IPSUM";
    match f_out_.write_all(lorem_ipsum.as_bytes()) {
        Err(e)  => panic!("couldn't write to {}: {}",
                          f_out.display(), Error::description(&e)),
        Ok(_)   => println!("success"),
    }

    //let f_out = match File::open(&f_out) {
    //    Err(e)  => panic!("Failed to open output file {}: {}", 
    //                      f_out.display(), Error::description(&e)),
    //    Ok(f)   => f,
    //};

}
