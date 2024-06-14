use eframe::egui;

use crate::money_service::money_service_request;
use crate::MyApp;



impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            let body_style = egui::TextStyle::Body.resolve(ui.style());
            let button_style = egui::TextStyle::Button.resolve(ui.style());
            let mut new_body_style = body_style.clone();
            let new_button_style = button_style.clone();
            new_body_style.size = 20.0;
            new_body_style.size = 20.0;
            ui.style_mut().text_styles.insert(egui::TextStyle::Body, new_body_style);
            ui.style_mut().text_styles.insert(egui::TextStyle::Button, new_button_style);


            ui.heading("Karlito's External");

            //ui.spacing_mut().item_spacing.y = 10.0;
            //ui.spacing_mut().item_spacing.x = 10.0;

            ui.group(|ui| {
                ui.set_min_height(100.0);
                ui.set_min_width(100.0);
                ui.label("Visual");
                ui.add_space(5.0);
                if ui.checkbox(&mut self.glow_check, "Glow").clicked() {
                    let _ = self.tx_glow_data.send(self.glow_check);

                }
                if ui.checkbox(&mut self.antiflash_check, "Antiflash").clicked() {
                    let _ = self.tx_antiflash_data.send(self.antiflash_check);
                }
                
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.set_min_height(200.0);
                ui.set_min_width(100.0);
                ui.label("Money Service");
                ui.add_space(10.0);
            
                let (names, money) = money_service_request();
                for (name, plr_money) in names.iter().zip(money.iter()) {
                    ui.label(format!("{} : ${}", name, plr_money));
                }
            });
            
        });

        
    }
}

pub(crate) fn start_gui(data : MyApp) {
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    

    let _ = eframe::run_native(
        "Karlito's External",
        options,
        Box::new(|_cc| Box::new(data)),
    );
}

