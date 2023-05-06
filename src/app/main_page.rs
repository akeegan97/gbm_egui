use egui_extras::DatePickerButton;
use eframe::egui;
use egui::{Color32, RichText, FontId, Vec2};


pub fn start(ctx:&egui::Context, picked_path: &mut String){
    egui::TopBottomPanel::top("Main Page")
        .frame(egui::Frame::default()
            .inner_margin(MARGIN)
            .fill(Color32::GRAY)
        ).show(ctx,|ui|{
            ui.columns(3,|c|{
                c[1].add(egui::Label::new(RichText::new("Geometric Brownian Motion Sim").color(Color32::WHITE).font(FontId::proportional(25.0))));
            });

        });
    egui::SidePanel::left("Parameters")
        .frame(egui::Frame::default()
            .inner_margin(MARGIN)
            .fill(Color32::GRAY)
        ).show(ctx, |ui|{
            if ui.add(egui::Button::new("Click to Select File")).clicked(){
                if let Some(path) = rfd::FileDialog::new().pick_file(){
                    *picked_path = path.display().to_string();
                }
            }
            //going to add two selectable labels -> [backtest],[real predict]
            //if backtest the prediction date has to be found inside of the data file
            //if real predict the prediction date doesn't have to be
            //select how many paths to create and run 10-100-1000-10000-100000
            //? other parameters, maybe change the type of Error term used, or allow a scaleable coefficent if desired
            
        });
}



const MARGIN:Vec2 = egui::vec2(7.0,7.0);