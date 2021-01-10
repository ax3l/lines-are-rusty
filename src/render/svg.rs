use crate::render::renderlib::{line_to_css_color, BoundingBox};
use crate::*;
use std::io::Write;

const WIDTH_FACTOR: f32 = 0.8;

pub fn render_constant_width_line(line: &Line, css_color: &str) -> svg::node::element::Path {
    let first_point = &line.points[0];

    let mut data = svg::node::element::path::Data::new().move_to((first_point.x, first_point.y));
    for point in line.points.iter() {
        data = data.line_to((point.x, point.y));
    }

    let mut path = svg::node::element::Path::new()
        .set("fill", "none")
        .set("d", data)
        .set("stroke", css_color)
        .set("class", format!("{:#?}", line.brush_type));

    match line.brush_type {
        BrushType::Highlighter => {
            path = path.set("stroke-width", first_point.width)
                .set("stroke-linecap", "butt")
                .set("stroke-opacity", 0.25);
            }
        _ => {
            path = path.set("stroke-width", first_point.width * WIDTH_FACTOR)
                .set("stroke-linecap", "round");
        }
    }

    path
}

pub fn render_variable_width_line(line: &Line, css_color: &str) -> svg::node::element::Group {
    let mut stroke_group = svg::node::element::Group::new()
        .set("fill", "none")
        .set("stroke", css_color)
        .set("stroke-linecap", "round")
        .set("class", format!("{:#?}", line.brush_type));

    for (previous_index, point) in line.points[1..].iter().enumerate() {
        let prev_point = &line.points[previous_index];
        let data = svg::node::element::path::Data::new()
            .move_to((prev_point.x, prev_point.y))
            .line_to((point.x, point.y));
        let opacity = match line.brush_type {
            BrushType::BallPoint => point.pressure.powf(5.0) + 0.7,
            _ => 1.0,
        };

        let mut path = svg::node::element::Path::new()
            .set("stroke-width", point.width * WIDTH_FACTOR)
            .set("d", data);
        if opacity < 1.0 {
            path = path.set("stroke-opacity", opacity);
        }
        stroke_group = stroke_group.add(path);
    }
    stroke_group
}

pub fn render_svg(
    output: &mut dyn Write,
    page: &Page,
    auto_crop: bool,
    layer_colors: &LayerColors,
) {
    let mut doc = svg::Document::new();
    for (layer_id, layer) in page.layers.iter().enumerate() {
        let mut layer_group = svg::node::element::Group::new().set("class", "layer");
        for line in layer.lines.iter() {
            if line.points.is_empty() {
                continue;
            }
            let css_color = line_to_css_color(&line, layer_id, layer_colors);
            match line.brush_type {
                BrushType::Highlighter | BrushType::Fineliner => {
                    layer_group = layer_group.add(render_constant_width_line(line, css_color))
                }
                BrushType::EraseArea
                | BrushType::Eraser
                | BrushType::EraseAll
                | BrushType::SelectionBrush => (),
                _ => layer_group = layer_group.add(render_variable_width_line(line, css_color)),
            }
        }
        doc = doc.add(layer_group);
    }
    if auto_crop {
        let BoundingBox {
            min_x,
            min_y,
            max_x,
            max_y,
        } = BoundingBox::new().enclose_page(page);
        doc = doc.set("viewBox", (min_x, min_y, max_x - min_x, max_y - min_y));
    } else {
        doc = doc.set("viewBox", (0, 0, 1404, 1872));
    }
    svg::write(output, &doc).expect("Failed to save svg doc");
}
