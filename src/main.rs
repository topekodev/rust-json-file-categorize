use std::env;
use std::path;
use std::fs;
use std::os::unix;
use gjson;

#[macro_use]
extern crate lazy_static;
mod config;

lazy_static!{
    pub static ref CONFIG_FILE: String = get_args(1);
    pub static ref CONFIG: config::Config = config::load_config(&CONFIG_FILE);
}

fn get_args(n: usize) -> String {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        args[n].clone()
    } else {
        "".to_string()
    }
}

fn main() {
    if path::Path::new(&CONFIG.out_dir).exists() {
        fs::remove_dir_all(&CONFIG.out_dir).expect("Could not remove existing out dir");
    }

    let labels = get_labels();
    println!("Processing...");
    for label in labels {
        let label_data = fs::read_to_string(&label).expect("Could not read label file");
        let label_json: gjson::Value = gjson::parse(&label_data);
        let linking_file = label_json.get(&CONFIG.file_link_property).to_string();
        let file_path = path::Path::new(&CONFIG.file_dir).join(&linking_file);
        if file_path.is_file() {
            for category in &CONFIG.categories {
                if match_filters(&label_json, &category.filters) {
                    symlink_file(&linking_file, &category.name);
                }
            }
        }
    }
    println!("Done!");
}

fn match_filters(label: &gjson::Value, filters: &Vec<config::Filter>) -> bool {
    for filter in filters {
        if filter.values.contains(&label.get(&filter.property).to_string()) {
            continue;
        } else {
            return false
        }
    }
    true
}

fn get_labels() -> Vec<path::PathBuf> {
    println!("Getting label files...");
    let json_files = fs::read_dir(CONFIG.label_dir.clone())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "json"))
        .collect::<Vec<_>>();
    println!("Found {} items", json_files.len());
    json_files
}

fn symlink_file(file: &String, category: &String) {
    let source_dir = if path::Path::new(&CONFIG.out_dir).is_relative() {
        path::Path::new("../").join(&CONFIG.out_dir).to_path_buf()
    } else {
        path::Path::new(&CONFIG.out_dir).to_path_buf()
    };
    let source_path = source_dir.join(&file);
    let dest_dir = path::Path::new(&CONFIG.out_dir).join(category);
    let dest_path = dest_dir.join(file);
    fs::create_dir_all(dest_dir).expect("Could not create destionation dir");
    unix::fs::symlink(source_path, dest_path).expect("Could not symlink file");
}
