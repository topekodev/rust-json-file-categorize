use std::fs;
use gjson;

#[derive(Debug)]
pub struct Config {
    pub file_dir: String,
    pub label_dir: String,
    pub file_out_dir: String,
    pub label_out_dir: String,
    pub file_link_property: String,
    pub file_link_property_get_filename: bool,
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
            property: json.get("property").to_string(),
            values
        }
    }
}

pub fn load_config(config_path: &str) -> Config {
    let config_data = fs::read_to_string(config_path).expect("Could not read config file");
    let config_json: gjson::Value = gjson::parse(&config_data);
    let mut categories: Vec<Category> = vec![];
    for category in config_json.get("categories").array() {
        categories.push(Category::new(category));
    }
    let config: Config = Config {
        file_dir: config_json.get("file_dir").to_string(),
        label_dir: config_json.get("label_dir").to_string(),
        file_out_dir: config_json.get("file_out_dir").to_string(),
        label_out_dir: config_json.get("label_out_dir").to_string(),
        file_link_property: config_json.get("file_link_property").to_string(),
        file_link_property_get_filename: config_json.get("file_link_property_get_filename").bool(),
        categories
    };
    config
}
