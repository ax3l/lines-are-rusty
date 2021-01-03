use crate::{BrushType, Color, LayerColors, Line, Page, Point};
use core::f32::{INFINITY, NEG_INFINITY};

pub(crate) struct BoundingBox {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl BoundingBox {
    pub fn new() -> BoundingBox {
        BoundingBox {
            min_x: INFINITY,
            min_y: INFINITY,
            max_x: NEG_INFINITY,
            max_y: NEG_INFINITY,
        }
    }

    pub fn enclose_point(mut self, point: &Point) -> BoundingBox {
        self.min_x = self.min_x.min(point.x);
        self.min_y = self.min_y.min(point.y);
        self.max_x = self.max_x.max(point.x);
        self.max_y = self.max_y.max(point.y);
        self
    }

    pub fn enclose_line(mut self, line: &Line) -> BoundingBox {
        for point in line.points.iter() {
            self = self.enclose_point(point);
        }
        self
    }

    pub fn enclose_page(mut self, page: &Page) -> BoundingBox {
        for layer in page.layers.iter() {
            for line in layer.lines.iter() {
                self = self.enclose_line(line);
            }
        }
        self
    }
}

pub fn line_to_css_color<'a>(
    line: &Line,
    layer_id: usize,
    layer_colors: &'a LayerColors,
) -> &'a str {
    match line.brush_type {
        BrushType::Highlighter => "rgb(240, 220, 40)",
        _ => match line.color {
            Color::Black => &layer_colors.colors[layer_id].0,
            Color::Grey => &layer_colors.colors[layer_id].1,
            Color::White => &layer_colors.colors[layer_id].2,
        },
    }
}
