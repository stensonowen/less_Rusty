//http://www.cs.rpi.edu/academics/courses/spring15/csci1200/hw/01_image_processing/hw.pdf

extern crate clap;
use clap::{App, Arg, SubCommand,};
use std::fs::File;
use std::io::{BufReader, BufRead};
//use std::io::Write;
use std::error::Error;
use std::path::Path;
use std::fmt;


#[derive(PartialEq,Debug)]
//Makes it easy to tell whether a cell was recently changed
//this way one call to `dilation` won't affect one cell more than once
enum Cell {
    Old(char),
    New(char),
}

#[derive(Debug)]
//stores board metadata and table
//point (0,0) is the top-left
//point (0,9) is the top-right
struct Board {
    width:  usize,
    height: usize,
    board:  Vec<Vec<Cell>>,
}

impl fmt::Display for Board {
    //print; for debugging porpoises
    //using Display instead of Debug because Debug displays Enum wrapper
    //need Display for Debug purposes
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Board: ({}x{})\n\t", self.width, self.height));
        for line in &self.board {
            for c in line {
                let c = match *c {
                    Cell::New(c) => c,
                    Cell::Old(c) => c,
                };
                try!(write!(f, "{}", c));
            }
            try!(write!(f, "\n\t"));
        }
        write!(f, "")
    }
}

#[allow(dead_code)]
impl Board {
    fn new(f_in: &File) -> Board {
        let mut board: Vec<Vec<Cell>> = vec![];
        let mut char_line: Vec<Cell> = vec![];
        let reader = BufReader::new(f_in);
        for file_line in reader.lines() {
            for c in file_line.unwrap().chars() {
                //chars are smaller than u8s (by x2)
                char_line.push(Cell::Old(c));
            }
            board.push(char_line);
            char_line = vec![];
        }
        Board{ 
            width:  board[0].len(),
            height: board.len(),
            board:  board 
        }
    }
    fn replace(&mut self, old: char, new: char) {
        //replace every instance of `old` with `new`
        //doesn't need to use Cell::New
        for line in &mut self.board {
            for mut c in line {
                if *c == Cell::Old(old) {
                    *c = Cell::Old(new);
                }
            }
        }
    }
    fn submit(&mut self) {
        //mark each cell as 'Old'
        for line in &mut self.board {
            for mut c in line {
                if let &mut Cell::New(c_) = c {
                    *c = Cell::Old(c_);
                }
            }
        }
    }
    fn modify(&mut self, x: usize, y: usize, c: char){
        //sets the point at (x,y) to character c
        // iff it is `Old` (i.e. unchanged from the last round)
        //more Rust-y to modify the iterator through characters 
        // rather than by using lots of indexing?
        assert!(x < self.width && y < self.height);
        if let Cell::Old(_) = self.board[y][x] {
            self.board[y][x] = Cell::New(c);
        }
    }

}



#[allow(unused_variables)]
fn main() {
    let matches = App::new("hw1_s15")
                    //necessary positional args:
                    .arg(Arg::with_name("input")
                         .help("file containing starting board")
                         .index(1)
                         .required(true)
                         )
                    .arg(Arg::with_name("output")
                         .help("where to write resulting board")
                         .index(2)
                         .required(true))
                    //would be nice to put these in a group (exactly one is necessary),
                    .subcommand(SubCommand::with_name("replace")
                                .arg(Arg::with_name("old").required(true))
                                .arg(Arg::with_name("new").required(true)))
                    .subcommand(SubCommand::with_name("dilation")
                                .arg(Arg::with_name("old").required(true)))
                    .subcommand(SubCommand::with_name("erosion")
                                .arg(Arg::with_name("old").required(true))
                                .arg(Arg::with_name("new").required(true)))
                    .subcommand(SubCommand::with_name("floodfill")
                                .arg(Arg::with_name("x").required(true))
                                .arg(Arg::with_name("y").required(true))
                                .arg(Arg::with_name("new").required(true)))
                    .get_matches();
    
    let input  = matches.value_of("input") .unwrap();
    let output = matches.value_of("output").unwrap();
	//file io
    let f_in = match File::open(&input) {
        Err(e) => panic!("Failed to open input file {}: {}", 
                         input, Error::description(&e)),
        Ok(f)  => f,
    };
    let fn_out= Path::new(&output);
    let f_out = match File::create(&fn_out){
        Err(e)  => panic!("failed to create file {}: {}", 
                          fn_out.display(), Error::description(&e)),
        Ok(f)   => f,
    };

    let mut board = Board::new(&f_in);
    //board.replace('X', 'Y');
    //board.submit();
    //board.modify(18,0,'0');
    println!("{}", board);
    
    if let Some(m) = matches.subcommand_matches("replace"){
        if let (Some(new), Some(old)) = (m.value_of("new"), m.value_of("old")){
            //`char` doesn't implement FromStr (understandably),
            // so clap's typing macro won't help us
            assert!(new.len() == 1 && old.len() == 1);
            let new: char = new.chars().nth(0).unwrap();
            let old: char = old.chars().nth(0).unwrap();
            board.replace(old, new);
            println!("new: '{}'; old: '{}'", new, old);
        }
        else { assert!(false); } //clap should prevent this from ever being triggered, right?
    } else if let Some(m) = matches.subcommand_matches("dilation"){
        if let Some(old) = m.value_of("old"){
        }
        else { assert!(false); }
    } else if let Some(m) = matches.subcommand_matches("erosion"){
        if let (Some(new), Some(old)) = (m.value_of("new"), m.value_of("old")){
        }
        else { assert!(false); }
    } else if let Some(m) = matches.subcommand_matches("floodfill"){
        if let (Some(new), Some(x), Some(y)) = 
            (m.value_of("new"), m.value_of("x"), m.value_of("y")){
        }
        else { assert!(false); }
    } else {
        println!("No command given :(");
        println!("Commands include `replace`, `dilation`, `erosion`, and `floodfill`");
        std::process::exit(1);  //exit code clap uses
    }



    println!("{}", board);

}
