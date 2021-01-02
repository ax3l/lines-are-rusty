use crate::render::renderlib::crop;
use crate::*;
use std::io::Write;

// black,grey,white;red,magenta,white;blue,cyan,white;limegreen,yellow,white;darkorchid,darkorange,white

const WIDTH_FACTOR: f32 = 0.8;

pub fn line_to_svg_color<'a>(line: &Line, layer_id: usize, layer_colors: &'a LayerColors) -> &'a str {
    match line.brush_type {
        BrushType::Highlighter => "rgb(240, 220, 40)",
        _ => match line.color {
            Color::Black => &layer_colors.colors[layer_id].0,
            Color::Grey => &layer_colors.colors[layer_id].1,
            Color::White => &layer_colors.colors[layer_id].2,
        },
    }
}

pub fn render_highlighter_line(line: &Line, min_x: f32, min_y: f32, layer_id: usize, layer_colors: &LayerColors) -> svg::node::element::Path {
    let first_point = &line.points[0];

    let mut data = svg::node::element::path::Data::new().move_to((first_point.x-min_x, first_point.y-min_y));
    for point in line.points.iter() {
        data = data.line_to((point.x-min_x, point.y-min_y));
    }

    svg::node::element::Path::new()
        .set("fill", "none")
        .set("stroke", line_to_svg_color(line, layer_id, layer_colors))
        .set("stroke-width", first_point.width) // no WIDTH_FACTOR used here! factor is 1
        .set("stroke-linecap", "butt")
        .set("stroke-linejoin", "bevel")
        .set("stroke-opacity", 0.25)
        .set("d", data)
}

pub fn render_fineliner_line(line: &Line, min_x: f32, min_y: f32, layer_id: usize, layer_colors: &LayerColors) -> svg::node::element::Path {
    let first_point = &line.points[0];

    let mut data = svg::node::element::path::Data::new().move_to((first_point.x-min_x, first_point.y-min_y));
    for point in line.points.iter() {
        data = data.line_to((point.x-min_x, point.y-min_y));
    }

    svg::node::element::Path::new()
        .set("fill", "none")
        .set("stroke", line_to_svg_color(line, layer_id, layer_colors))
        .set("stroke-width", first_point.width * WIDTH_FACTOR)
        .set("stroke-linecap", "round")
        .set("d", data)
}

pub fn render_svg(output: &mut dyn Write, page: &Page, auto_crop: bool, layer_colors: &LayerColors) {
    let (min_x, min_y, max_x, max_y) = if auto_crop {
        crop(page)
    } else {
        (0_f32, 0_f32, 1404_f32, 1872_f32)
    };
    let mut doc = svg::Document::new();
    for (layer_id, layer) in page.layers.iter().enumerate() {
        for line in layer.lines.iter() {
            if line.points.is_empty() {
                continue;
            }
            let color = line_to_svg_color(&line, layer_id, layer_colors);
            match line.brush_type {
                BrushType::Highlighter => doc = doc.add(render_highlighter_line(line, min_x, min_y, layer_id, layer_colors)),
                BrushType::Fineliner => doc = doc.add(render_fineliner_line(line, min_x, min_y, layer_id, layer_colors)),
                BrushType::EraseArea => (),
                BrushType::Eraser => (),
                BrushType::EraseAll => (),
                BrushType::SelectionBrush => (),
                _ => {
                    for (previous_index, point) in line.points[1..].iter().enumerate() {
                        let prev_point = &line.points[previous_index];
                        let data = svg::node::element::path::Data::new()
                            .move_to((prev_point.x - min_x, prev_point.y - min_y))
                            .line_to((point.x - min_x, point.y - min_y));
                        let (width, opacity) = match line.brush_type {
                            BrushType::BallPoint => (point.width, point.pressure.powf(5.0) + 0.7),
                            BrushType::Marker
                            | BrushType::SharpPencil
                            | BrushType::TiltPencil
                            | BrushType::Brush
                            | BrushType::Calligraphy
                            | BrushType::Pen => (point.width, 1.0),
                            BrushType::Highlighter
                            | BrushType::Fineliner
                            | BrushType::Eraser
                            | BrushType::EraseArea
                            | BrushType::EraseAll
                            | BrushType::SelectionBrush => unreachable!("Should have been handled above"),
                        };

                        if opacity != 1.0 {
                            doc = doc.add(
                                svg::node::element::Path::new()
                                    .set("fill", "none")
                                    .set("stroke", color)
                                    .set("stroke-width", width * WIDTH_FACTOR)
                                    .set("stroke-linecap", "round")
                                    .set("stroke-opacity", opacity)
                                    .set("d", data),
                            );
                        } else {
                            doc = doc.add(
                                svg::node::element::Path::new()
                                    .set("fill", "none")
                                    .set("stroke", color)
                                    .set("stroke-width", width * WIDTH_FACTOR)
                                    .set("stroke-linecap", "round")
                                    .set("d", data),
                            );
                        }
                    }
                }
            }
        }
    }
    let doc = doc.set("viewBox", (0, 0, max_x-min_x, max_y-min_y));
    svg::write(output, &doc).expect("Failed to save svg doc");
}
