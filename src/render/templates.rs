use phf::phf_map;

use crate::{Error, Result};

pub static TEMPLATES: phf::Map<&'static str, &str> = phf_map! {
    "Blank" => include_str!("../../templates/Blank.svg"),
    "Isometric" => include_str!("../../templates/Isometric.svg"),
    "LS Calligraphy large" => include_str!("../../templates/LS Calligraphy large.svg"),
    "LS Calligraphy medium" => include_str!("../../templates/LS Calligraphy medium.svg"),
    "LS Checklist double" => include_str!("../../templates/LS Checklist double.svg"),
    "LS Checklist" => include_str!("../../templates/LS Checklist.svg"),
    "LS Dayplanner" => include_str!("../../templates/LS Dayplanner.svg"),
    "LS Dots bottom" => include_str!("../../templates/LS Dots bottom.svg"),
    "LS Dots top" => include_str!("../../templates/LS Dots top.svg"),
    "LS Four storyboards" => include_str!("../../templates/LS Four storyboards.svg"),
    "LS Grid bottom" => include_str!("../../templates/LS Grid bottom.svg"),
    "LS Grid margin large" => include_str!("../../templates/LS Grid margin large.svg"),
    "LS Grid margin med" => include_str!("../../templates/LS Grid margin med.svg"),
    "LS Grid top" => include_str!("../../templates/LS Grid top.svg"),
    "LS Lines bottom" => include_str!("../../templates/LS Lines bottom.svg"),
    "LS Lines medium" => include_str!("../../templates/LS Lines medium.svg"),
    "LS Lines small" => include_str!("../../templates/LS Lines small.svg"),
    "LS Lines top" => include_str!("../../templates/LS Lines top.svg"),
    "LS Margin medium" => include_str!("../../templates/LS Margin medium.svg"),
    "LS Margin small" => include_str!("../../templates/LS Margin small.svg"),
    "LS One storyboard 2" => include_str!("../../templates/LS One storyboard 2.svg"),
    "LS One storyboard" => include_str!("../../templates/LS One storyboard.svg"),
    "LS Piano sheet large" => include_str!("../../templates/LS Piano sheet large.svg"),
    "LS Piano sheet medium" => include_str!("../../templates/LS Piano sheet medium.svg"),
    "LS Piano sheet small" => include_str!("../../templates/LS Piano sheet small.svg"),
    "LS Two storyboards" => include_str!("../../templates/LS Two storyboards.svg"),
    "LS Week US" => include_str!("../../templates/LS Week US.svg"),
    "LS Week" => include_str!("../../templates/LS Week.svg"),
    "Notes" => include_str!("../../templates/Notes.svg"),
    "P Bass tab" => include_str!("../../templates/P Bass tab.svg"),
    "P Black dots" => include_str!("../../templates/P Black dots.svg"),
    "P Black grid" => include_str!("../../templates/P Black grid.svg"),
    "P Black lines" => include_str!("../../templates/P Black lines.svg"),
    "P Black" => include_str!("../../templates/P Black.svg"),
    "P Calligraphy large" => include_str!("../../templates/P Calligraphy large.svg"),
    "P Calligraphy medium" => include_str!("../../templates/P Calligraphy medium.svg"),
    "P Checklist" => include_str!("../../templates/P Checklist.svg"),
    "P Cornell" => include_str!("../../templates/P Cornell.svg"),
    "P Day" => include_str!("../../templates/P Day.svg"),
    "P Dots S bottom" => include_str!("../../templates/P Dots S bottom.svg"),
    "P Dots S top" => include_str!("../../templates/P Dots S top.svg"),
    "P Dots S" => include_str!("../../templates/P Dots S.svg"),
    "P Dots large" => include_str!("../../templates/P Dots large.svg"),
    "P Four storyboards" => include_str!("../../templates/P Four storyboards.svg"),
    "P Grid bottom" => include_str!("../../templates/P Grid bottom.svg"),
    "P Grid large" => include_str!("../../templates/P Grid large.svg"),
    "P Grid margin large" => include_str!("../../templates/P Grid margin large.svg"),
    "P Grid margin med" => include_str!("../../templates/P Grid margin med.svg"),
    "P Grid medium" => include_str!("../../templates/P Grid medium.svg"),
    "P Grid small" => include_str!("../../templates/P Grid small.svg"),
    "P Grid top" => include_str!("../../templates/P Grid top.svg"),
    "P Guitar chords" => include_str!("../../templates/P Guitar chords.svg"),
    "P Guitar tab" => include_str!("../../templates/P Guitar tab.svg"),
    "P Hexagon large" => include_str!("../../templates/P Hexagon large.svg"),
    "P Hexagon medium" => include_str!("../../templates/P Hexagon medium.svg"),
    "P Hexagon small" => include_str!("../../templates/P Hexagon small.svg"),
    "P Lined bottom" => include_str!("../../templates/P Lined bottom.svg"),
    "P Lined heading" => include_str!("../../templates/P Lined heading.svg"),
    "P Lined top" => include_str!("../../templates/P Lined top.svg"),
    "P Lines large" => include_str!("../../templates/P Lines large.svg"),
    "P Lines medium" => include_str!("../../templates/P Lines medium.svg"),
    "P Lines small" => include_str!("../../templates/P Lines small.svg"),
    "P Margin large" => include_str!("../../templates/P Margin large.svg"),
    "P Margin medium" => include_str!("../../templates/P Margin medium.svg"),
    "P Margin small" => include_str!("../../templates/P Margin small.svg"),
    "P One storyboard" => include_str!("../../templates/P One storyboard.svg"),
    "P Piano sheet large" => include_str!("../../templates/P Piano sheet large.svg"),
    "P Piano sheet medium" => include_str!("../../templates/P Piano sheet medium.svg"),
    "P Piano sheet small" => include_str!("../../templates/P Piano sheet small.svg"),
    "P Two storyboards" => include_str!("../../templates/P Two storyboards.svg"),
    "P US College" => include_str!("../../templates/P US College.svg"),
    "P US Legal" => include_str!("../../templates/P US Legal.svg"),
    "P Week 2" => include_str!("../../templates/P Week 2.svg"),
    "P Week US" => include_str!("../../templates/P Week US.svg"),
    "P Week" => include_str!("../../templates/P Week.svg"),
    "Perspective1" => include_str!("../../templates/Perspective1.svg"),
    "Perspective2" => include_str!("../../templates/Perspective2.svg"),
};

pub fn template_snippet(template_name: &str) -> Result<&str> {
    TEMPLATES
        .get(template_name)
        .map(|template| {
            let svg_start_tag_start = template.find("<svg").expect("Missing svg tag");
            let svg_start_tag_end = template[svg_start_tag_start..]
                .find(">")
                .expect("Missing svg end tag")
                + 1;
            let svg_end = template.find("</svg>").expect("Missing svg closing tag");

            &template[svg_start_tag_end..svg_end]
        })
        .ok_or(Error::UnknownTemplate(template_name.to_string()))
}
