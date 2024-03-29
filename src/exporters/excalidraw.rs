use serde::Serialize;
use serde_json::{Map, Value};

use super::excalidraw_config::{consts::NON_LOCKED, BoundElement, Roundness};
use crate::exporters::excalidraw_config::{roundness, Binding};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcalidrawFile {
    pub r#type: String,
    pub version: i32,
    pub source: Option<String>,
    pub elements: Vec<Element>,
    pub app_state: AppState,
    pub files: Map<String, Value>,
}

impl Default for ExcalidrawFile {
    fn default() -> Self {
        Self {
            r#type: "excalidraw".into(),
            version: 2,
            source: None,
            elements: Vec::with_capacity(0),
            app_state: Default::default(),
            files: Map::with_capacity(0),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Element {
    #[serde(rename_all = "camelCase")]
    Text {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        group_ids: Vec<String>,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        roughness: i32,
        opacity: i32,
        stroke_sharpness: String,
        locked: bool,
        text: String,
        font_size: i32,
        font_family: i32,
        text_align: String,
        vertical_align: String,
        baseline: i32,
    },
    #[serde(rename_all = "camelCase")]
    Arrow {
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        roundness: Option<Roundness>,
        roughness: i32,
        opacity: i32,
        start_binding: Binding,
        end_binding: Binding,
        stroke_sharpness: String,
        locked: bool,
        points: Vec<[i32; 2]>,
    },
    #[serde(rename_all = "camelCase")]
    Rectangle {
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        group_ids: Vec<String>,
        bound_elements: Vec<BoundElement>,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        roughness: i32,
        roundness: Option<Roundness>,
        opacity: i32,
        stroke_sharpness: String,
        locked: bool,
    },
    #[serde(rename_all = "camelCase")]
    Ellipse {
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        group_ids: Vec<String>,
        bound_elements: Vec<BoundElement>,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        roughness: i32,
        opacity: i32,
        stroke_sharpness: String,
        locked: bool,
    },
}

pub mod elements {
    pub const ANGLE: i32 = 0;
    pub const STROKE_COLOR: &str = "#000000";
    pub const NETWORK_COLOR: &str = "#f2f0e6";
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

#[allow(clippy::too_many_arguments)]
impl Element {
    pub fn text(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        group_ids: Vec<String>,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        opacity: i32,
        stroke_sharpness: String,
        text: String,
        font_size: i32,
        font_family: i32,
        text_align: String,
        vertical_align: String,
    ) -> Self {
        Self::Text {
            x,
            y,
            width,
            height,
            group_ids,
            angle,
            stroke_color,
            background_color,
            fill_style,
            stroke_width,
            stroke_style,
            roughness: 0,
            opacity,
            stroke_sharpness,
            locked: NON_LOCKED,
            text,
            font_size,
            font_family,
            text_align,
            vertical_align,
            baseline: 15,
        }
    }

    pub fn arrow(
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        start_binding: Binding,
        end_binding: Binding,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        roundness: Option<Roundness>,
        opacity: i32,
        stroke_sharpness: String,
        points: Vec<[i32; 2]>,
    ) -> Self {
        Self::Arrow {
            id,
            x,
            y,
            width,
            height,
            start_binding,
            end_binding,
            angle,
            stroke_color,
            background_color,
            fill_style,
            stroke_width,
            stroke_style,
            roundness,
            roughness: 2, // roughness: 0
            opacity,
            stroke_sharpness,
            locked: NON_LOCKED,
            points,
        }
    }

    pub fn rectangle(
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        group_ids: Vec<String>,
        bound_elements: Vec<BoundElement>,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        roundness: Option<Roundness>,
        opacity: i32,
        stroke_sharpness: String,
    ) -> Self {
        Self::Rectangle {
            id,
            x,
            y,
            width,
            height,
            group_ids,
            bound_elements,
            angle,
            stroke_color,
            background_color,
            fill_style,
            stroke_width,
            stroke_style,
            roughness: 2, // roughness: 0, - strict
            roundness,
            opacity,
            stroke_sharpness,
            locked: NON_LOCKED,
        }
    }

    pub fn ellipse(
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        group_ids: Vec<String>,
        bound_elements: Vec<BoundElement>,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        opacity: i32,
        stroke_sharpness: String,
    ) -> Self {
        Self::Ellipse {
            id,
            x,
            y,
            width,
            height,
            group_ids,
            bound_elements,
            angle,
            stroke_color,
            background_color,
            fill_style,
            stroke_width,
            stroke_style,
            roughness: 1, // roughness: 0
            opacity,
            stroke_sharpness,
            locked: NON_LOCKED,
        }
    }

    pub fn draw_ellipse(
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        group_ids: Vec<String>,
        bound_elements: Vec<BoundElement>,
        background_color: String,
        fill_style: String,
    ) -> Self {
        Self::ellipse(
            id,
            x,
            y,
            width,
            height,
            group_ids,
            bound_elements,
            elements::ANGLE,
            elements::STROKE_COLOR.into(),
            background_color, //elements::BACKGROUND_COLOR.into(),
            fill_style,       //elements::FILL_STYLE.into(),
            elements::STROKE_WIDTH,
            elements::STROKE_STYLE.into(),
            elements::OPACITY,
            elements::STROKE_SHARPNESS.into(),
        )
    }

    pub fn draw_small_monospaced_text(
        text: String,
        x: i32,
        y: i32,
        group_ids: Vec<String>,
        font_size: i32,
        font_family: i32,
    ) -> Self {
        Self::text(
            x,
            y,
            (4 + text.chars().count() * 18) as i32,
            (text.lines().count() * 19) as i32,
            group_ids,
            0,
            elements::STROKE_COLOR.into(),
            elements::BACKGROUND_COLOR.into(),
            elements::FILL_STYLE.into(),
            elements::STROKE_WIDTH,
            elements::STROKE_STYLE.into(),
            elements::OPACITY,
            elements::STROKE_SHARPNESS.into(),
            text,
            font_size,   //elements::FONT_SIZE_SMALL,
            font_family, //elements::FONT_FAMILY_MONOSPACE,
            elements::TEXT_ALIGN_LEFT.into(),
            elements::VERTICAL_ALIGN_TOP.into(),
        )
    }

    pub fn simple_arrow(
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        stroke_style: String,
        edge: String,
        points: Vec<[i32; 2]>,
        start_binding: Binding,
        end_binding: Binding,
    ) -> Self {
        Self::arrow(
            id,
            x,
            y,
            width, // TODO
            height,
            start_binding,
            end_binding,
            elements::ANGLE,
            elements::STROKE_COLOR.into(),
            elements::BACKGROUND_COLOR.into(),
            elements::FILL_STYLE.into(),
            elements::STROKE_WIDTH,
            stroke_style,
            roundness(edge),
            elements::OPACITY,
            elements::STROKE_SHARPNESS.into(),
            points,
        )
    }

    pub fn simple_rectangle(
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        group_ids: Vec<String>,
        bound_elements: Vec<BoundElement>,
        background_color: String,
        fill_style: String,
        stroke_style: String,
        edge: String,
    ) -> Self {
        Self::rectangle(
            id,
            x,
            y,
            width,
            height,
            group_ids,
            bound_elements,
            elements::ANGLE,
            elements::STROKE_COLOR.into(),
            background_color, //elements::BACKGROUND_COLOR.into(),
            fill_style,       //elements::FILL_STYLE.into(),
            elements::STROKE_WIDTH,
            stroke_style,
            roundness(edge),
            elements::OPACITY,
            elements::STROKE_SHARPNESS.into(),
        )
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub grid_size: i32,
    pub view_background_color: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            grid_size: 20,
            view_background_color: "#ffffff".into(),
        }
    }
}
