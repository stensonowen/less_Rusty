//Tetris... in Rust
//shoutout to ncurses and ncurses-rs
//this is kind of a freebie:
// https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_4.rs
//(I installed ncurses with package `libncursesw5-dev`)

extern crate ncurses;

const WIDTH:  usize = 20;
const HEIGHT: usize = 20;

struct Cell {
    x: u32,
    y: u32,
    c: Color,
}

enum Color { Red, Orange, Yellow, Green, Blue, Indigo, Violet, }
enum Shape { I, O, T, Z, S, L, J, }

struct Piece {
  //color: Color,
    shape: Shape,
    cells: [Cell; 4],
}

//Board is an array of rows
struct Board([[Cell; WIDTH]; HEIGHT]);

fn main() {
    let a : Option<Cell> = None;
    ncurses::initscr();
    ncurses::raw();
    ncurses::noecho();
    ncurses::printw("hello world\n");
    loop {
        match ncurses::getch() {
            97  => ncurses::endwin(),
            _   => ncurses::printw("x"),
        };
    }
}
