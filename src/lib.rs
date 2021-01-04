pub mod render {
    pub mod svg;
    pub mod pdf;
    pub mod renderlib;
}
pub use render::pdf::render_pdf;
pub use render::svg::render_svg;
use std::error;
use std::fmt;
use std::ops::{Add, Sub};

mod parse;

use std::convert::TryFrom;

#[derive(Debug, Default)]
pub struct VersionError {
    version_string: String,
}

impl VersionError {
    fn boxed(version_string: &str) -> Box<VersionError> {
        Box::new(VersionError {
            version_string: version_string.to_string(),
        })
    }
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unsupported version string: {}", self.version_string)
    }
}

impl error::Error for VersionError {}

#[derive(Debug, Default)]
pub struct LinesData {
    pub version: i32,
    pub pages: Vec<Page>,
}

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
    EraseAll,
    Calligraphy,
    Pen,
    SelectionBrush,
}

impl std::convert::TryFrom<i32> for BrushType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            // There seem to be different "versions" of similar brushes (e.g.
            // "Brush" at 0 and 12). v3 seems e.g. to use Brush 0 while v5 seems
            // to use Brush 12.
            0 => Ok(BrushType::Brush),
            1 => Ok(BrushType::TiltPencil),
            2 => Ok(BrushType::Pen),
            3 => Ok(BrushType::Marker),
            4 => Ok(BrushType::Fineliner),
            5 => Ok(BrushType::Highlighter),
            6 => Ok(BrushType::Eraser),
            7 => Ok(BrushType::SharpPencil),
            8 => Ok(BrushType::EraseArea),
            9 => Ok(BrushType::EraseAll),
            10 => Ok(BrushType::SelectionBrush),
            11 => Ok(BrushType::SelectionBrush),
            12 => Ok(BrushType::Brush),
            13 => Ok(BrushType::SharpPencil),
            14 => Ok(BrushType::TiltPencil),
            15 => Ok(BrushType::BallPoint),
            16 => Ok(BrushType::Marker),
            17 => Ok(BrushType::Fineliner),
            18 => Ok(BrushType::Highlighter),
            21 => Ok(BrushType::Calligraphy),
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
    // TODO: Make this a Result to avoid panic if index is out of range
    fn segment_length(&self, i: usize) -> Result<f32, &str> {
        if i + 1 >= self.points.len() {
            Err("Line segment index out of bounds")
        } else {
            Ok(self.points[i].distance(&self.points[i + 1]))
        }
    }

    fn length(&self) -> f32 {
        let mut length = 0.0;
        for (previous_index, point) in self.points[1..].iter().enumerate() {
            length += point.distance(&self.points[previous_index]);
        }
        length
    }

    /// Average of each segment's width, weighted by the segment length.
    /// Primarily useful for rendering to targets requiring a fixed line width.
    fn average_width(&self) -> f32 {
        // TODO: Are the width values of the first and second point always the same?
        let mut average_width = 0.0;
        let mut total_length = 0.0;
        for (i, point) in self.points[1..].iter().enumerate() {
            let segment_length = self.segment_length(i).unwrap_or_else(|_| unreachable!());
            total_length += segment_length;
            average_width += segment_length / total_length * (point.width - average_width);
        }
        average_width
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
    fn distance(&self, point: &Point) -> f32 {
        ((self.x - point.x).powi(2) + (self.y - point.y).powi(2)).sqrt()
    }
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = DirectionVec;

    fn sub(self, other: &Point) -> DirectionVec {
        DirectionVec {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, 'b> Add<&'b DirectionVec> for &'a Point {
    type Output = Point;

    fn add(self, other: &DirectionVec) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            ..Default::default()
        }
    }
}

impl<'a, 'b> Sub<&'b DirectionVec> for &'a Point {
    type Output = Point;

    fn sub(self, other: &DirectionVec) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            ..Default::default()
        }
    }
}

pub struct DirectionVec {
    x: f32,
    y: f32,
}

impl DirectionVec {
    fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn normalize(mut self) -> Result<DirectionVec, &'static str> {
        let length = self.length();
        if length == 0.0 {
            return Err("Can't normalize a 0-length vector");
        }
        self.x /= length;
        self.y /= length;
        Ok(self)
    }

    fn rotate_orthogonally(mut self) -> DirectionVec {
        std::mem::swap(&mut self.x, &mut self.y);
        self.y *= -1.0;
        self
    }
}

impl Add for DirectionVec {
    type Output = DirectionVec;

    fn add(self, other: DirectionVec) -> DirectionVec {
        DirectionVec {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub struct LayerColors {
    pub colors: Vec<(String, String, String)>,
}
