use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct ExcalidrawConfig {
    pub font: Font,
    pub services: Services,
    pub ports: Ports,
    pub connections: Connections,
    pub alignment: Alignment,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Font {
    pub size: i32,
    pub family: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Services {
    pub background_color: String,
    pub fill: String,
    pub edge: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ports {
    pub background_color: String,
    pub fill: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Connections {
    pub visible: bool,
    pub edge: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Alignment {
    pub mode: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundElement {
    pub id: String,
    #[serde(rename = "type")]
    pub element_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    pub element_id: String,
    pub focus: f32,
    pub gap: u16,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Roundness {
    #[serde(rename = "type")]
    pub roundness_type: i32,
}

pub fn binding(element_id: String) -> Binding {
    Binding {
        element_id,
        focus: 0.05,
        gap: 1,
    }
}

pub fn arrow_bounded_element(id: String) -> BoundElement {
    BoundElement {
        id,
        element_type: "arrow".to_string(),
    }
}

pub fn roundness(edge: String) -> Option<Roundness> {
    match edge.as_str() {
        "round" => Some(Roundness { roundness_type: 3 }),
        _ => None,
    }
}

// TODO no magic numbers - extract to consts!
///// option: 1 horizontal
// let x_margin = 60;
// let y_margin = 0;

///// option: 2 vertical
// let x_margin = 0;
// let y_margin = 60;

// option: 3 'stepped'
// let x_margin = 60;
// let y_margin = 60;
pub fn margins(alignment_mode: String)-> (i32, i32) {
    dbg!(alignment_mode.clone());
    match alignment_mode.as_str() {
        "horizontal" => (60, 0),
        "vertical" => (0, 60),
        "stepped" => (60, 60),
        _ => (60, 60),
    }
} 

/*
pub mod elements {
    pub const ANGLE: i32 = 0;
    pub const STROKE_COLOR: &str = "#000000";
    pub const BACKGROUND_COLOR: &str = "transparent";
    pub const FILL_STYLE: &str = "hachure";
    pub const STROKE_WIDTH: i32 = 1;
    pub const STROKE_STYLE: &str = "solid";
    pub const CONNECTION_STYLE: &str = "dashed";
    pub const OPACITY: i32 = 100;
    pub const STROKE_SHARPNESS: &str = "sharp";
    pub const FONT_SIZE_SMALL: i32 = 16;
    pub const FONT_SIZE_MEDIUM: i32 = 20;
    pub const FONT_SIZE_LARGE: i32 = 28;
    pub const FONT_SIZE_EXTRA_LARGE: i32 = 36;
    pub const TEXT_ALIGN_LEFT: &str = "left";
    pub const VERTICAL_ALIGN_TOP: &str = "top";
}
 */