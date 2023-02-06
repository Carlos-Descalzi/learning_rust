
#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {

    pub const UP:Point = Point {x:0,y:-1};
    pub const DOWN:Point = Point {x:0,y:1};
    pub const LEFT:Point = Point {x:-1,y:0};
    pub const RIGHT:Point = Point {x:1,y:0};

    pub fn add(&mut self, other: Point) -> Point {
        Point {x: self.x+other.x, y: self.y+other.y}
    }
    pub fn eq(&self, other: Point) -> bool {
        self.x == other.x && self.y == other.y
    }
    pub fn reverse(&mut self) -> Point {
        Point{x: -self.x,y: -self.y}
    }
}

