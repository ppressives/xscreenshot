use std::cmp;

pub struct Selection {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

#[derive(Clone)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Selection {
    pub fn new(start: &Point, end: &Point) -> Selection {
        let top_left = Point::new(cmp::min(start.x, end.x), cmp::min(start.y, end.y));
        let bottom_right = Point::new(cmp::max(start.x, end.x), cmp::max(start.y, end.y));
        Selection { x1: top_left.x, y1: top_left.y, x2: bottom_right.x, y2: bottom_right.y }
    }

    pub fn width(&self) -> u32 {
        self.x2 - self.x1
    }

    pub fn height(&self) -> u32 {
        self.y2 - self.y1
    }
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}