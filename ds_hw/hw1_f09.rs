//http://www.cs.rpi.edu/academics/courses/fall09/ds/hw/01_text_justification/hw.pdf

use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, Write};

#[derive(PartialEq)]    //==
enum Align { 
    Left, 
    Right, 
    Full,
}

fn format<'a>(mut lines: &'a Vec<String>, align: Align, width: usize) -> &'a Vec<String> {
    for line in lines {
        //line will be string of words separated by spaces
        if align == Align::Full {
            let v : Vec<_> = line.split(' ').collect();
            let num_letters = line.len() - match v.len() {
                //number of spaces should be excluded from count of current real letters:
                0|1 => 0,
                x   => x-1,
            };
            //let tmp = v.iter().fold(String::new(), |l, w| l.push_str(w));
            let mut spaces = String::new();
            for i in 0..(width-num_letters) {
                spaces.push(' ');
            }
            //let line = &v.connect(&spaces);
            //let line = "foo";
            let line = "foo".to_string();
        }
        for l in lines {
            println!(":{}", l);
        }
        let spaces = match align {
            //align = l/r justify: => # spaces before/after
            //align = c justify:   => # spaces in between
            Align::Left | Align::Right => width - line.len(),
            //Align::Cent 
           _    => 5, 
       };
    }
    &lines
}

fn split_up(f_in: &File, width: usize) -> Vec<String> {
    //prepare vector that is to hold formatted lines
    let mut lines = Vec::<String>::new();
    lines.push(String::new());
    let mut l = 0;  //current line to alter
    //prepare to read from input one line at a time
    //dealing with one line at a time is preferable to reading 
    // into a single string and then tokenizing it all
    let f_in_reader = BufReader::new(f_in);
    for line in f_in_reader.lines(){
        //split line into words and cycle through them
        let line = line.unwrap();
        let words = line.split(' ');
        for word in words {
            //not sophisticated enough to split individual words by syllable
            // no hyphenation -> undefined case: throw error
            if word.len() > width {
                panic!("Error: word <{}> in line <{}> is longer than the desired width: {} > {}",
                       word, line, word.len(), width);
            }
            //append word to formatted line if it won't put it over the width threshold
            else if lines[l].len() + word.len() < width {
                lines[l].push(' ');
                lines[l].push_str(word);
            } 
            //otherwise push it to the next line
            else {
                lines.push(word.to_string());
                l += 1;
            }
        }
    }
    /*for line in &lines {
        println!("{}", line);
    }*/
    lines
}

fn write_out(mut f_out: &File, lines: &Vec<String>) {
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
    let width: usize = match args[3].trim().parse(){
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
    let f_out = match File::create(&fn_out){
        Err(e)  => panic!("failed to create file {}: {}", 
                          fn_out.display(), Error::description(&e)),
        Ok(f)   => f,
    };

    //puttin' in work
    let mut lines = split_up(&f_in, width);
    lines = format(&lines, align, width);
    write_out(&f_out, &lines);

}
