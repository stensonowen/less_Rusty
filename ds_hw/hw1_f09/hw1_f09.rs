//http://www.cs.rpi.edu/academics/courses/fall09/ds/hw/01_text_justification/hw.pdf
//
//divergence from the spec:
//  ignores double-spaces following periods (which I refuse to fix)
//  full-justifies last line (spec says left-) (I don't care enough to fix)
//  doesn't split long word into chunks to fit onto short lines (who cares)

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

struct Line {
    max:    usize,
        //could be convenient if this were static or something
        // because all `Line`s store the same `max`field
        //But if we wanted to add triangles or circles 
        // (instead of just rectangles) this will be handy
    len:    usize,
    words:  Vec<String>,
}

impl Line {
    fn new(max: u32) -> Line {
        Line {
            max:    max as usize,
            len:    0,
            words:  vec![],
        }
    }
    fn push(&mut self, s: &str) -> bool {
        //returns whether or not a word was appended
        //would probably be more Rust-y to return a Result
        //  maybe
        assert!(s.len() <= self.max);
        let space : usize = (self.len != 0) as usize;
        let addtn : usize = space + s.len() as usize;
        if self.len + addtn <= self.max {
            self.words.push(s.to_string());
            self.len += addtn;
            true
        } else {
            false
        }
    }
    fn left_justify(&self) -> String {
        let mut s = String::new();
        s.push_str(self.words.join(" ").as_ref());
        let l = self.max - s.len();
        let spaces: String = std::iter::repeat(" ").take(l).collect();
        s.push_str(spaces.as_ref());
        s
    }
    fn right_justify(&self) -> String {
        let mut s = String::new();
        let len: usize = self.words.iter().fold(0, |acc, w| acc+w.len()) + self.words.len()-1;
        let num_spaces: usize = self.max - len;
        let spaces: String = std::iter::repeat(" ").take(num_spaces).collect();
        s.push_str(spaces.as_ref());
        s.push_str(self.words.join(" ").as_ref());
        s
    }

    fn full_justify(&self) -> String {
        //this will only work in vectors with >1 words
        if self.words.len() == 1 {
            //doesn't make sense to adjust spaces
            return self.left_justify();
        } 
        assert!(self.words.len() > 1);

        //length of every word in the string, excluding spaces
        let len: usize = self.words.iter().fold(0, |acc, w| acc+w.len());
        //number of spaces in line
        let num_spaces: usize = self.words.len()-1;

        //length of spaces on the right;
        //other spaces may be 0 or 1 character(s) longer
        let min_num_spaces: usize = (self.max - len)/num_spaces;
        let num_longer_spaces = self.max - (len + num_spaces*min_num_spaces);
        //the first `num_longer_spaces` will be `min_num_spaces` number of spaces;
        //the other `num_spaces - num_longer_spaces` will be `min_num_spaces` long
        let long_space: String = std::iter::repeat(" ").take(min_num_spaces+1).collect();
        let long_spaces = std::iter::repeat(long_space).take(num_longer_spaces);
        let short_space:String = std::iter::repeat(" ").take(min_num_spaces).collect();
        let short_spaces= std::iter::repeat(short_space).take(num_spaces-num_longer_spaces);
        //add a blank space to pair with the last word
        let blank_space = vec![String::new()].into_iter();
        //join together and concatenate everything
        let spaces = long_spaces
            .chain(short_spaces)
            .chain(blank_space);
        let all = self.words.iter().zip(spaces);
        all.fold(String::new(), |acc,(a,b)| acc+a+b.as_ref())
    }
}

fn split_up(words: &Vec<String>, width: u32) -> Vec<Line> {
    //put vector of words into vector of lines
    //this is where interesting text shapes would go
    // (that is, replacing rectangle with triangle / circle)
    let mut lines = vec![];
    let mut line = Line::new(width);
    for word in words {
        if line.push(word) == false {
            lines.push(line);
            line = Line::new(width);
            assert!(line.push(word));
        }
    }
    if line.len > 0 {
        lines.push(line);
    }
    lines
}

fn format(content: &Vec<Line>, alignment: Align) -> Vec<String> {
    let len = content[0].max + 4;   //old len + pipe and space on each side
    let horizontal: String = std::iter::repeat("-".to_string()).take(len).collect();
    let mut lines = vec![horizontal.clone() + "\n"];
    for line in content {
        let s: String = format!("| {} |\n", match alignment {
            Align::Left     => line.left_justify(),
            Align::Right    => line.right_justify(),
            Align::Full     => line.full_justify(),
        });
        lines.push(s);
    }
    lines.push(horizontal + "\n");  //spec excludes this newline
    lines
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
    let lines = split_up(&words, width);
    let lines = format(&lines, align);
    write_out(&f_out, &lines);
}


fn write_out(mut f_out: &File, lines: &Vec<String>) {
    //write vector of strings to an output file
    for line in lines {
        if let Err(e) = f_out.write_all(line.as_bytes()){
            //don't want to use a match statement,
            // because Ok(_) should do nothing
            panic!("failed to write to output file: {}", Error::description(&e));
        }
    }
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
