pub mod render;
pub mod v3;
pub mod v5;
pub use render::{render_pdf, render_svg};

use std::convert::TryFrom;

#[derive(Default, Debug)]
pub struct Page {
    pub layers: Vec<Layer>,
}

#[derive(Default, Debug)]
pub struct Layer {
    pub lines: Vec<Line>,
}

#[derive(Debug)]
pub enum BrushType {
    BallPoint,
    Marker,
    Fineliner,
    SharpPencil,
    TiltPencil,
    Brush,
    Highlighter,
    Eraser,
    EraseArea,
}

impl std::convert::TryFrom<i32> for BrushType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            15 => Ok(BrushType::BallPoint),
            16 => Ok(BrushType::Marker),
            17 => Ok(BrushType::Fineliner),
            13 => Ok(BrushType::SharpPencil),
            14 => Ok(BrushType::TiltPencil),
            12 => Ok(BrushType::Brush),
            18 => Ok(BrushType::Highlighter),
            6 => Ok(BrushType::Eraser),
            8 => Ok(BrushType::EraseArea),
            v => Err(format!("Unknown brush type: {}", v)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Grey,
    White,
}

impl TryFrom<i32> for Color {
    type Error = String;
    fn try_from(color_i: i32) -> Result<Self, Self::Error> {
        match color_i {
            0 => Ok(Color::Black),
            1 => Ok(Color::Grey),
            2 => Ok(Color::White),
            _ => Err(format!("Unknown color: {}", color_i)),
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub brush_type: BrushType,
    pub color: Color,
    pub unknown_line_attribute: i32,
    pub unknown_line_attribute_2: i32,
    pub brush_base_size: f32,
    pub points: Vec<Point>,
}

impl Line {
    pub fn new(t: (i32, i32, i32, f32), pts: Vec<Point>) -> Line {
        Line {
            brush_type: BrushType::try_from(t.0).unwrap(),
            color: Color::try_from(t.1).unwrap(),
            unknown_line_attribute: t.2,
            unknown_line_attribute_2: 0,
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
