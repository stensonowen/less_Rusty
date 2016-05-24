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

fn write_out(mut f_out: &File, lines: &Vec<&str>) {
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

    let words = tokenize(&f_in);
    let lines = split(&words, width);
    for line in &lines{
        for word in line{
            print!("{}, ", word);
        }
        println!("");
    }
    println!("{}", lines.len());
}

fn format(lines: &Vec<Vec<&String>>, align: Align, width: usize) -> Vec<String> {
    let mut output = Vec::<String>::new();
    output.push(String::new());
    for line in lines {
        //len = length of all the words in the line (excluding spaces)
        let len = line.iter().fold(0, |acc, &l| acc+l.len());
        let num_spaces = match line.len(){
            0|1 => 1,
            l   => l-1,
        };
        let delim_len = match align {
            Align::Left | Align::Right => 1,
            Align::Full => (width-len)/num_spaces,
        };
        let l_buffer = match align {
            Align::Right => width-(len+num_spaces),
            _ => 0,
        };
        let r_buffer = width - (len + num_spaces*delim_len);
        //let new_line = std::iter::repeat(' ').take(l_buffer).collect::<String>()
        //             + line.collect(std::iter::repeat(' ').take(delim_len).collect::<String>())
        //             + std::iter::repeat(' ').take(r_buffer).collect::<String>();
        let delim = std::iter::repeat(' ').take(delim_len).collect::<String>();
        let mut result = String::new();

        result.push_str(std::iter::repeat(' ').take(l_buffer).collect::<String>().as_ref());

        let mut tmp = String::new();
        for w in line{  tmp.push_str(&delim); tmp.push_str(w);  }

        let r = std::iter::repeat(' ').take(r_buffer).collect::<String>();

        //line.as_slice().join("F");
       // [line].to_vec().join(&&String::from("x"));
        let mut x = line.clone();
        x.join("F");
        

        output.push(result);
    }
    output

}

fn tokenize(f_in: &File) -> Vec<String>{
    let mut words = Vec::new();
    let reader = BufReader::new(f_in);
    for line in reader.lines(){
        for word in line.unwrap().split(' '){
            //creates copy of each word
            //could be done better using lifetimes?
            words.push(word.to_string());
        }
    }
    words
}

//fn split(words: &Vec<String>, width: usize) -> Vec<Vec<String>>{
fn split(words: &Vec<String>, width: usize) -> Vec<Vec<&String>>{
    let mut lines = Vec::new();
    lines.push(Vec::new());
    let mut line_len = 0;
    let mut line_cnt = 1;
    for word in words{
        //lines[0].push(word.to_string());
        if line_len + word.len() <= width {
            line_len += word.len() + 1;
        } else {
            line_len = 0;
            lines.push(Vec::new());
            line_cnt += 1;
        }
        lines[line_cnt-1].push(word);
    }
    lines
}

