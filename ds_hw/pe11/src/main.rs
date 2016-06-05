//Project Euler Problem 11: Largest Product in a Grid
//This was one I could never solve a few years ago in Python
//Find the greatest product of 4 adjacent numbers in a 20x20 grid
// diagonals included 
//The problem isn't particularly difficult, but I always came up with 
// the wrong ansert
//It's not as elegant as it could be, but it produces the correct result
//
//UPDATE: the problem I think I had last time was only testing one kind
// of diagonal line (\) and omitting the other type (/)
//
//Results of `time cargo run` (after running `cargo build`)
//  real  0m0.062s
//  user  0m0.044s
//  sys   0m0.012s
//Surprisingly fast considering it was written more to be debug-friendly

use std::fs::File;
use std::fmt;
use std::io::{BufReader, BufRead};

//dimension of the board
static LEN: i32 = 20;


struct Table(Vec<Vec<i32>>); 

impl Table {
    fn populate(f: String) -> Table {
        //read input:
        let f = File::open(f).expect("file not found");
        //run this from `pe11` directory (not src or anything)
        let br = BufReader::new(f);
        let mut numbers: Vec<Vec<i32>> = vec![]; 
        for line in br.lines() {
            //let nums = line.unwrap().split_whitespace();
            let mut ns = Vec::<i32>::new();
            for word in line.unwrap().split_whitespace() {
                ns.push(word.parse().unwrap());
            }
            numbers.push(ns);
        }
        Table(numbers)
    }
    fn get(&self, p: &Point) -> i32 {
        self.0[p.y as usize][p.x as usize]
    }
    fn generate_lines(&self, Point{x,y}: Point) -> Vec<Line> {
        //(try to) form lines going right, down, and right-down
        let mut lines = vec![];
        //try horizontal, vertical, diag1, diag2 (-, |, \, /)
        for (i,j) in vec![(x+3,y), (x,y+3), (x+3,y+3), (x-3,y+3),] {
            let p = Point{x:i, y:j};
            if p.is_valid() {
                let (dx,dy) = ((i-x)/3,(j-y)/3);
                let points  = (0..4)
                                .into_iter()
                                .map(|n| Point{x:x+n*dx, y:y+n*dy})
                                .collect();
                lines.push(Line(points));
            }
        }
        lines
    }
}


struct Line (Vec<Point>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:02},{:02}) -> ({:02},{:02}) -> ({:02},{:02}) -> ({:02},{:02})",
            self.0[0].x, self.0[0].y,
            self.0[1].x, self.0[1].y,
            self.0[2].x, self.0[2].y,
            self.0[3].x, self.0[3].y)
    }
}


struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn is_valid(&self) -> bool {
        self.x >= 0 && self.x < LEN 
        && self.y >= 0 && self.y < LEN
    }
}


fn main() {
    let table = Table::populate("grid".to_string());
    let p = Point{x:17, y:17};
    table.generate_lines(p);

    let mut lines = vec![];
    for x in 0..50 {
        for y in 0..50 {
            let p = Point{x:x,y:y};
            if p.is_valid() {
                let mut l = table.generate_lines(p);
                lines.append(&mut l);
            }
        }
    }
    println!("Lines: \t{}", lines.len());

    let mut max: i32 = 0;
    let mut n: i32;
    let mut best: Line = Line(vec![]);
    for line in lines {
        n = 1;
        for point in &line.0 {
            n *= table.get(point);
        }
        if n > max {
            max = n;
            best = line;
        }
    }
    println!("max: \t{}", max);
    println!("best: \t{}", best);
}
