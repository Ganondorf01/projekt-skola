use tauri::[]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::io;


#[derive(Serialize, Deserialize, Debug)]
struct Goods {
    id: String,
    name: String,
    price_per_unit: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Supermarket {
    inventory: HashMap<String, Goods>,
}

impl Supermarket {
    fn new() -> Self {
        Supermarket {
            inventory: HashMap::new(),
        }
    }
