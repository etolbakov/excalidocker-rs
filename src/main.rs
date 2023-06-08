mod exporters;
mod error;

use clap::{Parser, arg, command};
use exporters::excalidraw::BoundElement;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::vec;

use serde::{Serialize, Deserialize};
use exporters::excalidraw::{ExcalidrawFile, Element};
use serde_yaml::Value;

use crate::error::ExcalidockerError::{self,
    InvalidDockerCompose, FileIncorrectExtension, 
    FileNotFound, FileFailedRead, FileFailedParsing
};
use crate::exporters::excalidraw::Binding;
use crate::exporters::excalidraw::elements;

#[derive(Parser)]
#[command(name = "Excalidocker")]
#[command(author = "Evgeny Tolbakov <ev.tolbakov@gmail.com>")]
#[command(version = "0.1.4")]
#[command(about = "Utility to convert docker-compose into excalidraw", long_about = None)]
struct Cli {
    /// file path to the docker-compose.yaml
    #[arg(short, long)]
    input_path: String,
    /// display connecting lines between services; if `true` then only service without the lines are rendered
    #[arg(short, long, default_value_t = false)]
    skip_dependencies: bool,
    /// file path for the output excalidraw file.
    /// By default the file content is sent to console output
    #[arg(short, long)]
    output_path: Option<String>,
}

#[derive(Debug,Clone)]
struct ContainerPoint(String, i32, i32);

