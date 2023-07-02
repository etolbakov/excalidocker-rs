use crate::exporters::excalidraw_config::consts::{
    NO_X_ALIGNMENT_FACTOR, NO_X_MARGIN, NO_Y_ALIGNMENT_FACTOR, NO_Y_MARGIN, X_ALIGNMENT_FACTOR,
    X_MARGIN, Y_ALIGNMENT_FACTOR, Y_MARGIN,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExcalidrawConfig {
    pub font: Font,
    pub services: Services,
    pub ports: Ports,
    pub connections: Connections,
    pub alignment: Alignment,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Font {
    pub size: i32,
    pub family: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Services {
    pub background_color: String,
    pub fill: String,
    pub edge: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ports {
    pub background_color: String,
    pub fill: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connections {
    pub visible: bool,
    pub edge: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub mod consts {
    pub const NO_X_MARGIN: i32 = 0;
    pub const NO_Y_MARGIN: i32 = 0;
    pub const X_MARGIN: i32 = 60;
    pub const Y_MARGIN: i32 = 60;
    pub const X_ALIGNMENT_FACTOR: i32 = 1;
    pub const NO_X_ALIGNMENT_FACTOR: i32 = 0;
    pub const Y_ALIGNMENT_FACTOR: i32 = 1;
    pub const NO_Y_ALIGNMENT_FACTOR: i32 = 0;
}

/// Based on the previous implementation it was observed
/// for 'horizontal' and 'stepped' alignment
/// x += x_margin + container_width;
/// y += y_margin;
///
/// and for 'vertical' alignment
/// x += x_margin;
/// y += y_margin + scale;
pub fn margins(alignment_mode: &str) -> (i32, i32, i32, i32) {
    match alignment_mode {
        "horizontal" => (
            X_MARGIN,
            NO_Y_MARGIN,
            X_ALIGNMENT_FACTOR,
            NO_Y_ALIGNMENT_FACTOR,
        ),
        "vertical" => (
            NO_X_MARGIN,
            Y_MARGIN,
            NO_X_ALIGNMENT_FACTOR,
            Y_ALIGNMENT_FACTOR,
        ),
        _ => (
            X_MARGIN,
            Y_MARGIN,
            X_ALIGNMENT_FACTOR,
            NO_Y_ALIGNMENT_FACTOR,
        ), // "stepped" is default
    }
}
