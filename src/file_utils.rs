use std::{fs::File, process::exit};
use std::io::Read;

use isahc::ReadResponseExt;

use serde_yaml::{Mapping, Value};

use crate::{error::ExcalidockerError::{
    self, FileIncorrectExtension, FileNotFound, RemoteFileFailedRead,
}, exporters::excalidraw_config::ExcalidrawConfig};


pub fn get_excalidraw_config(file_path: &str) -> ExcalidrawConfig {
    let excalidocker_config_contents = match read_yaml_file(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Configuration file issue: {}", err);
            exit(1);
        }
    };
    return match serde_yaml::from_str(&excalidocker_config_contents) {
            Ok(cfg) => cfg,
            Err(err) => {
                println!("Configuration parsing issue: {}", err);
                exit(1);
            }
        }
}

pub fn get_docker_compose_content(file_path: &str) ->  Mapping {
    let file_content = match get_file_content(file_path) {
        Ok(content) => content,
        Err(err) => {
            println!("{}", err);
            return Mapping::new();
        }
    };
    return match serde_yaml::from_str::<Value>(&file_content) {
        Ok(mut yaml_content) => {
            let _ = yaml_content.apply_merge(); // TODO potentially here we know which files are using anchors
            yaml_content
                .as_mapping()
                .unwrap_or(&Mapping::new())
                .to_owned()
        }
        Err(err) => {
            println!("{}", err);
            return Mapping::new();
        }
    };
}

/// Read yaml file content into a String
fn read_yaml_file(file_path: &str) -> Result<String, ExcalidockerError> {
    if !(file_path.ends_with(".yaml") || file_path.ends_with(".yml")) {
        return Err(FileIncorrectExtension {
            path: file_path.to_string(),
        });
    }
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(FileNotFound {
                path: file_path.to_string(),
                msg: err.to_string(),
            })
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(err) => Err(FileNotFound {
            path: file_path.to_string(),
            msg: err.to_string(),
        }),
    }
}

/// Get file content as a String. 
/// Both remote (f.e. from Github) and local files are supported
fn get_file_content(file_path: &str) -> Result<String, ExcalidockerError> {
    if file_path.starts_with("http") {
        let url = rewrite_github_url(file_path);
        let mut response = match isahc::get(url) {
            Ok(rs) => rs,
            Err(err) => {
                return Err(RemoteFileFailedRead {
                    path: file_path.to_string(),
                    msg: err.to_string(),
                })
            }
        };
        match response.text() {
            Ok(data) => Ok(data),
            Err(err) => Err(RemoteFileFailedRead {
                path: file_path.to_string(),
                msg: err.to_string(),
            }),
        }
    } else {
        read_yaml_file(file_path)
    }
}

/// When a Github website link provided instead of a link to a raw file
/// this method rewrites the url thus it's possible to get the referenced file content.
fn rewrite_github_url(input: &str) -> String {
    if input.contains("github.com") {
        input
            .replace("https://github.com/", "https://raw.githubusercontent.com/")
            .replace("/blob/", "/")
    } else {
        input.to_owned()
    }
}

#[test]
fn test_rewrite_github_url() {
    let input1 = "https://github.com/etolbakov/excalidocker-rs/blob/main/data/compose/docker-compose-very-large.yaml";
    assert_eq!(
        "https://raw.githubusercontent.com/etolbakov/excalidocker-rs/main/data/compose/docker-compose-very-large.yaml",
        rewrite_github_url(input1)
    );
    let input2 =
        "https://github.com/treeverse/lakeFS/blob/master/deployments/compose/docker-compose.yml";
    assert_eq!(
        "https://raw.githubusercontent.com/treeverse/lakeFS/master/deployments/compose/docker-compose.yml",
        rewrite_github_url(input2)
    );
    let input3 = "https://github.com/etolbakov/excalidocker-rs/blob/feat/edge-type-support/data/compose/docker-compose-very-large.yaml";
    assert_eq!(
        "https://raw.githubusercontent.com/etolbakov/excalidocker-rs/feat/edge-type-support/data/compose/docker-compose-very-large.yaml",
        rewrite_github_url(input3)
    );
    let input4 = "https://raw.githubusercontent.com/etolbakov/excalidocker-rs/blob/edge-type-support/data/compose/docker-compose-very-large.yaml";
    assert_eq!(
        "https://raw.githubusercontent.com/etolbakov/excalidocker-rs/blob/edge-type-support/data/compose/docker-compose-very-large.yaml",
        rewrite_github_url(input4)
    );
}
