mod error;
mod exporters;
mod file_utils;

use clap::{arg, command, Parser};
use exporters::excalidraw::elements::{
    FONT_SIZE_EXTRA_LARGE, FONT_SIZE_LARGE, FONT_SIZE_MEDIUM, FONT_SIZE_SMALL,
};
use exporters::excalidraw_config::{
    arrow_bounded_element, binding, BoundElement, DEFAULT_CONFIG_PATH,
};
use exporters::excalidraw_config::{margins, ExcalidrawConfig};
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::vec;

use exporters::excalidraw::{Element, ExcalidrawFile};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::error::ExcalidockerError::InvalidDockerCompose;
use crate::exporters::excalidraw::elements;

#[derive(Parser)]
#[command(name = clap::crate_name!())]
#[command(author = clap::crate_authors!())]
#[command(version = clap::crate_version!())]
#[command(about = clap::crate_description!(), long_about = None)]
#[command(override_usage(format!(
    "
{} {}
    ╰→ excalidocker --input-path <INPUT_PATH>
    ╰→ excalidocker --show-config",
    clap::crate_name!(),
    clap::crate_version!())
))]
struct Cli {
    /// show configuration file
    #[arg(short = 'C', long, default_value_t = false)]
    show_config: bool,
    /// file path to the docker-compose.yaml
    #[arg(short, long, required_unless_present = "show_config")]
    input_path: Option<String>,
    /// display connecting lines between services; if `true` then only service without the lines are rendered
    #[arg(short, long, default_value_t = false)]
    skip_dependencies: bool,
    /// file path for the output excalidraw file.
    /// By default the file content is sent to console output
    #[arg(short, long)]
    output_path: Option<String>,
    /// config file path for the excalidraw.
    #[arg(short, long, default_value_t = DEFAULT_CONFIG_PATH.to_string())]
    config_path: String,
}

#[derive(Debug, Clone)]
struct ContainerPoint(String, i32, i32);

impl ContainerPoint {
    fn new(name: String, x: i32, y: i32) -> Self {
        Self(name, x, y)
    }
    // fn set_name(mut self, name: String) -> Self {
    //     self.0 = name;
    //     self
    // }
}

#[derive(Debug, Clone)]
struct DependencyComponent {
    id: String,
    name: String,
    parent: Vec<DependencyComponent>,
}

impl DependencyComponent {
    fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            parent: Vec::new(),
        }
    }
}

fn traverse_in_hierarchy(
    name: &str,
    dependencies: &HashMap<&str, DependencyComponent>,
    containers_traversal_order: &mut Vec<String>,
    visited: &mut HashSet<String>,
) {
    if let Some(children) = dependencies.get(name) {
        for child in &children.parent {
            if !visited.contains(&child.name.to_string()) {
                traverse_in_hierarchy(
                    child.name.as_str(),
                    dependencies,
                    containers_traversal_order,
                    visited,
                );
            }
        }
    }

    if !visited.contains(name) {
        containers_traversal_order.push(name.to_string());
        visited.insert(name.to_string());
    }
}

// fn build_hierarchy(containers: &HashMap<String, Container>) -> HashMap<String, Vec<String>> {
//     let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();

//     for (name, container) in containers {
//         for dependency in &container.depends_on {
//             dependencies
//                 .entry(dependency.clone())
//                 .or_insert_with(Vec::new)
//                 .push(name.clone());
//         }
//     }

/// This struct is introduced to hold intermediate state of the rectange
/// Due to the implementation logic the rectangle initialization (`x`, `y`, `width`, `height`)
/// is happening in the beginning of the program while `group_ids` and `bound_elements`
/// could be added/updated later.
#[derive(Debug, Clone)]
struct RectangleStruct {
    pub id: String,
    pub container_name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub group_ids: Vec<String>,
    pub text_group_ids: Vec<String>,
    pub bound_elements: Vec<BoundElement>,
}

