//project euler problem 11
//This was one I could never solve a few years ago in Python
//find the greatest product of 4 adjacent numbers in a 20x20 grid
//diagonals included 

use std::fs::File;
use std::fmt;
use std::io::{BufReader, BufRead};

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
    //fn get(&self, x: usize, y: usize) -> i32 {
    //fn get(&self, Point{x,y}: Point) -> i32 {
    fn get(&self, p: &Point) -> i32 {
        self.0[p.y as usize][p.x as usize]
    }
    fn generate_lines(&self, Point{x,y}: Point) -> Vec<Line> {
        //(try to) form lines going right, down, and right-down
        let mut lines = vec![];
        //for (i,j) in vec![(x+3,y), (x,y+3), (x+3,y+3)] {
        for (i,j) in vec![(x+3,y), (x,y+3), (x+3,y+3), (x-3,y-3), (x-3,y+3), (x+3,y-3)] {
            let p = Point{x:i, y:j};
            if p.is_valid() {
                let (dx,dy) = ((i-x)/3,(j-y)/3);
                //TODO: do functionally
                let points = Line(vec![
                    Point{x:x+0*dx, y:y+0*dy},
                    Point{x:x+1*dx, y:y+1*dy},
                    Point{x:x+2*dx, y:y+2*dy},
                    Point{x:x+3*dx, y:y+3*dy}]);
                //println!("Points: \n{:?}", points);
                lines.push(points);
            }
        }
        lines
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
            let mut l = table.generate_lines(p);
            lines.append(&mut l);
        }
    }

    for l in lines.iter().take(10) {
        let mut n = 1;
        for point in &l.0 {
            n *= table.get(point);
        }
        println!("{}: \t{}", l, n);
    }
    println!("total lines: {}", lines.len());

    let mut max: i32 = 0;
    let mut n: i32;
    let mut best: Line = Line(vec![]);
    for line in lines {
        n = 1;
        for point in &line.0 {
            if point.is_valid(){
                n *= table.get(point);
                assert!(point.x >= 0 && point.y >= 0);
                assert!(point.x <LEN && point.y <LEN);
            }
        }
        if n > max {
            max = n;
            best = line;
        }
    }
    println!("max: {}", max);
    println!("best: {:?}", best);
    //println!("Number of lines: {}", lines.len());
    assert!(table.0[0].len() == LEN as usize);
    assert!(table.0.len() == LEN as usize);
    //println!("table at (2,0): {}", table.get(&Point{x:2,y:0}));
}
