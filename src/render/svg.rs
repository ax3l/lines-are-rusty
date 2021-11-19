use crate::render::renderlib::{line_to_css_color, BoundingBox};
use crate::{BrushType, LayerColors, Line, Page, Result};
use std::io;

const WIDTH_FACTOR: f32 = 0.8;

pub fn render_constant_width_line(
    line: &Line,
    css_color: &str,
    distance_threshold: f32,
    debug_dump: bool,
) -> svg::node::element::Path {
    let mut point_iter = line.points.iter().enumerate();

    let mut prev_point = if let Some((_, p)) = point_iter.next() {
        p
    } else {
        return svg::node::element::Path::new();
    };

    let mut data = svg::node::element::path::Data::new().move_to((prev_point.x, prev_point.y));
    for (idx, point) in point_iter {
        let dist = point - prev_point;
        let is_last_point = idx + 1 == line.points.len();
        if dist.length() < distance_threshold && !is_last_point {
            continue;
        }
        data = data.line_to((point.x, point.y));
        prev_point = point;
    }

    let mut path = svg::node::element::Path::new()
        .set("fill", "none")
        .set("d", data)
        .set("color", css_color)
        .set("stroke-linejoin", "round")
        .set("stroke", "currentColor")
        .set("class", format!("{:#?}", line.brush_type));

    match line.brush_type {
        BrushType::Highlighter => {
            path = path
                .set("stroke-width", prev_point.width)
                .set("stroke-linecap", "butt")
                .set("stroke-opacity", 0.25f32);
        }
        _ => {
            path = path
                .set("stroke-width", prev_point.width * WIDTH_FACTOR)
                .set("stroke-linecap", "round");
        }
    }

    if debug_dump {
        path = path.add(tooltip(&format!("{:#?}", line)));
    }

    path
}

pub fn render_variable_width_line(
    line: &Line,
    css_color: &str,
    distance_threshold: f32,
    debug_dump: bool,
) -> svg::node::element::Group {
    let mut stroke_group = svg::node::element::Group::new()
        .set("fill", "none")
        .set("color", css_color)
        .set("stroke", "currentColor")
        .set("stroke-linecap", "round")
        .set("class", format!("{:#?}", line.brush_type));

    let mut point_iter = line.points.iter().enumerate();

    let mut prev_point = if let Some((_, p)) = point_iter.next() {
        p
    } else {
        return svg::node::element::Group::new();
    };

    for (idx, point) in point_iter {
        let dist = point - prev_point;
        let is_last_point = idx + 1 == line.points.len();
        if dist.length() < distance_threshold && !is_last_point {
            continue;
        }

        let data = svg::node::element::path::Data::new()
            .move_to((prev_point.x, prev_point.y))
            .line_to((point.x, point.y));

        prev_point = point;

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

        if debug_dump {
            path = path.add(tooltip(&format!("{:#?}\n{:#?}", prev_point, point)));
        }

        stroke_group = stroke_group.add(path);
    }
    stroke_group
}

pub fn render_svg(
    output: &mut dyn io::Write,
    page: &Page,
    auto_crop: bool,
    layer_colors: &LayerColors,
    distance_threshold: f32,
    debug_dump: bool,
) -> Result<()> {
    let mut doc = svg::Document::new();
    for (layer_id, layer) in page.layers.iter().enumerate() {
        let mut layer_group = svg::node::element::Group::new().set("class", "layer");
        for line in layer.lines.iter() {
            let css_color = line_to_css_color(line, layer_id, layer_colors);
            match &line.brush_type {
                BrushType::Highlighter | BrushType::Fineliner => {
                    layer_group = layer_group.add(render_constant_width_line(
                        line,
                        css_color,
                        distance_threshold,
                        debug_dump,
                    ))
                }
                BrushType::EraseArea
                | BrushType::Eraser
                | BrushType::EraseAll
                | BrushType::SelectionBrush => (),
                _ => {
                    layer_group = layer_group.add(render_variable_width_line(
                        line,
                        css_color,
                        distance_threshold,
                        debug_dump,
                    ))
                }
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
    if debug_dump {
        doc = add_debug_style(doc);
    }
    svg::write(output, &doc)
}

fn tooltip(tooltip_text: &str) -> svg::node::element::Title {
    let title = svg::node::element::Title::new();
    title.add(svg::node::Text::new(tooltip_text))
}

fn add_debug_style(svg: svg::node::element::SVG) -> svg::node::element::SVG {
    svg.add(svg::node::element::Style::new(
        r#"
        path:hover {
            filter: drop-shadow(0 0 5px #e00);
            color: #e00;
        }
    "#,
    ))
}
