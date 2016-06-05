//Tetris... in Rust
//shoutout to ncurses and ncurses-rs
//this is kind of a freebie:
// https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_4.rs
//(I installed ncurses with package `libncursesw5-dev`)

extern crate ncurses;

fn main() {
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