impl ContainerPoint {
    fn new(name: String, x: i32, y: i32) -> Self {
        Self(name, x, y)
    } 
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
                traverse_in_hierarchy(child.name.as_str(), dependencies, containers_traversal_order, visited);
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

#[derive(Debug, Clone)]
struct RectangleTempStruct {
    pub id: String, 
    pub x: i32, 
    pub y: i32, 
    pub width: i32, 
    pub height: i32, 
    pub group_ids: Vec<String>, 
    pub bound_elements: Vec<BoundElement>,
}
   

fn main() {
    let cli = Cli::parse();
    let mut excalidraw_file = ExcalidrawFile::default();
    let scale = excalidraw_file.app_state.grid_size;
    let mut x = 0;
    let mut y = 0;
    let width = 140;
    let height = 60;
    let port_diameter = 60;
    let locked = false;
    
    let x_margin = 60;
    let y_margin = 60;

    let mut components = Vec::new();
    let mut container_name_rectangle_temp_structs = HashMap::new();
    let mut container_name_to_point = HashMap::new();
    let mut container_name_to_parents: HashMap<&str, DependencyComponent> = HashMap::new();
    let mut container_name_to_container_struct = HashMap::new();
    // let mut container_name_to_excalidraw_element = HashMap::new();

    let input_filepath = cli.input_path.as_str();
    let docker_compose_yaml = match parse_yaml_file(input_filepath) {
        Ok(yaml_content) => yaml_content,
        Err(err) => {
            println!("{}", err);
            return;
        },
    };
    let services = match docker_compose_yaml.get("services") {
        Some(services) => services,
        None => {
            println!("{}", InvalidDockerCompose { 
                path: input_filepath.to_string(), 
                msg: "Failed to get 'services' attribute".to_string()
            });
            return;
        }
    }; 
    let mut identifier: i32 = 1;
    for (container_name_val, container_data_val) in services.as_mapping().unwrap() {
        let container_id = format!("container_{}", identifier);
        let container_struct = convert_to_container(container_id.clone(), container_data_val).unwrap();
        let container_name_str = container_name_val.as_str().unwrap();

        let mut dependency_component = DependencyComponent::new(container_id, container_name_str.to_string());                
        if let Some(dependencies) = &container_struct.depends_on {
            dependencies
            .iter()
            .for_each(|name|
                dependency_component.parent.push(DependencyComponent::new("".to_string(), name.to_string()))
            );
        } 
        components.push(dependency_component.clone());
        container_name_to_parents.insert(container_name_str, dependency_component);
        container_name_to_container_struct.insert(container_name_str, container_struct);
        identifier+=1;
    }

    let containers_traversal_order = find_containers_traversal_order(container_name_to_parents);

    for cn_name in containers_traversal_order { 
        let container_width = width + find_additional_width(cn_name.as_str().len(), &scale);
        
        let container_struct = container_name_to_container_struct.get(cn_name.as_str()).unwrap();
        container_name_to_point.insert(cn_name.clone(), ContainerPoint::new(cn_name.clone(), x, y));
        
        // let bound_elements = vec![BoundElement{
        //     id: "555".to_string(), 
        //     element_type: "arrow".to_string()
        // }];
        
        // ------------ Draw container ------------
        let container_group = vec![format!("group_{}_container_{}_id",cn_name, container_struct.id)];
       
        // let container_rectangle = Element::simple_rectangle(
        //     container_struct.id.clone(),
        //     x,
        //     y,
        //     container_width,
        //     height,
        //     container_group.clone(),
        //     vec![], // bound_elements,
        //     locked,
        // );
        let mut rectangle_temp_struct = RectangleTempStruct {
            id: container_struct.id.clone(),
            x,
            y,
            width,
            height,
            group_ids: container_group.clone(),
            bound_elements: vec![],
        };

        let container_text = Element::draw_small_monospaced_text(
            x + scale,
            y + scale,
            container_group.clone(),
            locked,
            cn_name.clone(),
        );
        // container_name_to_excalidraw_element.insert(cn_name.clone(), container_rectangle.clone());

        // ------------ Draw ports ------------
        let ports = container_struct.clone().ports.unwrap_or(Vec::new()); 
        for (i, port) in ports.iter().enumerate() {
            let container_x = x + (i as i32 * 80);
            let container_y = y + scale * 8;
            let (host_port_str, container_port_str) = extract_host_container_ports(port);
            let ellipse_port_group = vec![format!("group_{}_hostport_{}_text",cn_name, i)];

            let ellipse_host_port_id = format!("ellipse_{}", generate_random_string_id());
            let host_port_arrow_id = format!("port_arrow_{}",generate_random_string_id());

            let host_port = Element::draw_ellipse (
                ellipse_host_port_id.clone(), 
                container_x,
                container_y,
                port_diameter, 
                port_diameter, 
                ellipse_port_group.clone(),
                vec![BoundElement{
                    id: host_port_arrow_id.clone(), 
                    element_type: "arrow".to_string()
                }],
                locked,
            ); 
            let host_port_text = Element::draw_small_monospaced_text(
                container_x + 15,
                container_y + 20,
                ellipse_port_group.clone(),
                locked,
                host_port_str.clone(),
            );

            let host_port_arrow = Element::simple_arrow(
                host_port_arrow_id.clone(),
                x + 70,
                y + 60,
                200,
                100,
                locked,
                elements::STROKE_STYLE.into(),
                vec![
                    [0, 0],
                    [(i as i32 * 80) - 35, (i as i32 + 100)]
                ],
                Binding {
                    element_id: container_struct.id.clone(),
                    focus: 0.05,
                    gap: 1,
                },
                Binding {
                    element_id: ellipse_host_port_id,
                    focus: 0.05,
                    gap: 1,
                },
            );

            // bind the port arrow to the container
            rectangle_temp_struct.bound_elements.push(BoundElement{
                id: host_port_arrow_id.to_string(), 
                element_type: "arrow".to_string()
            });

            if host_port_str != container_port_str {
                let container_port_text = Element::draw_small_monospaced_text(
                    x + 20 + (i as i32 * 80),
                    y + 80,
                    container_group.clone(),
                    locked,
                    container_port_str,
                );
                excalidraw_file.elements.push(container_port_text);
            }
            excalidraw_file.elements.push(host_port);
            excalidraw_file.elements.push(host_port_text);
            excalidraw_file.elements.push(host_port_arrow);
        }

        // ------------ Draw 'depends_on' relationship ------------
        // excalidraw_file.elements.push(container_rectangle); // TODO postponing
        excalidraw_file.elements.push(container_text);
        
        x += container_width + x_margin;
        y += y_margin;
        container_name_rectangle_temp_structs.insert(cn_name, rectangle_temp_struct);     
    }

    for DependencyComponent {id, name, parent} in &components {
        let ContainerPoint(_, x, y) = container_name_to_point.get(name).unwrap();
        let mut sorted_container_points = if cli.skip_dependencies {
            Vec::<ContainerPoint>::new()
        } else {
            parent
                .iter()
                .map(|dc| {
                    let x1 = container_name_to_point.get(&dc.name).unwrap();
                    ContainerPoint::new(dc.name.clone(), x1.1, x1.2)
                })
                .collect::<Vec<ContainerPoint>>()
        };
        sorted_container_points.sort_by(|cp1, cp2| cp2.1.cmp(&cp1.1));
        
        for (i, parent_point) in sorted_container_points.iter().enumerate() {
                let parent_name = &parent_point.0;
                let mut parent_temp_struct = container_name_rectangle_temp_structs.get_mut(parent_name).unwrap();

                let x_parent = &parent_point.1;
                let y_parent = &parent_point.2;
                let level_height = y_parent - y;
                let interation_x_margin = (i + 1) as i32 * scale;
                let connecting_arrow_points = vec![
                    [0, 0],
                    [0, level_height - height],
                    [-*x + x_parent + width - interation_x_margin * 2, level_height - height],
                    [-*x + x_parent + width - interation_x_margin * 2, *y_parent - y]
                ];
                let connecting_arrow_id = format!("connecting_arrow_{}",generate_random_string_id());
                let connecting_arrow = Element::simple_arrow(
                    connecting_arrow_id.clone(),
                    x + interation_x_margin,
                    *y,
                    0,
                    y_margin,
                    locked,
                    elements::CONNECTION_STYLE.into(),
                    connecting_arrow_points,
                    Binding {
                        element_id: id.to_string(), //"2".to_string(), // child container
                        focus: -0.7142857142857143,
                        gap: 1
                    },
                    Binding {
                        element_id: parent_temp_struct.id.clone(), //"1".to_string(),    // parent container
                        focus: 0.7142857142857143,
                        gap: 1
                    },
                );

                // add child container id to the binding
                // add parent container id to the binding 
                // add boundElements for the child container (id of the connecting_arrow)
                // add boundElements for the parent container (id of the connecting_arrow)                
                // parent_temp_struct.bound_elements.push(BoundElement{
                //     id: connecting_arrow_id.to_string(), 
                //     element_type: "arrow".to_string()
                // });
                let connecting_arrow_bound = BoundElement{
                    id: connecting_arrow_id,
                    element_type: "arrow".to_string(),
                };
                // dbg!(parent_temp_struct);
                
                // let mut parent_bounded_elements = &mut parent_temp_struct.bound_elements;
                // parent_bounded_elements.push(connecting_arrow_bound);
                // parent_temp_struct.bound_elements = parent_bounded_elements;
                parent_temp_struct.bound_elements.push(connecting_arrow_bound.clone());

                let mut current_temp_struct = container_name_rectangle_temp_structs.get_mut(name).unwrap();
                
                current_temp_struct.bound_elements.push(connecting_arrow_bound);
                // dbg!(current_temp_struct);

                // container_name_rectangle_temp_structs.insert(cn_name, rectangle_temp_struct); 

                excalidraw_file.elements.push(connecting_arrow);
        }
    }

    container_name_rectangle_temp_structs.values().into_iter().for_each(|rect|{
        let container_rectangle = Element::simple_rectangle(
                rect.id.clone(),
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                rect.group_ids.clone(),
                rect.bound_elements.clone(),
                locked,
            );
            excalidraw_file.elements.push(container_rectangle);
        }
    );
    let excalidraw_data = serde_json::to_string(&excalidraw_file).unwrap();
    match cli.output_path {
        Some(output_file_path) => {
            fs::write(output_file_path.clone(), excalidraw_data).expect("Unable to write file");
            println!("\nThe input file is '{}'", input_filepath);
            println!("The excalidraw file is successfully generated and put at '{}'\n", output_file_path);
        }
        None => println!("{}", excalidraw_data),
    }
}

/// There are several to declare ports in docker-compose
///  - "0" single port value(range of values): a container port(range) will be assigned to random host port(range)
///  - "1" colon separated values (range of values): container port (range) is assigned to given host port (range)
///  - "_" detailed decrlaration which may include `host_ip`, `protocol` etc
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
                port_string.chars().skip(colon_index + 1).collect()
            )
        }
    }    
}

