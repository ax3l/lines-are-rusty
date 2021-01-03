use crate::{BrushType, Color, LayerColors, Line};
use crate::Page;

pub fn crop(page: &Page) -> (f32, f32, f32, f32) {
    let mut min_x = 1404_f32;
    let mut min_y = 1872_f32;
    let mut max_x = 0_f32;
    let mut max_y = 0_f32;
    for layer in page.layers.iter() {
        for line in layer.lines.iter() {
            for point in line.points.iter() {
                min_x = min_x.min(point.x);
                min_y = min_y.min(point.y);
                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);
            }
        }
    }
    (min_x, min_y, max_x, max_y)
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
