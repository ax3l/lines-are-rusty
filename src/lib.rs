pub mod parse;
pub mod render;

pub use self::parse::*;

#[derive(Default, Debug)]
pub struct Page {
    pub layers: Vec<Layer>,
}

#[derive(Default, Debug)]
pub struct Layer {
    pub lines: Vec<Line>,
}

#[derive(Default, Debug)]
pub struct Line {
    pub brush_type: i32,
    pub color: i32,
    pub unknown_line_attribute: i32,
    pub brush_base_size: f32,
    pub points: Vec<Point>,
}

impl Line {
    pub fn new(t: (i32, i32, i32, f32), pts: Vec<Point>) -> Line {
        Line {
            brush_type: t.0,
            color: t.1,
            unknown_line_attribute: t.2,
            brush_base_size: t.3,
            points: pts,
        }
    }
}

#[derive(Default, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub direction: f32,
    pub width: f32,
    pub pressure: f32,
}

impl Point {
    pub fn new(f: (f32, f32, f32, f32, f32, f32)) -> Point {
        Point {
            x: f.0,
            y: f.1,
            speed: f.2,
            direction: f.3,
            width: f.4,
            pressure: f.5,
        }
    }
}
