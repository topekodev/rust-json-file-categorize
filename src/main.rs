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
    if path::Path::new(&CONFIG.file_out_dir).exists() {
        fs::remove_dir_all(&CONFIG.file_out_dir).expect("Could not remove existing out dir");
    }
    if path::Path::new(&CONFIG.label_out_dir).exists() {
        fs::remove_dir_all(&CONFIG.label_out_dir).expect("Could not remove existing out dir");
    }

    let labels = get_labels();
    println!("Processing...");
    let mut count = 0;
    for label in labels {
        let label_data = fs::read_to_string(&label).expect("Could not read label file");
        let label_json: gjson::Value = gjson::parse(&label_data);
        let linking_file = if CONFIG.file_link_property_get_filename {
            &path::Path::new(&label_json.get(&CONFIG.file_link_property).to_string()).file_name().and_then(|s| s.to_str()).unwrap_or("").to_string()
        } else {
            &label_json.get(&CONFIG.file_link_property).to_string()
        };
        let file_path = path::Path::new(&CONFIG.file_dir).join(&linking_file);
        if file_path.exists() {
            for category in &CONFIG.categories {
                if match_filters(&label_json, &category.filters) {
                    symlink(&file_path, &label, &category.name);
                    count+=1;
                }
            }
        }
    }
    println!("Symlinked {} files", count);
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

fn symlink(file: &path::PathBuf, label: &path::PathBuf, category: &String) {
    let file_name = file.file_name().unwrap();
    let file_dest_dir = path::Path::new(&CONFIG.file_out_dir).join(category);
    let file_dest_path = file_dest_dir.join(file_name);
    let label_name = label.file_name().unwrap();
    let label_dest_dir = path::Path::new(&CONFIG.label_out_dir).join(category);
    let label_dest_path = label_dest_dir.join(label_name);
    fs::create_dir_all(file_dest_dir).expect("Could not create destionation dir");
    fs::create_dir_all(label_dest_dir).expect("Could not create destionation dir");
    unix::fs::symlink(file, file_dest_path).expect("Could not symlink file");
    unix::fs::symlink(label, label_dest_path).expect("Could not symlink file");
}
