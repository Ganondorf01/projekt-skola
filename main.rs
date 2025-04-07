use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;

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
        Self {
            inventory: HashMap::new(),
        }
    }

    fn add_new_item(
        &mut self,
        id: Option<&str>,
        name: &str,
        price_per_unit: f64,
        requires_id_check: bool,
    ) {
        let item_id = match id {
            Some(id) => id.to_string(),
            None => Uuid::new_v4().to_string(),
        };

        let item = Goods {
            id: item_id.clone(),
            name: name.to_string(),
            price_per_unit,
            requires_id_check,
        };

        self.inventory.insert(item_id, item);
    }

    fn save_inventory(&self, filename: &str) {
        if let Ok(serialized) = serde_json::to_string_pretty(&self) {
            let _ = fs::write(filename, serialized);
        }
    }

    fn load_inventory(filename: &str) -> Self {
        let data = fs::read_to_string(filename).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_else(|_| Self::new())
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
    message: Option<String>,
    confirm_removal: Option<String>,
}

impl SupermarketApp {
    fn new(filename: &str) -> Self {
        let supermarket = Supermarket::load_inventory(filename);
        Self {
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
            message: None,
            confirm_removal: None,
        }
    }

    fn add_to_cart(&mut self, id: &str, qty: f64) {
        for item in self.cart.iter_mut() {
            if item.0 == id {
                item.1 += qty;
                return;
            }
        }
        self.cart.push((id.to_string(), qty));
    }

    fn remove_from_cart(&mut self, id: &str) {
        self.cart.retain(|(item_id, _)| item_id != id);
    }
    /*
    pub fn sort_items_by_price(&mut self) {
        let mut items: Vec<_> = self.supermarket.inventory.values().collect();
        items.sort_by(|a, b| a.price_per_unit.partial_cmp(&b.price_per_unit).unwrap());
    }
    */
}

impl eframe::App for SupermarketApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Supermarket System");
                ui.separator();

                if let Some(item_id) = self.confirm_removal.take() {
                    let mut open = true;
                    egui::Window::new("Confirm Removal")
                        .open(&mut open)
                        .resizable(false)
                        .collapsible(false)
                        .show(ctx, |ui| {
                            ui.label(format!(
                                "Are you sure you want to remove {} from the cart?",
                                item_id
                            ));
                            ui.horizontal(|ui| {
                                if ui.button("Yes").clicked() {
                                    self.remove_from_cart(&item_id);
                                }
                                if ui.button("No").clicked() {
                                }
                            });
                        });
                    return;
                }

                ui.heading("Přidání nového zboží");

                ui.label("Název zboží:");
                ui.text_edit_singleline(&mut self.new_name);

                ui.label("Cena (Kč):");
                ui.text_edit_singleline(&mut self.new_price);

                ui.checkbox(&mut self.new_requires_id, "Vyžaduje OP");

                if ui.button("Přidat zboží").clicked() {
                    if let Ok(price) = self.new_price.parse::<f64>() {
                        if !self.new_name.is_empty() {
                            self.supermarket.add_new_item(
                                None,
                                &self.new_name,
                                price,
                                self.new_requires_id,
                            );
                            self.supermarket.save_inventory(&self.filename);
                            self.message = Some(format!("Přidáno: {}", self.new_name));
                            self.new_name.clear();
                            self.new_price.clear();
                            self.new_requires_id = false;
                        } else {
                            self.message = Some("Vyplňte název zboží".into());
                        }
                    } else {
                        self.message = Some("Cena musí být číslo".into());
                    }
                }

                if let Some(msg) = &self.message {
                    ui.label(msg);
                }

                ui.separator();
                ui.heading("Nákupní košík");

                self.total_price = 0.0;
                let mut requires_id_check = false;
                let mut removals: Vec<String> = vec![];

                for (id, qty) in &self.cart {
                    if let Some(item) = self.supermarket.inventory.get(id) {
                        let price = qty * item.price_per_unit;
                        self.total_price += price;
                        if item.requires_id_check {
                            requires_id_check = true;
                        }

                        ui.horizontal(|ui| {
                            ui.label(format!("{} x {} = {:.2} Kč", qty, item.name, price));
                            if ui.button("Odebrat").clicked() {
                                removals.push(id.clone());
                            }
                        });
                    }
                }

                for id in removals {
                    self.remove_from_cart(&id);
                }

                ui.label(format!("Celková cena: {:.2} Kč", self.total_price));

                if requires_id_check {
                    ui.colored_label(
                        egui::Color32::RED,
                        "Některé položky vyžadují kontrolu OP!",
                    );
                }

                ui.horizontal(|ui| {
                    ui.label("Zaplaceno:");
                    ui.text_edit_singleline(&mut self.amount_paid);
                });

                if let Ok(paid) = self.amount_paid.parse::<f64>() {
                    self.change_due = paid - self.total_price;
                    if self.change_due >= 0.0 {
                        ui.label(format!("Vydat zpět: {:.2} Kč", self.change_due));
                    } else {
                        ui.colored_label(egui::Color32::RED, "Zadaná částka je nedostatečná.");
                    }
                }

                if ui.button("Vymazat košík").clicked() {
                    self.cart.clear();
                    self.total_price = 0.0;
                    self.amount_paid.clear();
                    self.change_due = 0.0;
                    self.message = None;
                }

                ui.separator();
                ui.heading("Sklad");

                let inventory_items: Vec<(String, Goods)> = self
                    .supermarket
                    .inventory
                    .iter()
                    .map(|(id, item)| (id.clone(), item.clone()))
                    .collect();

                for (id, item) in inventory_items {
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{} - {} ({:.2} Kč/ks){}",
                            id,
                            item.name,
                            item.price_per_unit,
                            if item.requires_id_check {
                                "Vyžaduje OP"
                            } else {
                                ""
                            }
                        ));

                        let qty_entry = self.quantity_inputs.entry(id.clone()).or_default();
                        ui.text_edit_singleline(qty_entry);

                        if ui.button("Přidat do košíku").clicked() {
                            if let Some(qty_entry) = self.quantity_inputs.get(&id) {
                                if let Ok(qty) = qty_entry.parse::<f64>() {
                                    if qty > 0.0 {
                                        self.add_to_cart(&id, qty);
                                        if let Some(qty_entry) = self.quantity_inputs.get_mut(&id) {
                                            qty_entry.clear();
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            });
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
