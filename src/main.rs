use std::env;

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
    println!("{:?}", CONFIG.categories);
}