fn main() {
    let cli = Cli::parse();
    let excalidraw_config: ExcalidrawConfig =
        file_utils::get_excalidraw_config(cli.config_path.as_str());
    if cli.show_config {
        println!("{}", serde_yaml::to_string(&excalidraw_config).unwrap());
        return;
    }

    let mut excalidraw_file = ExcalidrawFile::default();
    let scale = excalidraw_file.app_state.grid_size;
    let mut x = 0;
    let mut y = 0;
    let width = 140;
    let height = 60;
    let port_diameter = 60;

    let mut components = Vec::new();
    let mut container_name_rectangle_structs = HashMap::new();
    let mut container_name_to_point = HashMap::new();
    let mut container_name_to_parents: HashMap<&str, DependencyComponent> = HashMap::new();
    let mut container_name_to_container_struct = HashMap::new();

    let input_path = &cli.input_path.unwrap();
    let input_filepath = input_path.as_str();
    let docker_compose_yaml = file_utils::get_docker_compose_content(input_filepath);

    let alignment_mode = excalidraw_config.alignment.mode.as_str();
    let (x_margin, y_margin, x_alignment_factor, y_alignment_factor) = margins(alignment_mode);

    let services = match docker_compose_yaml.get("services") {
        Some(services) => services,
        None => {
            println!(
                "{}",
                InvalidDockerCompose {
                    path: input_filepath.to_string(),
                    msg: "Failed to get 'services' attribute".to_string()
                }
            );
            return;
        }
    };

    let _networks = docker_compose_yaml
        .get("networks")
        .and_then(DockerContainer::parse_networks);
    // dbg!(networks);

    let mut identifier: i32 = 1;
    for (container_name_val, container_data_val) in services.as_mapping().unwrap() {
        let container_id = format!("container_{}", identifier);

        let container_struct =
            DockerContainer::convert_to_container(container_id.clone(), container_data_val);
        let container_name_str = container_name_val.as_str().unwrap();

        let mut dependency_component =
            DependencyComponent::new(container_id, container_name_str.to_string());
        if let Some(dependencies) = &container_struct.depends_on {
            dependencies.iter().for_each(|name| {
                dependency_component
                    .parent
                    .push(DependencyComponent::new("".to_string(), name.to_string()))
            });
        }
        components.push(dependency_component.clone());
        container_name_to_parents.insert(container_name_str, dependency_component);
        container_name_to_container_struct.insert(container_name_str, container_struct);
        identifier += 1;
    }

    let containers_traversal_order = find_containers_traversal_order(container_name_to_parents);

    for cn_name in containers_traversal_order {
        let container_width =
            width + find_additional_width(cn_name.as_str(), &scale, &excalidraw_config.font.size);
        let container_struct = container_name_to_container_struct
            .get(cn_name.as_str())
            .unwrap();
        container_name_to_point.insert(cn_name.clone(), ContainerPoint::new(cn_name.clone(), x, y));

        // ------------ Define container ------------
        let container_group = vec![format!("container_group_{}", generate_id())];

        let mut rectangle_struct = RectangleStruct {
            id: container_struct.id.clone(),
            container_name: cn_name.clone(),
            x,
            y,
            width: container_width,
            height,
            group_ids: container_group.clone(),
            text_group_ids: container_group.clone(),
            bound_elements: vec![],
        };

        // ------------ Define ports ------------
        let ports = container_struct.clone().ports.unwrap_or(Vec::new());
        for (i, port) in ports.iter().enumerate() {
            let i = i as i32;
            let (container_adjustment_x, container_adjustment_y) =
                get_container_xy(alignment_mode, &container_width, &scale, i);
            let container_x = x + container_adjustment_x;
            let container_y = y + container_adjustment_y;

            let (host_port_str, container_port_str) = extract_host_container_ports(port);
            let ellipse_port_group = vec![format!("group_{}_hostport_{}_text", cn_name, i)];

            let ellipse_host_port_id = format!("ellipse_{}", generate_id());
            let host_port_arrow_id = format!("port_arrow_{}", generate_id());

            let host_port = Element::draw_ellipse(
                ellipse_host_port_id.clone(),
                container_x,
                container_y,
                port_diameter,
                port_diameter,
                ellipse_port_group.clone(),
                vec![arrow_bounded_element(host_port_arrow_id.clone())],
                excalidraw_config.ports.background_color.clone(),
                excalidraw_config.ports.fill.clone(),
            );
            let host_port_text = Element::draw_small_monospaced_text(
                host_port_str.clone(),
                container_x + 15,
                container_y + 20,
                ellipse_port_group.clone(),
                excalidraw_config.font.size,
                excalidraw_config.font.family,
            );

            let (host_port_arrow_x, host_port_arrow_y) =
                get_host_port_arrow_xy(alignment_mode, &height, &width, &container_width);
            let host_port_arrow = Element::simple_arrow(
                host_port_arrow_id.clone(),
                x + host_port_arrow_x,
                y + host_port_arrow_y,
                200,
                100,
                elements::STROKE_STYLE.into(),
                "sharp".to_string(),
                get_host_port_arrow_points(alignment_mode, i),
                binding(container_struct.id.clone()),
                binding(ellipse_host_port_id),
            );

            // bind the port arrow to the container
            rectangle_struct
                .bound_elements
                .push(arrow_bounded_element(host_port_arrow_id.to_string()));

            if host_port_str != container_port_str {
                let (container_port_text_x, container_port_text_y) =
                    get_container_port_text_xy(alignment_mode, &height, &width, i);
                let container_port_text = Element::draw_small_monospaced_text(
                    container_port_str,
                    x + container_port_text_x,
                    y + container_port_text_y,
                    container_group.clone(),
                    excalidraw_config.font.size,
                    excalidraw_config.font.family,
                );
                excalidraw_file.elements.push(container_port_text);
            }
            excalidraw_file.elements.push(host_port);
            excalidraw_file.elements.push(host_port_text);
            excalidraw_file.elements.push(host_port_arrow);
        }

        // ------------ Define Alignment ------------
        let (x_alignment, y_alignment) = get_alignment_factor_xy(
            alignment_mode,
            x_alignment_factor,
            y_alignment_factor,
            container_width,
            scale,
        );
        x += x_margin + x_alignment;
        y += y_margin + y_alignment;

        container_name_rectangle_structs.insert(cn_name, rectangle_struct);
    }

    for DependencyComponent { id, name, parent } in &components {
        let ContainerPoint(_, x, y) = container_name_to_point.get(name).unwrap();
        // any of those two conditions (cli argument or configuration setting) can switch off the connections
        let sorted_container_points =
            if cli.skip_dependencies || !excalidraw_config.connections.visible {
                Vec::<ContainerPoint>::new()
            } else {
                let mut points = parent
                    .iter()
                    .map(|dc| {
                        let cp = container_name_to_point.get(&dc.name).unwrap();
                        ContainerPoint::new(dc.name.clone(), cp.1, cp.2)
                    })
                    .collect::<Vec<ContainerPoint>>();
                points.sort_by(|cp1, cp2| cp2.1.cmp(&cp1.1));
                points
            };

        for (i, parent_point) in sorted_container_points.iter().enumerate() {
            let i = i as i32;
            let parent_name = &parent_point.0;
            let parent_temp_struct = container_name_rectangle_structs
                .get_mut(parent_name)
                .unwrap();

            let x_parent = &parent_point.1;
            let y_parent = &parent_point.2;
            let level_height = y_parent - y;
            let interation_x_margin = (i + 1) * scale;

            let connecting_arrow_points = get_connecting_arrow_points(
                alignment_mode,
                x,
                y,
                x_parent,
                y_parent,
                &height,
                &width,
                &interation_x_margin,
                &scale,
                level_height,
                i,
            );
            let connecting_arrow_id = format!("connecting_arrow_{}", generate_id());
            let (connecting_arrow_x, connecting_arrow_y) =
                get_connecting_arrow_xy(alignment_mode, interation_x_margin);
            let connecting_arrow = Element::simple_arrow(
                connecting_arrow_id.clone(),
                x + connecting_arrow_x,
                y + connecting_arrow_y,
                0,
                y_margin,
                elements::CONNECTION_STYLE.into(),
                excalidraw_config.connections.edge.clone(),
                connecting_arrow_points,
                binding(id.to_string()),                // child container
                binding(parent_temp_struct.id.clone()), // parent container
            );

            // for dependency connection we need to add:
            // - child container id to the binding
            // - parent container id to the binding
            // - boundElements for the child container (id of the connecting_arrow)
            // - boundElements for the parent container (id of the connecting_arrow)

            let connecting_arrow_bound = arrow_bounded_element(connecting_arrow_id);
            parent_temp_struct
                .bound_elements
                .push(connecting_arrow_bound.clone());
            let current_temp_struct = container_name_rectangle_structs.get_mut(name).unwrap();
            current_temp_struct
                .bound_elements
                .push(connecting_arrow_bound);
            excalidraw_file.elements.push(connecting_arrow);
        }
    }

    container_name_rectangle_structs.values().for_each(|rect| {
        let container_rectangle = Element::simple_rectangle(
            rect.id.clone(),
            rect.x,
            rect.y,
            rect.width,
            rect.height,
            rect.group_ids.clone(),
            rect.bound_elements.clone(),
            excalidraw_config.services.background_color.clone(),
            excalidraw_config.services.fill.clone(),
            excalidraw_config.services.edge.clone(),
        );
        let container_text = Element::draw_small_monospaced_text(
            rect.container_name.clone(),
            rect.x + scale,
            rect.y + scale,
            rect.text_group_ids.clone(),
            excalidraw_config.font.size,
            excalidraw_config.font.family,
        );
        excalidraw_file.elements.push(container_rectangle);
        excalidraw_file.elements.push(container_text);
    });
    let excalidraw_data = serde_json::to_string(&excalidraw_file).unwrap();
    match cli.output_path {
        Some(output_file_path) => {
            fs::write(output_file_path.clone(), excalidraw_data).expect("Unable to write file");
            println!("\nConfiguration file : '{}'", cli.config_path.as_str());
            println!("\nInput file : '{}'", input_filepath);
            println!(
                "\nExcalidraw file is successfully generated and can be found at '{}'\n",
                output_file_path
            );
        }
        None => println!("{}", excalidraw_data),
    }
}

