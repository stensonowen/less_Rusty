//http://www.cs.rpi.edu/academics/courses/spring15/csci1200/hw/01_image_processing/hw.pdf

#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand,};
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::error::Error;
use std::path::Path;
use std::fmt;


#[derive(PartialEq)]
//Makes it easy to tell whether a cell was recently changed
//this way one call to `dilation` won't affect one cell more than once
enum Cell {
    Old(char),
    New(char),
}

//stores board metadata and table
//point (0,0) is the top-left
struct Board {
    width:  usize,
    height: usize,
    board:  Vec<Vec<Cell>>,
        //vector of horizontal lines of chars
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
        //this probably isn't necessary because <=1 action will be done.
        //But this would be useful if this code should be more extensible
        // and multiple commands can be chained together
        for line in &mut self.board {
            for mut c in line {
                if let &mut Cell::New(c_) = c {
                    *c = Cell::Old(c_);
                }
            }
        }
    }
    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize,usize)> {
        let mut points = vec![];
        if y > 0             {  points.push((x,y-1));   }   //above
        if x < self.width-1  {  points.push((x+1,y));   }   //right
        if y < self.height-1 {  points.push((x,y+1));   }   //below
        if x > 0             {  points.push((x-1,y));   }   //left
        points
    }
    fn dilate(&mut self, c: char) {
        //spread a field of `c`s outward
        //  ...    .x.
        //  .x. => xxx
        //  ...    .x.
        //for each column...
        for x in 0..self.width {
            //in each row...
            for y in 0..self.height {
                //if the cell at (x,y) is the proper character...
                if self.board[y][x] == Cell::Old(c) {
                    //for each of the bordering points (above, right, below, left)...
                    for (i,j) in self.get_neighbors(x,y) {
                        //and it is marked `Old`...
                        if let Cell::Old(c_) = self.board[j][i] {
                            //and it is not already the proper char
                            if c_ != c {
                                //then change (marked `New`)
                                self.board[j][i] = Cell::New(c);
                            }
                        }
                    }
                }
            }
        }
    }
    fn erode(&mut self, old: char, new: char) {
        //dilate the surroundings of a field (opposite of dilating the field)
        // can't just dialate background in case there are more than 2 chars
        //  .x.    ...
        //  xxx => .x.
        //  .x.    ...
        for x in 0..self.width {
            for y in 0..self.height {
                //if cell at (x,y) is an old instance of proper char:
                if self.board[y][x] == Cell::Old(old) {
                    //if it neighbor is something that's not that char:
                    for (i,j) in self.get_neighbors(x,y) {
                        if let Cell::Old(c_) = self.board[j][i] {
                            if c_ != old {
                                //then set it to that
                                self.board[y][x] = Cell::New(new);
                            }
                        }
                    }
                }
            }
        }
    }
    fn floodfill(&mut self, x: usize, y: usize, new: char) {
        //replace the field of characters containing (x,y) with the char `new`
        //base case: no neighbors are Cell::Old(old)
        //otherwise, set (x,y) to Cell::New(new) and call floodfill on neighbors
        if let Cell::Old(old) = self.board[y][x] {
            self.board[y][x] = Cell::New(new);
            for (i,j) in self.get_neighbors(x,y) {
                if let Cell::Old(c) = self.board[j][i] {
                    if c == old {
                        self.floodfill(i,j,new);
                    }
                }
            }
        }
        else { assert!(false); }
    }
    fn write(&self, mut f_out: &File) {
        //write formatted results to output file
        let mut s = String::new();
        for line in &self.board {
            for c in line {
                let c = match *c {
                    Cell::New(c) => c,
                    Cell::Old(c) => c,
                };
                s.push(c);
            }
            s.push('\n');
            //write each line; cap buffer at `self.width` bytes
            //inefficient to write a lot, but more probably irresponsible 
            // to store entire output in a single string (for large tables)
            if let Err(e) = f_out.write_all(s.as_bytes()){
                panic!("failed to write board to file; error: {}", 
                       Error::description(&e));
            }
            s.clear();
        }
    }
}

fn main() {
    //set up arg parsing
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
                    // but I don't think that can be done with `subcommand`s
                    .subcommand(SubCommand::with_name("replace")
                                .arg(Arg::with_name("old").required(true))
                                .arg(Arg::with_name("new").required(true)))
                    .subcommand(SubCommand::with_name("dilation")
                                .arg(Arg::with_name("old").required(true)))
                    .subcommand(SubCommand::with_name("erosion")
                                .arg(Arg::with_name("old").required(true))
                                .arg(Arg::with_name("new").required(true)))
                    .subcommand(SubCommand::with_name("floodfill")
                                .arg(Arg::with_name("x")  .required(true))
                                .arg(Arg::with_name("y")  .required(true))
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

    //create board
    let mut board = Board::new(&f_in);
    //println!("Start:\n{}", board);
    
    //call functions
    //afaik, clap doesn't let you perfectly implement this,
    // so my manaul checking is a bit verbose
    if let Some(m) = matches.subcommand_matches("replace"){
        if let (Some(new), Some(old)) = (m.value_of("new"), m.value_of("old")){
            //`char` doesn't implement FromStr (understandably),
            // so clap's typing macro won't help us
            assert!(new.len() == 1 && old.len() == 1);
            let new: char = new.chars().nth(0).unwrap();
            let old: char = old.chars().nth(0).unwrap();
            board.replace(old, new);
        }
        else { assert!(false); } //clap should prevent this from ever being triggered, right?
    } else if let Some(m) = matches.subcommand_matches("dilation"){
        if let Some(old) = m.value_of("old"){
            assert!(old.len() == 1);
            let old: char = old.chars().nth(0).unwrap();
            board.dilate(old);
            board.submit();
        }
        else { assert!(false); }
    } else if let Some(m) = matches.subcommand_matches("erosion"){
        if let (Some(new), Some(old)) = (m.value_of("new"), m.value_of("old")){
            assert!(new.len() == 1 && old.len() == 1);
            let new: char = new.chars().nth(0).unwrap();
            let old: char = old.chars().nth(0).unwrap();
            board.erode(old, new);
        }
        else { assert!(false); }
    } else if let Some(m) = matches.subcommand_matches("floodfill"){
        if let Some(new) = m.value_of("new") {
            assert!(new.len() == 1);
            let new: char = new.chars().nth(0).unwrap();
            let x = value_t!(m, "x", usize).unwrap();
            let y = value_t!(m, "y", usize).unwrap();
            board.floodfill(x,y,new);
        }
        else { assert!(false); }
    } else {
        println!("No command given :(");
        println!("Commands include `replace`, `dilation`, `erosion`, and `floodfill`");
        std::process::exit(1);  //exit code clap uses
    }
    
    board.write(&f_out);
    //println!("Done:\n{}", board);
}
