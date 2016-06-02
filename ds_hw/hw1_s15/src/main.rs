//http://www.cs.rpi.edu/academics/courses/spring15/csci1200/hw/01_image_processing/hw.pdf

extern crate clap;
use clap::{App, Arg, SubCommand,};
use std::fs::File;
use std::io::{BufReader, BufRead};
//use std::io::Write;
use std::error::Error;
use std::path::Path;


#[derive(Debug)]
enum State {
    Old(char),
    New(char),
}

#[derive(Debug)]
struct Board {
    width:  usize,
    height: usize,
    board:  Vec<Vec<State>>,
}

impl Board {
    fn new(f_in: &File) -> Board {
        let mut board: Vec<Vec<State>> = vec![];
        let mut char_line: Vec<State> = vec![];
        let reader = BufReader::new(f_in);
        for file_line in reader.lines() {
            for c in file_line.unwrap().chars() {
                //chars are smaller than u8s (by x2)
                char_line.push(State::Old(c));
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
    fn print(&self) {
        println!("Printing board; height={}, width={}", self.height, self.width);
        for line in &self.board {
            for c in line {
                let c = match *c {
                    State::New(c) => c,
                    State::Old(c) => c,
                };
                print!("{}", c);
            }
            println!("");
        }
    }
    fn submit(&mut self) {
        for mut line in &mut self.board {
            for mut c in line {
                *c = State::New('x');
            }
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
    board.print();
    board.submit();
    board.print();
    
    if let Some(m) = matches.subcommand_matches("replace"){
        if let (Some(new), Some(old)) = (m.value_of("new"), m.value_of("old")){
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




}