fn get_connecting_arrow_xy(alignment_mode: &str, interation_margin: i32) -> (i32, i32) {
    if alignment_mode == "vertical" {
        (0, interation_margin / 2)
    } else {
        (interation_margin, 0)
    }
}

#[allow(clippy::too_many_arguments)]
fn get_connecting_arrow_points(
    alignment_mode: &str,
    x: &i32,
    y: &i32,
    x_parent: &i32,
    y_parent: &i32,
    height: &i32,
    width: &i32,
    interation_x_margin: &i32,
    scale: &i32,
    level_height: i32,
    i: i32,
) -> Vec<[i32; 2]> {
    if alignment_mode == "vertical" {
        vec![
            [0, 0],
            [-2 * (i + 1) * scale, 0],
            [
                -2 * (i + 1) * scale,
                // level_height
                level_height + scale,
            ],
            [
                -1,
                // level_height
                level_height + scale,
            ],
        ]
    } else {
        vec![
            [0, 0],
            [0, level_height - height],
            [
                -x + x_parent + width - interation_x_margin * 2,
                level_height - height,
            ],
            [
                -x + x_parent + width - interation_x_margin * 2,
                y_parent - y,
            ],
        ]
    }
}

fn get_alignment_factor_xy(
    alignment_mode: &str,
    x_alignment_factor: i32,
    y_alignment_factor: i32,
    container_width: i32,
    scale: i32,
) -> (i32, i32) {
    (
        x_alignment_factor * container_width,
        if alignment_mode == "vertical" {
            y_alignment_factor * 2 * scale // TODO should we increase the step or make it configurable??
        } else {
            y_alignment_factor * scale
        },
    )
}

