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

    fn add_new_item(&mut self, id: &str, name: &str, price_per_unit: f64) {
        let item = Goods {
            id: id.to_string(),
            name: name.to_string(),
            price_per_unit,
        };
        self.inventory.insert(id.to_string(), item);
    }

    fn save_inventory(&self, filename: &str) {
        let serialized = serde_json::to_string(&self).unwrap();
        fs::write(filename, serialized).unwrap();
    }

    fn load_inventory(filename: &str) -> Self {
        let data = fs::read_to_string(filename).unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str(&data).unwrap_or_else(|_| Supermarket::new())
    }
}

fn self_service_checkout(supermarket: &mut Supermarket, filename: &str) {
    let mut total_price = 0.0;
    loop {
        println!("Zadej brasko id predmet (konec pro zaplaceni nebo novy pro pridani noveho predmetu) ");
        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        let id = id.trim();

        if id == "novy" {
            println!("ID zboží:");
            let mut item_id = String::new();
            io::stdin().read_line(&mut item_id).unwrap();
            let item_id = item_id.trim().to_string();

            println!("Název zboží:");
            let mut item_name = String::new();
            io::stdin().read_line(&mut item_name).unwrap();
            let item_name = item_name.trim().to_string();

            println!("Cena zboží (v formátu xy.z):");
            let mut item_price = String::new();
            io::stdin().read_line(&mut item_price).unwrap();
            let item_price: f64 = item_price.trim().parse().unwrap_or(0.0);

            supermarket.add_new_item(&item_id, &item_name, item_price);
            supermarket.save_inventory(filename);
            println!("Předmět úspěšně přidán!");
            continue; 
        }

        if id == "konec" {
            break;
        }

        if let Some(item) = supermarket.inventory.get(id) {
            println!("Zadejte množství: ");
            let mut qty = String::new();
            io::stdin().read_line(&mut qty).unwrap();
            let qty: f64 = qty.trim().parse().unwrap_or(1.0);
            
            let price = qty * item.price_per_unit;
            total_price += price;
            println!("Přidáno: {} x {} = {:.2} Kč", qty, item.name, price);
        } else {
            println!("Zboží nenalezeno.");
        }
    }
    println!("Celková cena: {:.2} Kč", total_price);
}

fn main() {
    let filename = "inventory.json";
    let mut supermarket = Supermarket::load_inventory(filename);
    
    supermarket.save_inventory(filename);
    
    self_service_checkout(&mut supermarket, filename);
}

