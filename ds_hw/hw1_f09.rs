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

    //file io
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
    let boxed_lines = boxify(formatted_lines, width);
    write_out(&f_out, &boxed_lines);

}

fn boxify(lines: Vec<String>, n: usize) -> Vec<String> {
    //was initially a part of join() because it avoids all the O(n) insert()s, 
    //but I guess organization is more important than performance in this context
    let mut lines = lines;
    let mut row = String::new();
    for _ in 0..n+4 {
        row.push('-');
    }
    row.push('\n');
    for mut line in &mut lines {
        line.insert(0, '|');
        line.insert(1, ' ');
        if line.len() < n+2 {
            let tmp = n+2-line.len();
            append_spaces(&mut line, tmp);
        }
        line.insert(n+2, '|');
        line.insert(n+2, ' ');
        line.push('\n');
    }
    lines.insert(0, row.clone());
    lines.push(row);
    lines
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

fn strip(s: &mut String) {
    //strip trailing whitespace
}

fn append_spaces(s: &mut String, n: usize) {
    for _ in 0..n {
        s.push(' ');
    }
}

fn join(lines: &Vec<Vec<&String>>, align: Align, width: usize) -> Vec<String> {
    //takes a vector of vectors (lines of words), and inserts spaces according to alignment
    let mut output = Vec::<String>::new();
    let mut l_buff;
    let mut c_buff;

    for line in lines {
        let mut buffer = String::with_capacity(width+3);
        let line_len = line.iter().fold(0, |len, word| len+word.len()) + (line.len()-1);
        assert!(line_len <= width); //can be broken if a word is longer than the width
        //will screw up the box: should panic instead
        
        //these ended up harder to follow than I thought
        //maybe they're not great candidates for matches
        l_buff = match align {
            Align::Left | Align::Full => 0,
            Align::Right => match width>line_len {
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