fn find_containers_traversal_order(container_name_to_parents: HashMap<&str, DependencyComponent>) -> Vec<String> {
    let mut containers_traversal_order: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    for name in container_name_to_parents.keys() {
        traverse_in_hierarchy(name, &container_name_to_parents, &mut containers_traversal_order, &mut visited);
    }
    // Vec::from_iter(visited)
    containers_traversal_order
}

/// According to current `exc.app_state.grid_size` setting and text/font size
/// it's possible to accommodate approximately 3 letters in one grid item.
/// The container width is 7 grid items(140) in total and uses only 5 grid items
/// to accommodate the text up to 14 characters(`max_container_name_len`)
fn find_additional_width(container_name_len: usize, scale: &i32) -> i32 {
    let container_name_len_max = 14;
    let text_accommodation_len_default = 5; 
    let text_accommodation_margin = 1; 
    if container_name_len > container_name_len_max {
        let required_space_for_text = ((container_name_len / 3) - text_accommodation_len_default + text_accommodation_margin) as i32;
        scale * required_space_for_text
    } else {
        0
    }
}

fn parse_yaml_file(file_path: &str) -> Result<HashMap<String, serde_yaml::Value>, ExcalidockerError> {    
    let contents = match read_file(file_path) {
        Ok(contents) => contents,
        Err(err) => return Err(err),
    };
    match serde_yaml::from_str(&contents) {
        Ok(yaml) => Ok(yaml),
        Err(err) => return Err(FileFailedParsing {
            path: file_path.to_string(),
            msg: err.to_string()
        })
    }
}

