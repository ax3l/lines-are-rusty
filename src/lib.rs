pub mod render {
    pub mod pdf;
    pub mod renderlib;
    pub mod svg;
    pub mod templates;
}
pub mod parse {
    pub mod parse_lines;
}
pub use render::pdf::render_pdf;
pub use render::svg::render_svg;
use std::ops::{Add, Div, Mul, Sub};
use thiserror::Error;

use std::convert::TryFrom;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unsupported version string: {0}")]
    UnsupportedVersion(String),

    #[error("Unknown brush type: {0}")]
    UnknownBrush(i32),

    #[error("Unknown color: {0}")]
    UnknownColor(i32),

    #[error("Unknown template: '{0}' Valid templates are:\n  {}", crate::render::templates::TEMPLATES.keys().copied().map(|t| format!("\n  {}", t)).collect::<String>())]
    UnknownTemplate(String),

    #[error("Invalid Segment index: {0}")]
    InvalidSegmentIndex(usize),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
}
type Result<T> = core::result::Result<T, Error>;

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

impl Default for BrushType {
    fn default() -> BrushType {
        BrushType::Fineliner
    }
}

impl std::convert::TryFrom<i32> for BrushType {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self> {
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
            v => Err(Error::UnknownBrush(v)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Grey,
    White,
    Blue,
    Red,
}

impl TryFrom<i32> for Color {
    type Error = Error;

    fn try_from(color_i: i32) -> Result<Self> {
        match color_i {
            0 => Ok(Color::Black),
            1 => Ok(Color::Grey),
            2 => Ok(Color::White),
            6 => Ok(Color::Blue),
            7 => Ok(Color::Red),
            _ => Err(Error::UnknownColor(color_i)),
        }
    }
}

impl Default for Color {
    fn default() -> Color {
        Color::Black
    }
}

#[derive(Default, Debug)]
pub struct Line {
    pub brush_type: BrushType,
    pub color: Color,
    pub unknown_line_attribute: i32,
    pub unknown_line_attribute_2: i32,
    pub brush_base_size: f32,
    pub points: Vec<Point>,
}

impl Line {
    fn segment_length(&self, i: usize) -> Result<f32> {
        if i + 1 >= self.points.len() {
            Err(Error::InvalidSegmentIndex(i))
        } else {
            Ok(self.points[i].distance(&self.points[i + 1]))
        }
    }

    fn length(&self) -> f32 {
        self.points
            .iter()
            .zip(self.points[1..].iter())
            .map(|(previous_point, point)| previous_point.distance(point))
            .sum()
    }

    /// Average of each segment's width, weighted by the segment length.
    /// Primarily useful for rendering to targets requiring a fixed line width.
    fn average_width(&self) -> f32 {
        // TODO: Are the width values of the first and second point always the same?

        // Algorithm for weighted average see e.g. notes by Tony Finch:
        // Incremental calculation of weighted mean and variance, chapter 4
        // https://fanf2.user.srcf.net/hermes/doc/antiforgery/stats.pdf#page=3
        let mut average_width = 0.0;
        let mut total_length = 0.0;
        for (i, point) in self.points[1..].iter().enumerate() {
            let segment_length = self.segment_length(i).unwrap_or_else(|_| unreachable!());
            total_length += segment_length;
            average_width += segment_length / total_length * (point.width - average_width);
        }
        average_width
    }

    /// Produces the offset vectors for each line segment for creating a offset
    /// polyline. Each offset vector indicates the direction and distance for
    /// offsetting the line segment. The offset vector can be mirrored to get
    /// the offset to the other side of the polyline segment.
    fn offsets(&self, offset_distance: f32) -> Vec<DirectionVec> {
        let points = &self.points;
        (1..points.len())
            .map(|i| {
                let v = &points[i] - &points[i - 1];
                v.rotate_orthogonally().set_length(offset_distance)
            })
            .collect()
    }

