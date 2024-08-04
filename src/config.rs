use std::fs;
use gjson;

#[derive(Debug)]
pub struct Config {
    pub file_dir: String,
    pub label_dir: String,
    pub out_dir: String,
    pub label_file_link_property: Vec<String>,
    pub categories: Vec<Category>,
}
#[derive(Debug)]
pub struct Category {
    pub name: String,
    pub filters: Vec<Filter>,
}
impl Category {
    pub fn new(json: gjson::Value) -> Self {
        let mut filters: Vec<Filter> = vec![];
        for filter in json.get("filters").array() {
            filters.push(Filter::new(filter));
        }
        Category {
            name: json.get("name").to_string(),
            filters
        }
    }
}
#[derive(Debug)]
pub struct Filter {
    pub property: String,
    pub values: Vec<String> 
}
impl Filter {
    pub fn new(json: gjson::Value) -> Self {
        let mut values: Vec<String> = vec![];
        for value in json.get("values").array() {
            values.push(value.to_string()); 
        }
        Filter {
            property: json.get("properties").to_string(),
            values
        }
    }
}

pub fn load_config(config_path: &str) -> Config {
    let config_data = fs::read_to_string(config_path).expect("Could not read config file");
    let config_json: gjson::Value = gjson::parse(&config_data);
    let mut label_file_link_property: Vec<String> = vec![];
    for property in config_json.get("label_file_link_property").array() {
        label_file_link_property.push(property.to_string());
    }
    let mut categories: Vec<Category> = vec![];
    for category in config_json.get("categories").array() {
        categories.push(Category::new(category));
    }
    println!("{:?}", categories);
    let config: Config = Config {
        file_dir: config_json.get("file_dir").to_string(),
        label_dir: config_json.get("label_dir").to_string(),
        out_dir: config_json.get("out_dir").to_string(),
        label_file_link_property,
        categories
    };
    config
}