fn read_file(file_path: &str) -> Result<String, ExcalidockerError> {
    if !(file_path.ends_with(".yaml") || file_path.ends_with(".yml")) {
        return Err(FileIncorrectExtension {
            path: file_path.to_string(),
        })
    }
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(FileNotFound {
            path: file_path.to_string(),
            msg: err.to_string()
        })
    };
    let mut contents = String::new();
    return match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(err) => Err(FileFailedRead {
            path: file_path.to_string(),
            msg: err.to_string()
        })
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
    // TODO: add other fields
}

fn convert_to_container(id: String, value: &Value) -> Option<DockerContainer> {
    let mapping = value.as_mapping()?;
    let mut container = DockerContainer {
        id,
        image: String::new(),
        command: None,
        environment: None,
        ports: None,
        volumes: None,
        depends_on: None,
    };

    for (key, value) in mapping {
        let key_str = key.as_str()?;
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
                        if let (Value::String(key), Value::String(value)) = (env_key, env_value) {
                            env_map.insert(key.clone(), value.clone());
                        }
                    }
                    container.environment = Some(env_map);
                }
            }
            "ports" => {
                if let Value::Sequence(ports) = value {
                    let port_strings = ports.iter().filter_map(|port| port.as_str().map(|p| p.to_string())).collect();
                    container.ports = Some(port_strings);
                }
            }
            "volumes" => {
                if let Value::Sequence(volumes) = value {
                    let volume_strings = volumes.iter().filter_map(|volume| volume.as_str().map(|v| v.to_string())).collect();
                    container.volumes = Some(volume_strings);
                }
            }
            "depends_on" => {
                if let Value::Sequence(depends_on) = value {
                    let depends_on = depends_on.iter().filter_map(|port| port.as_str().map(|p| p.to_string())).collect();
                    container.depends_on = Some(depends_on);
                }
            }
             // TODO: Handle other fields
            _ => (),
        }
    }
    Some(container)
}

fn generate_random_string_id() -> String {
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
fn check_port_parsing() {
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