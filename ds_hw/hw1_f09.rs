//http://www.cs.rpi.edu/academics/courses/fall09/ds/hw/01_text_justification/hw.pdf

use std::env;
//use std::io::File;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

enum Align { 
    Left, 
    Right, 
    Full,
}

fn main() {
    //Arg parsing:
    let args: Vec<String> = env::args().collect();
    //if env::args().len() != 5 { 
    if args.len() != 5 { 
        panic!("Usage: ./a.out input output width alignment"); 
    }
    //let align = match env::args().nth(4){
    let align = match args[4].as_ref(){
        "flush_left" => Align::Left,
        //Some(ref x) if x == "flush_left"    => Align::Left,
        //Some(ref x) if x == "flush_right"   => Align::Right,
        //Some(ref x) if x == "full_justify"  => Align::Full,
        _   => panic!("Alignment must be 'flush_left', 'flush_right', or 'full_justify'"), 
    };
    //let fn_in = env::args().nth(1);
    //let fn_out= env::args().nth(2);
    let fn_in = &args[1];
    let fn_out= &args[2];
    //let width: u32 = match env::args().nth(3).map(|a| a.parse()) {
    //    Some(Ok(n)) => n,
    //    _   => panic!("Width must be an unsigned integer"),
    //};
    let width: u32 = match args[3].trim().parse(){
    //let width: u32 = match args[3].parse().unwrap(){
        Ok(n) => n,
        _   => panic!("SDF"),
    };
    println!("Width = {}", width);
    //
    ////file io
    //let path = Path::new("tmp.txt");
    //let mut f_in = match File::open(&path) {
    //    Err(e) => panic!("failed to open {}: {}", path.display(), Error::description(&e)),
    //    Ok(f)  => f,
    //};
    //let tmp : String = fn_in;

}