fn get_container_port_text_xy(
    alignment_mode: &str,
    height: &i32,
    width: &i32,
    i: i32,
) -> (i32, i32) {
    if alignment_mode == "vertical" {
        (width + 20, height / 2 + (i * 40) - 35)
    } else {
        (20 + i * 80, 80)
    }
}

fn get_host_port_arrow_points(alignment_mode: &str, i: i32) -> Vec<[i32; 2]> {
    if alignment_mode == "vertical" {
        vec![[0, 0], [i + 100, i * 80 - 35]]
    } else {
        vec![[0, 0], [i * 80 - 35, i + 100]]
    }
}

fn get_host_port_arrow_xy(
    alignment_mode: &str,
    height: &i32,
    width: &i32,
    container_width: &i32,
) -> (i32, i32) {
    if alignment_mode == "vertical" {
        (*container_width, height / 2)
    } else {
        (width / 2, *height)
    }
}

fn get_container_xy(alignment_mode: &str, width: &i32, scale: &i32, i: i32) -> (i32, i32) {
    if alignment_mode == "vertical" {
        (*width + scale * 5, i * 80 - 35)
    } else {
        (i * 80, scale * 8)
    }
}

/// There are several ways to declare ports:
///  - "0" single port value(range of values): a container port(range) will be assigned to random host port(range)
///  - "1" colon separated values (range of values): container port (range) is assigned to given host port (range)
///  - "_" detailed declaration which may include `host_ip`, `protocol` etc
fn extract_host_container_ports(port: &str) -> (String, String) {
    let port_parts: Vec<_> = port.rmatch_indices(':').collect();
    let port_string = port.to_string();
    match port_parts.len() {
        0 => (port_string.clone(), port_string),
        1 => {
            let split = port.split(':').collect::<Vec<&str>>();
            (split[0].to_string(), split[1].to_string())
        }
        _ => {
            let colon_index = port_parts.first().unwrap().0;
            (
                port_string.chars().take(colon_index).collect(),
                port_string.chars().skip(colon_index + 1).collect(),
            )
        }
    }
}

