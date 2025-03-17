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
}
if let Some(item) = supermarket.inventory.get(id) {
            println!("Zadejte množství: ");
            let mut qty = String::new();
            io::stdin().read_line(&mut qty).unwrap();
            let qty: f64 = qty.trim().parse().unwrap_or(1.0);
            
            let price = qty * item.price_per_unit;
            total_price += price;
            println!("Přidáno: {} x {} = {:.2} Kč", qty, item.name, price);
        }
    else {
            println!("Zboží nenalezeno.");
        }

    println!("Celková cena: {:.2} Kč", total_price);

fn main() {
    let filename = "inventory.json";
    let mut supermarket = Supermarket::load_inventory(filename);
    
    supermarket.add_new_item("123", "Mléko", 20.0);
    supermarket.add_new_item("456", "Chléb", 30.0);
    supermarket.save_inventory(filename);
    
    self_service_checkout(&supermarket);
}
