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

fn write_out(mut f_out: &File, lines: &Vec<String>) {
    for line in lines {
        if let Err(e) = f_out.write_all(line.as_bytes()){
            //don't want to use a match statement,
            // because Ok(_) should do nothing
            panic!("failed to write to output file: {}", Error::description(&e));
        }
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

    //and here... we... go
    let words = tokenize(&f_in);
    let lines = split(&words, width);
    let formatted_lines = join(&lines, align, width);
    write_out(&f_out, &formatted_lines);

}

fn append_spaces(s: &mut String, n: usize) {
    for _ in 0..n {
        s.push(' ');
    }
}
fn num_spaces(n: usize) -> String {
    let mut s = String::new();
    for _ in 1..n {
        s.push(' ');
    }
    s
}

fn join(lines: &Vec<Vec<&String>>, align: Align, width: usize) -> Vec<String> {
    //takes a vector of vectors (lines of words), and inserts spaces according to alignment
    let mut output = Vec::<String>::new();
    let mut l_buff;
    let mut c_buff;

    for line in lines {
        let mut buffer = String::new();
        let line_len = line.iter().fold(0, |len, word| len+word.len()) + (line.len()-1);
        //assert!(line_len <= width); //can be broken if a word is longer than the width
        
        //these ended up harder to follow than I thought
        //maybe they're not great candidates for matches
        l_buff = match align {
            Align::Left | Align::Full => 0,
            Align::Right => match (width>line_len) {
                true => width-line_len,
                false=> 0,
            },
        };
        let num_spaces = line.len()-1;
        c_buff = match align {
            Align::Left | Align::Right => 1,
            Align::Full => match num_spaces {
                0 => 0,
                n => (width-line_len+n)/n,
            },
        };

        append_spaces(&mut buffer, l_buff);
        for word in line {
            buffer.push_str(word);
            append_spaces(&mut buffer, c_buff);
        }
        buffer.push('\n');
        output.push(buffer);
    }

    output
}

fn tokenize(f_in: &File) -> Vec<String>{
    //reads an input file and returns a vector of all of its words
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

fn split(words: &Vec<String>, width: usize) -> Vec<Vec<&String>>{
    //Input: a vector of words and a desired line length
    //Output: a vector of lines, each one a vector of words, which meet the line length
    let mut lines = Vec::new();
    lines.push(Vec::new());
    let mut line_len = 0; 
    let mut line_cnt = 1;
    for word in words{
        if line_len + word.len() >= width {
            //must start adding words to a new line
            line_len = 0;
            lines.push(Vec::new());
            line_cnt += 1;
        }
        lines[line_cnt-1].push(word);
        line_len += word.len() + match line_len {
            //add 1 for a new space, unless this word is the first
            0 => 0,
            _ => 1,
        };
    }
    lines
}