    fn with_points(template: Point, points: &[(f32, f32)]) -> Line {
        Line {
            points: points
                .iter()
                .map(|p| {
                    let mut point = template.clone();
                    point.x = p.0;
                    point.y = p.1;
                    point
                })
                .collect(),
            ..Default::default()
        }
    }
}

#[test]
fn test_line_offsets() {
    let line = Line::with_points(
        Point {
            width: 2.0,
            ..Default::default()
        },
        &vec![(0.0, 0.0), (3.0, 4.0), (6.0, 4.0)][..],
    );

    assert_eq!(
        line.offsets(5.0),
        vec![
            DirectionVec { x: -4.0, y: 3.0 },
            DirectionVec { x: 0.0, y: 5.0 }
        ]
    );
}

#[derive(Default, Debug, Clone)]
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

#[derive(Default, Debug, PartialEq)]
pub struct DirectionVec {
    x: f32,
    y: f32,
}

impl DirectionVec {
    const ZERO: DirectionVec = DirectionVec { x: 0.0, y: 0.0 };

    fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn set_length(mut self, length: f32) -> DirectionVec {
        let factor = self.length() / length;
        if factor != 0.0 {
            self.x /= factor;
            self.y /= factor;
        }
        self
    }

    fn rotate_orthogonally(mut self) -> DirectionVec {
        std::mem::swap(&mut self.x, &mut self.y);
        self.x *= -1.0;
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

impl Mul<f32> for DirectionVec {
    type Output = DirectionVec;

    fn mul(self, other: f32) -> DirectionVec {
        DirectionVec {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f32> for DirectionVec {
    type Output = DirectionVec;

    fn div(self, other: f32) -> DirectionVec {
        DirectionVec {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

// A PDF style 2D affine transformation matrix [a b c d e f].
// PDF omits the third column of the matrix that in full would look like:
// [ a b 0 ]
// [ c d 0 ]
// [ e f 1 ]
pub struct Matrix([f32; 6]);

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut result: [f32; 6] = [0.0; 6];
        for row in 0..2 {
            for col in 0..3 {
                let mut x = 0.0;
                for i in 0..2 {
                    x += self.0[row + 2 * i] * rhs.0[2 * col + i];
                }
                result[2 * col + row] = x;
            }
        }
        result[4] += self.0[4];
        result[5] += self.0[5];

        Matrix(result)
    }
}

#[test]
fn test_matrix_mul() {
    let a = Matrix([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let b = Matrix([2.0, 1.0, 3.0, 2.0, 4.0, 3.0]);

    assert_eq!([5.0, 8.0, 9.0, 14.0, 18.0, 26.0], (a * b).0,);
}

impl Mul<[f32; 2]> for Matrix {
    type Output = [f32; 2];

    fn mul(self, point: [f32; 2]) -> [f32; 2] {
        [
            self.0[0] * point[0] + self.0[2] * point[1] + self.0[4],
            self.0[1] * point[0] + self.0[3] * point[1] + self.0[5],
        ]
    }
}

impl Mul<Point> for Matrix {
    type Output = [f32; 2];

    fn mul(self, point: Point) -> [f32; 2] {
        self * [point.x, point.y]
    }
}

#[test]
fn test_matrix_point_mul() {
    let m = Matrix([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let p = Point {
        x: 2.0,
        y: 1.0,
        ..Default::default()
    };

    assert_eq!([10.0, 14.0], m * p);
}

#[derive(Clone)]
pub struct LayerColor {
    pub black: String,
    pub grey: String,
    pub white: String,
    pub blue: String,
    pub red: String,
}

impl Default for LayerColor {
    fn default() -> Self {
        Self {
            black: "black".to_string(),
            grey: "#bfbfbf".to_string(),
            white: "white".to_string(),
            blue: "#0062cc".to_string(),
            red: "#d90707".to_string(),
        }
    }
}
