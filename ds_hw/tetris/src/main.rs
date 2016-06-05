//Tetris... in Rust
//shoutout to ncurses and ncurses-rs
//this is kind of a freebie:
// https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_4.rs
//(I installed ncurses with package `libncursesw5-dev`)

extern crate ncurses;
extern crate term;

use std::fmt;

const WIDTH:  usize = 20;
const HEIGHT: usize = 20;

#[derive(Debug, Clone, Copy)]
enum Color { Red, Orange, Yellow, Green, Blue, Indigo, Violet, }
enum Shape { I, O, T, Z, S, L, J, }


#[derive(Debug, Clone, Copy)]
struct Cell {
    x: u32,
    y: u32,
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
        let mut t = term::stdout().unwrap();
        let col = match self.c {
            Color::Red      => term::color::RED,
            Color::Orange   => term::color::WHITE,
            Color::Yellow   => term::color::YELLOW,
            Color::Green    => term::color::GREEN,
            Color::Blue     => term::color::BLUE,
            Color::Indigo   => term::color::CYAN,
            //Color::Violet   => term::color::MAGENTA,
            _               => term::color::BRIGHT_WHITE,
        };
        t.fg(col).unwrap();
        //let res = write!(f, "{}", 42);
        write!(t, "X");
        t.reset().unwrap();
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
}

fn main() {
    let c: Cell = Cell{x:0, y:0, c:Color::Red};
    println!("{:?}", c);
    println!("{:?}", Some(c));
    let b : Option<Cell> = None;
    println!("{:?}", b);
    let a: Board = Board::new(); 
    //println!("{:?}", a);
    for i in a.table.iter() {
        for j in i {
            //print!("{:?}, ", j);
        }
        //println!("");
    }


    
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
