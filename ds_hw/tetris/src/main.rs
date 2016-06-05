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
impl Shape {
    fn rand() -> Shape {
        let len = 7;
        use Shape::*;
        match rand::random::<u8>() % len {
            0 => I,
            1 => O,
            2 => T,
            3 => Z,
            4 => S,
            5 => L,
            _ => J,
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Cell {
    x: usize,
    y: usize,
    c: Color,
}
impl Cell {
    fn blank() -> Cell {
        Cell {
            x: 0,
            y: 0,
            c: Color::rand(),
        }
    }
}


struct Piece {
  //color: Color,
    shape: Shape,
    cells: [Cell; 4],
}

impl Piece {
    fn new(x:usize, y:usize, s:Shape, c:Color) -> Piece {
        //(x,y) is top-left corner
        let mut cells: [Cell; 4];
        //let mut cells = [Cell::blank(); 4];
        let cells = match s {
            Shape::I => [Cell{x:x,y:y,c:c}, 
                            Cell{x:x,y:y+1,c:c}, 
                            Cell{x:x,y:y+2,c:c},
                            Cell{x:x,y:y+3,c:c}],
            Shape::O => [Cell{x:x,y:y,c:c}, 
                            Cell{x:x+1,y:y,c:c}, 
                            Cell{x:x+1,y:y+1,c:c},
                            Cell{x:x,y:y+1,c:c}],
            Shape::T => [Cell{x:x,y:y,c:c}, 
                            Cell{x:x+1,y:y,c:c}, 
                            Cell{x:x+2,y:y,c:c},
                            Cell{x:x+1,y:y+1,c:c}],
            Shape::Z => [Cell{x:x,y:y,c:c}, 
                            Cell{x:x+1,y:y,c:c}, 
                            Cell{x:x+1,y:y+1,c:c},
                            Cell{x:x+2,y:y+1,c:c}],
            Shape::S => [Cell{x:x,y:y+1,c:c}, 
                            Cell{x:x+1,y:y+1,c:c}, 
                            Cell{x:x+1,y:y,c:c},
                            Cell{x:x+2,y:y,c:c}],
            Shape::L => [Cell{x:x,y:y,c:c}, 
                            Cell{x:x,y:y+1,c:c}, 
                            Cell{x:x,y:y+2,c:c},
                            Cell{x:x+1,y:y+2,c:c}],
            Shape::J => [Cell{x:x,y:y+2,c:c}, 
                            Cell{x:x+1,y:y+2,c:c}, 
                            Cell{x:x+1,y:y+1,c:c},
                            Cell{x:x+1,y:y,c:c}],
        };
        Piece {
            shape: s,
            cells: cells,
        }
    }
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
          //Color::Orange   => "x",         //orange??
            Color::Yellow   => "\x1B[93m",
            Color::Green    => "\x1B[32m",
            Color::Blue     => "\x1B[34m",
            Color::Indigo   => "\x1B[36m",
            Color::Violet   => "\x1B[35m",
          //_               => "\x1B[37m",  //hwhite
        }.to_string();
        //s.push_str("■");
        s.push_str("⠀");
        s.push_str("\x1B[0m");
        write!(f, "{}", s)
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "\n\t"));
        for i in self.table.iter() {
            for j in i {
                match *j {
                    Some(c) => try!(write!(f, "{}", c)),
                    None    => try!(write!(f, " ")),
                };
            }
            try!(write!(f, "\n\t"));
        }
        writeln!(f, "")
    }
}


impl Board{
    fn new() -> Board {
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
    fn get(&self, x:usize, y:usize) -> Option<Cell> {
        self.table[y][x]
    }
    fn compatible(&self, p: &Piece) -> bool {
        p.cells.iter().all(
            |&Cell{x:x,y:y,..}| 
            0 <= x && x < WIDTH  &&
            0 <= y && y < HEIGHT &&
            self.get(x,y).is_none()) 
    }
    fn incorporate(&mut self, p:Piece) {
        for &c in p.cells.iter() {
            self.table[c.y][c.x] = Some(c);
        }
    }
}

fn main() {
    //let b = Board::random();
    //println!("{}", b);
    let mut b = Board::new(); 
    let mut pieces = 0;
    for _ in 0..100 {
        let (x,y) = (rand::random::<usize>() % WIDTH, rand::random::<usize>() % HEIGHT);
        let p = Piece::new(x, y, Shape::rand(), Color::rand());
        if b.compatible(&p) {
            pieces += 1;
            b.incorporate(p)
        }
    }
    println!("{}", b);
    println!("Pieces: {}", pieces);


    
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
