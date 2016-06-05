//Tetris... in Rust
//shoutout to ncurses and ncurses-rs
//this is kind of a freebie:
// https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_4.rs
//(I installed ncurses with package `libncursesw5-dev`)

extern crate ncurses;
extern crate rand;

use std::fmt;

const WIDTH:  usize = 20;
const HEIGHT: usize = 20;

#[derive(Debug, Clone, Copy)]
enum Color { Red, Orange, Yellow, Green, Blue, Indigo, Violet, }
enum Shape { I, O, T, Z, S, L, J, }

impl Color {
    fn rand() -> Color {
        let len = 7;    //ROYGBIV
        use Color::*;
        match rand::random::<u8>() % len {
            0 => Red,
            1 => Orange,
            2 => Yellow,
            3 => Green,
            4 => Blue,
            5 => Indigo,
            _ => Violet,
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Cell {
    x: usize,
    y: usize,
    c: Color,
}

struct Piece {
  //color: Color,
    shape: Shape,
    cells: [Cell; 4],
}

//Board is an array of rows
#[derive(Debug)]
struct Board {
    table: [[Option<Cell>; WIDTH]; HEIGHT]
}

//impl fmt::Display for Line {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//write!(f, 
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = match self.c {
            Color::Red      => "\x1B[31m",
            Color::Orange   => "\x1B[33m",
            //Color::Orange   => "x",       //orange??
            Color::Yellow   => "\x1B[93m",
            Color::Green    => "\x1B[32m",
            Color::Blue     => "\x1B[34m",
            Color::Indigo   => "\x1B[36m",
            Color::Violet   => "\x1B[35m",
          //_               => "\x1B[37m",  //hwhite
        }.to_string();
        s.push_str("□");
        s.push_str("\x1B[0m");
        write!(f, "{}", s)
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.table.iter() {
            for j in i {
                //print!("{:?}, ", j);
                //print!("{}", j);
                match *j {
                    Some(c) => try!(write!(f, "{}", c)),
                    None    => try!(write!(f, " ")),
                };
            }
            try!(write!(f, "\n"));
        }
        write!(f, "")
    }
}


impl Board{
    fn new() -> Board {
        //Board([[None; WIDTH]; HEIGHT])
        Board {
            table: [[None; WIDTH]; HEIGHT]
        }
    }
    fn random() -> Board {
        let mut b = Board::new();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                use Color::*;
                let OPTS = 8;   // ROYGBIV + 1
                b.table[y][x] = match rand::random::<u8>() % OPTS {
                    0 => Some(Cell{x:x, y:y, c:Red}),
                    1 => Some(Cell{x:x, y:y, c:Orange}),
                    2 => Some(Cell{x:x, y:y, c:Yellow}),
                    3 => Some(Cell{x:x, y:y, c:Green}),
                    4 => Some(Cell{x:x, y:y, c:Blue}),
                    5 => Some(Cell{x:x, y:y, c:Indigo}),
                    6 => Some(Cell{x:x, y:y, c:Violet}),
                    _ => None,
                };
            }
        }
        b
    }
}

fn main() {
    let b = Board::random();
    println!("{}", b);


    
    ncurses::initscr();
    ncurses::raw();
    ncurses::noecho();
    ncurses::printw("hello world\n");
    loop {
        ncurses::endwin();
        break;
        match ncurses::getch() {
            97  => {
                ncurses::endwin();
                break
            }
            _   => ncurses::printw("x"),
        };
    }

}