fn find_containers_traversal_order(
    container_name_to_parents: HashMap<&str, DependencyComponent>,
) -> Vec<String> {
    let mut containers_traversal_order: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    for name in container_name_to_parents.keys() {
        traverse_in_hierarchy(
            name,
            &container_name_to_parents,
            &mut containers_traversal_order,
            &mut visited,
        );
    }
    // Vec::from_iter(visited)
    containers_traversal_order
}

/// According to current `exc.app_state.grid_size` setting and text/font size
/// it's possible to accommodate approximately 3 letters in one grid item.
/// The container width is 7 grid items(140) in total and uses only 5 grid items
/// to accommodate the text up to 14 characters(`max_container_name_len`)
/// Empirically found that for
///  20 | 1.5 letters in grid
///  28 | 1   letter in grid
///  36 | 1   letter in grid
fn find_additional_width(container_name: &str, scale: &i32, font_size: &i32) -> i32 {
    let container_name_len = container_name.len();
    let (container_name_len_max, elements_per_item_grid) = match *font_size {
        FONT_SIZE_SMALL => (14, 3),
        FONT_SIZE_MEDIUM => (9, 2),
        FONT_SIZE_LARGE => (5, 1),
        FONT_SIZE_EXTRA_LARGE => (2, 1),
        _ => (1, 1),
    };
    let text_accommodation_len_default = 5;
    let text_accommodation_margin = 3;
    if container_name_len > container_name_len_max {
        let required_space_for_text = ((container_name_len / elements_per_item_grid)
            - text_accommodation_len_default
            + text_accommodation_margin) as i32;
        scale * required_space_for_text
    } else {
        0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DockerContainer {
    pub id: String,
    image: String,
    command: Option<String>,
    environment: Option<HashMap<String, String>>,
    depends_on: Option<Vec<String>>,
    ports: Option<Vec<String>>, // HOST:CONTAINER
    volumes: Option<Vec<String>>,
    networks: Option<Vec<String>>,
    // TODO: add other fields
}

impl DockerContainer {
    fn new(id: String) -> Self {
        Self {
            id,
            image: String::new(),
            command: None,
            environment: None,
            ports: None,
            volumes: None,
            depends_on: None,
            networks: None,
        }
    }

    fn parse_depends_on(value: &Value) -> Option<Vec<String>> {
        match value {
            Value::Sequence(depends_on) => {
                let depends_on_vec: Vec<String> = depends_on
                    .iter()
                    .filter_map(|item| item.as_str().map(|s| s.to_string()))
                    .collect();
                Some(depends_on_vec)
            }
            Value::Mapping(depends_on) => {
                let depends_on_vec: Vec<String> = depends_on
                    .keys()
                    .filter_map(|key| key.as_str().map(|s| s.to_string()))
                    .collect();
                Some(depends_on_vec)
            }
            _ => None,
        }
    }

    fn parse_networks(value: &Value) -> Option<Vec<String>> {
        match value {
            Value::Sequence(networks) => {
                let networks_strings: Vec<String> = networks
                    .iter()
                    .filter_map(|network| network.as_str().map(|nw| nw.to_string()))
                    .collect();
                Some(networks_strings)
            }
            Value::Mapping(networks) => {
                let networks_vec: Vec<String> = networks
                    .keys()
                    .filter_map(|key| key.as_str().map(|s| s.to_string()))
                    .collect();
                Some(networks_vec)
            }
            _ => None,
        }
    }

    fn convert_to_container(id: String, value: &Value) -> Self {
        let mapping = value.as_mapping().unwrap();
        let mut container = DockerContainer::new(id);
        for (key, value) in mapping {
            let key_str = key.as_str().unwrap();
            match key_str {
                "image" => {
                    if let Value::String(image) = value {
                        container.image = image.clone();
                    }
                }
                "command" => {
                    if let Value::String(command) = value {
                        container.command = Some(command.clone());
                    }
                }
                "environment" => {
                    if let Value::Mapping(environment) = value {
                        let mut env_map = HashMap::new();
                        for (env_key, env_value) in environment {
                            if let (Value::String(key), Value::String(value)) = (env_key, env_value)
                            {
                                env_map.insert(key.clone(), value.clone());
                            }
                        }
                        container.environment = Some(env_map);
                    }
                }
                "ports" => {
                    if let Value::Sequence(ports) = value {
                        let port_strings = ports
                            .iter()
                            .filter_map(|port| port.as_str().map(|p| p.to_string()))
                            .collect();
                        container.ports = Some(port_strings);
                    }
                }
                "volumes" => {
                    if let Value::Sequence(volumes) = value {
                        let volume_strings = volumes
                            .iter()
                            .filter_map(|volume| volume.as_str().map(|v| v.to_string()))
                            .collect();
                        container.volumes = Some(volume_strings);
                    }
                }
                "depends_on" => {
                    if let Some(depends_on) = Self::parse_depends_on(value) {
                        container.depends_on = Some(depends_on);
                    }
                }
                "networks" => {
                    if let Some(networks) = Self::parse_networks(value) {
                        container.networks = Some(networks);
                    }
                }
                // TODO: Handle other fields
                _ => (),
            }
        }
        container
    }
}

// fn parse_depends_on(value: Value) -> Option<Vec<String>> {
//     match value {
//         Value::Sequence(depends_on) => {
//             let depends_on_vec: Vec<String> = depends_on
//                 .iter()
//                 .filter_map(|item| item.as_str().map(|s| s.to_string()))
//                 .collect();
//             Some(depends_on_vec)
//         }
//         Value::Mapping(depends_on) => {
//             let depends_on_vec: Vec<String> = depends_on
//                 .keys()
//                 .filter_map(|key| key.as_str().map(|s| s.to_string()))
//                 .collect();
//             Some(depends_on_vec)
//         }
//         _ => None,
//     }
// }

// fn parse_networks(value: &Value) -> Option<Vec<String>> {
//     match value {
//         Value::Sequence(networks) => {
//             let networks_strings: Vec<String> = networks
//                 .iter()
//                 .filter_map(|network| network.as_str().map(|nw| nw.to_string()))
//                 .collect();
//             Some(networks_strings)
//         }
//         Value::Mapping(networks) => {
//             let networks_vec: Vec<String> = networks
//                 .keys()
//                 .filter_map(|key| key.as_str().map(|s| s.to_string()))
//                 .collect();
//             Some(networks_vec)
//         }
//         _ => None
//     }
// }

fn generate_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

// #[test]
// fn check_parsing() {
//
// }

#[test]
fn test_check_port_parsing() {
    // - "3000"                 # container port (3000), assigned to random host port
    let (host_port, container_port) = extract_host_container_ports("3000");
    assert_eq!(host_port, "3000");
    assert_eq!(container_port, "3000");

    // - "3001-3005"            # container port range (3001-3005), assigned to random host ports
    let (host_port, container_port) = extract_host_container_ports("3001-3005");
    assert_eq!(host_port, "3001-3005");
    assert_eq!(container_port, "3001-3005");

    // - "8001:8001"            # container port (8001), assigned to given host port (8001)
    let (host_port, container_port) = extract_host_container_ports("8001:8001");
    assert_eq!(host_port, "8001");
    assert_eq!(container_port, "8001");

    // - "9090-9091:8080-8081"  # container port range (8080-8081), assigned to given host port range (9090-9091)
    let (host_port, container_port) = extract_host_container_ports("9090-9091:8080-8081");
    assert_eq!(host_port, "9090-9091");
    assert_eq!(container_port, "8080-8081");

    // - "127.0.0.1:8002:8002"  # container port (8002), assigned to given host port (8002) and bind to 127.0.0.1
    let (host_port, container_port) = extract_host_container_ports("127.0.0.1:8002:8002");
    assert_eq!(host_port, "127.0.0.1:8002");
    assert_eq!(container_port, "8002");

    // - "6060:6060/udp"        # container port (6060) restricted to UDP protocol, assigned to given host (6060)
    let (host_port, container_port) = extract_host_container_ports("6060:6060/udp");
    assert_eq!(host_port, "6060");
    assert_eq!(container_port, "6060/udp");
}
