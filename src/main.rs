use eframe::egui;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Goods {
    id: String,
    name: String,
    price_per_unit: f64,
    requires_id_check: bool,
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

    fn add_new_item(&mut self, id: &str, name: &str, price_per_unit: f64, requires_id_check: bool) {
        let item = Goods {
            id: id.to_string(),
            name: name.to_string(),
            price_per_unit,
            requires_id_check,
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

struct SupermarketApp {
    supermarket: Supermarket,
    filename: String,
    new_id: String,
    new_name: String,
    new_price: String,
    new_requires_id: bool,
    total_price: f64,
    cart: Vec<(String, f64)>,
    quantity_inputs: HashMap<String, String>,
    amount_paid: String,
    change_due: f64,
}

impl SupermarketApp {
    fn new(filename: &str) -> Self {
        let supermarket = Supermarket::load_inventory(filename);
        SupermarketApp {
            supermarket,
            filename: filename.to_string(),
            new_id: String::new(),
            new_name: String::new(),
            new_price: String::new(),
            new_requires_id: false,
            total_price: 0.0,
            cart: Vec::new(),
            quantity_inputs: HashMap::new(),
            amount_paid: String::new(),
            change_due: 0.0,
        }
    }
}

impl eframe::App for SupermarketApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Supermarket");

            ui.vertical(|ui| {
                ui.label("ID zboží:");
                ui.text_edit_singleline(&mut self.new_id);
            });

            ui.vertical(|ui| {
                ui.label("Název zboží:");
                ui.text_edit_singleline(&mut self.new_name);
            });

            ui.vertical(|ui| {
                ui.label("Cena:");
                ui.text_edit_singleline(&mut self.new_price);
            });

            ui.checkbox(&mut self.new_requires_id, "Vyžaduje OP");

            if ui.button("Přidat zboží").clicked() {
                if let Ok(price) = self.new_price.parse::<f64>() {
                    self.supermarket.add_new_item(&self.new_id, &self.new_name, price, self.new_requires_id);
                    self.supermarket.save_inventory(&self.filename);
                    self.new_id.clear();
                    self.new_name.clear();
                    self.new_price.clear();
                    self.new_requires_id = false;
                }
            }

            ui.separator();
            ui.heading("Nákupní košík");

            let mut requires_id_check = false;
            for (id, qty) in &self.cart {
                if let Some(item) = self.supermarket.inventory.get(id) {
                    let price = qty * item.price_per_unit;
                    ui.label(format!("{} x {} = {:.2} Kč", qty, item.name, price));
                    if item.requires_id_check {
                        requires_id_check = true;
                    }
                }
            }
            ui.label(format!("Celková cena: {:.2} Kč", self.total_price));

            if requires_id_check {
                ui.label("Některé položky vyžadují kontrolu OP!");
            }

            ui.separator();
            ui.label("Zaplacená částka:");
            ui.text_edit_singleline(&mut self.amount_paid);
            if ui.button("Vypočítat vrácenou částku").clicked() {
                if let Ok(amount_paid) = self.amount_paid.parse::<f64>() {
                    self.change_due = amount_paid - self.total_price;
                }
            }
            ui.label(format!("Vrácená částka: {:.2} Kč", self.change_due));

            ui.separator();
            ui.heading("Dostupné zboží");

            for (id, item) in &self.supermarket.inventory {
                ui.horizontal(|ui| {
                    ui.label(format!("{} - {} ({:.2} Kč/ks){}", id, item.name, item.price_per_unit, if item.requires_id_check { " 200+!!" } else { "" }));

                    let qty_entry = self.quantity_inputs.entry(id.clone()).or_insert_with(String::new);
                    ui.text_edit_singleline(qty_entry);

                    if ui.button("Přidat do košíku").clicked() {
                        if let Ok(qty) = qty_entry.parse::<f64>() {
                            self.total_price += qty * item.price_per_unit;
                            self.cart.push((id.clone(), qty));
                            qty_entry.clear();
                        }
                    }
                });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Supermarket GUI",
        options,
        Box::new(|_cc| Ok(Box::new(SupermarketApp::new("inventory.json")))),
    )
}

