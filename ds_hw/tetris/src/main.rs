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
//Either absolute coordinates if it's part of the board
//or relative coordinates if it's in a Piece
struct Cell {
    x:  usize,
    y:  usize,
    col:Color,
}
impl Cell {
    fn blank() -> Cell {
        Cell {
            x:  0,
            y:  0,
            col:Color::rand(),
        }
    }
}


struct Piece {
    x: usize,
    y: usize,
  //shape: Shape,
    cells: [Cell; 4],
}

impl Piece {
    fn rotate_counterclockwise(&mut self) {
        //self.theta = (self.theta + std::f64::consts::PI / 2.0) 
        //    % (std::f64::consts::PI*2.0);
        //(equal)
        //45deg: will be {-1,0,1}
        let cos = (std::f64::consts::PI/2.0).cos() as i32;  
        let sin = (std::f64::consts::PI/2.0).sin() as i32;
        //let trig = cos;

        let points : Vec<(i32,i32)> = {
            let points = self.cells.iter()
                .map(|  p  | (p.x as i32, p.y as i32))
                .map(|(x,y)| (x*cos-y*sin, x*sin+y*cos));
                //.map(|(x,y)| ((x-y)*trig, (x+y)*trig));
            //let mins = points.into_iter()
            let points: Vec<(i32,i32)> = points.collect();
            let (min_x, min_y) = points.iter()
                .fold(
                    (std::i32::MAX, std::i32::MAX),
                    |(xm,ym), &(x,y)| 
                        (std::cmp::min(xm,x), std::cmp::min(ym,y)));
            points.into_iter()
                .map(|(x,y)| (x-min_x, y-min_y))
                .collect()
                //.map(|(x,y)| (x-min_x, y-min_y))
        };
        for (i,(x,y)) in points.into_iter().enumerate() {
            self.cells[i].x = x as usize;
            self.cells[i].y = y as usize;
        }
        
    }
    fn new(x:usize, y:usize, s:Shape, c:Color) -> Piece {
        //(x,y) is top-left corner
        //let mut cells = [Cell::blank(); 4];
        let cells = match s {
            Shape::I => [Cell{x:0,y:0,col:c}, 
                            Cell{x:0,y:1,col:c}, 
                            Cell{x:0,y:2,col:c},
                            Cell{x:0,y:3,col:c}],
            Shape::O => [Cell{x:0,y:0,col:c}, 
                            Cell{x:1,y:0,col:c}, 
                            Cell{x:1,y:1,col:c},
                            Cell{x:0,y:1,col:c}],
            Shape::T => [Cell{x:0,y:0,col:c}, 
                            Cell{x:1,y:0,col:c}, 
                            Cell{x:2,y:0,col:c},
                            Cell{x:1,y:1,col:c}],
            Shape::Z => [Cell{x:0,y:0,col:c}, 
                            Cell{x:1,y:0,col:c}, 
                            Cell{x:1,y:1,col:c},
                            Cell{x:2,y:1,col:c}],
            Shape::S => [Cell{x:0,y:1,col:c}, 
                            Cell{x:1,y:1,col:c}, 
                            Cell{x:1,y:0,col:c},
                            Cell{x:2,y:0,col:c}],
            Shape::L => [Cell{x:0,y:0,col:c}, 
                            Cell{x:0,y:1,col:c}, 
                            Cell{x:0,y:2,col:c},
                            Cell{x:1,y:2,col:c}],
            Shape::J => [Cell{x:0,y:2,col:c}, 
                            Cell{x:1,y:2,col:c}, 
                            Cell{x:1,y:1,col:c},
                            Cell{x:1,y:0,col:c}],
        };
        Piece {
            x:      x,
            y:      y,
          //shape:  s,
            cells:  cells,
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
        let mut s = match self.col {
            Color::Red      => "\x1B[31m",
            Color::Orange   => "\x1B[33m",
            Color::Yellow   => "\x1B[93m",  //Bright Orange
            Color::Green    => "\x1B[32m",
            Color::Blue     => "\x1B[34m",
            Color::Indigo   => "\x1B[36m",  //Cyan
            Color::Violet   => "\x1B[35m",  //Magenta
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
                let opts = 8;   // ROYGBIV + 1
                b.table[y][x] = match rand::random::<u8>() % opts {
                    0 => Some(Cell{x:x, y:y, col:Red}),
                    1 => Some(Cell{x:x, y:y, col:Orange}),
                    2 => Some(Cell{x:x, y:y, col:Yellow}),
                    3 => Some(Cell{x:x, y:y, col:Green}),
                    4 => Some(Cell{x:x, y:y, col:Blue}),
                    5 => Some(Cell{x:x, y:y, col:Indigo}),
                    6 => Some(Cell{x:x, y:y, col:Violet}),
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
            |&Cell{x,y,..}| 
            0 <= p.x+x && p.x+x < WIDTH  &&
            0 <= p.y+y && p.y+y < HEIGHT &&
            self.get(x,y).is_none()) 
    }
    fn incorporate(&mut self, p:Piece) {
        for &c in p.cells.iter() {
            self.table[p.y+c.y][p.x+c.x] = Some(c);
        }
    }
}

fn main() {
    //let b = Board::random();
    //println!("{}", b);
    let mut b = Board::new();
    let mut p = Piece::new(15,15,Shape::rand(),Color::rand());
    p.rotate_counterclockwise();
    println!("{:?}", p.cells);
    b.incorporate(p);
    println!("{}", b);
    
    //let mut b = Board::new(); 
    //let mut pieces = 0;
    //for _ in 0..10 {
    //    let (x,y) = (rand::random::<usize>() % WIDTH, rand::random::<usize>() % HEIGHT);
    //    let p = Piece::new(x, y, Shape::rand(), Color::rand());
    //    if b.compatible(&p) {
    //        pieces += 1;
    //        b.incorporate(p)
    //    }
    //}
    //println!("{}", b);
    //println!("Pieces: {}", pieces);


    
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
