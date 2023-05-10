//use egui_extras::DatePickerButton;
use eframe::egui;
use egui::{Color32, RichText, FontId, Vec2,plot::{PlotPoints,Line, Legend, Plot}};
use chrono::{NaiveDate, Weekday, Datelike};
use crate::functions::gbm::{self};
#[derive(PartialEq)]
pub enum PriceType{
    High,
    Low,
    Close,
    Adjclose,
    Open,
}

pub fn start(ctx:&egui::Context, picked_path: &mut String,file_specified:&mut bool, t_start_date:&mut NaiveDate, t_end_date:&mut NaiveDate, selected_steps:&mut i64, paths:&mut i64, selected_price_type:&mut Option<PriceType>,predicted_price:&mut f64,
                mu:&mut f64,
                sigma:&mut f64,
                sigma_sq:&mut f64,
                plotting_vecs:&mut Vec<Vec<f64>>,
                real_price:&mut f64,
                step_size:&mut f64,
                starting_price:&mut f64){
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
                *file_specified = true;
            }
            let picked_file_path = &*picked_path;
            ui.add(egui::Label::new(RichText::new(picked_file_path).color(Color32::BLACK).font(FontId::proportional(10.0))));

            //Parameters

            ui.separator();
            ui.add(egui::Label::new(RichText::new("Select Training Data Start Date").color(Color32::BLACK).font(FontId::proportional(15.0))));
            ui.add(egui_extras::DatePickerButton::new(t_start_date).id_source("t_start"));
            ui.add(egui::Label::new(RichText::new("Select Training Data End Date").color(Color32::BLACK).font(FontId::proportional(15.0))));
            ui.add(egui_extras::DatePickerButton::new(t_end_date).id_source("t_end"));
            ui.add(egui::Label::new(RichText::new("Select Prediction Steps").color(Color32::BLACK).font(FontId::proportional(15.0))));
            ui.add(egui::Slider::new(selected_steps,1..=65).clamp_to_range(false).smart_aim(false));
            let prediction_day:NaiveDate = *t_end_date + chrono::Duration::days(*selected_steps);
            ui.add(egui::Label::new(RichText::new("Select Number of Paths").color(Color32::BLACK).font(FontId::proportional(15.0))));
            ui.add(egui::Slider::logarithmic(egui::Slider::new(paths, 0..=1_000_000_000), true));

            //making sure the predicted day is a trading day

            if !is_weekday(prediction_day){
                ui.add(egui::Label::new(RichText::new("Invalid Date Choose Different Steps").color(Color32::BLACK).font(FontId::proportional(15.0))));  
            }else{
                ui.add(egui::Label::new(RichText::new("Valid Date for Prediction").color(Color32::BLACK).font(FontId::proportional(15.0))));
            }
            ui.separator();

            //making sure file is selected before option of simulating

            ui.vertical(|v|{
                let high_price:bool = *selected_price_type == Some(PriceType::High);
                if v.selectable_label(high_price, egui::RichText::new("Use High Price").color(Color32::BLACK).font(FontId::proportional(15.0))).clicked(){
                    *selected_price_type = Some(PriceType::High)
                }
                let low_price:bool = *selected_price_type == Some(PriceType::Low);
                if v.selectable_label(low_price, egui::RichText::new("Use Low Price").color(Color32::BLACK).font(FontId::proportional(15.0))).clicked(){
                    *selected_price_type = Some(PriceType::Low)
                }
                let close_price:bool = *selected_price_type == Some(PriceType::Close);
                if v.selectable_label(close_price, egui::RichText::new("Use Close Price").color(Color32::BLACK).font(FontId::proportional(15.0))).clicked(){
                    *selected_price_type = Some(PriceType::Close)
                }
                let adj_close_price:bool = *selected_price_type == Some(PriceType::Adjclose);
                if v.selectable_label(adj_close_price, egui::RichText::new("Use Adj Close Price").color(Color32::BLACK).font(FontId::proportional(15.0))).clicked(){
                    *selected_price_type = Some(PriceType::Adjclose)
                }
                let open_price:bool = *selected_price_type == Some(PriceType::Open);
                if v.selectable_label(open_price, egui::RichText::new("Use Open Price").color(Color32::BLACK).font(FontId::proportional(15.0))).clicked(){
                    *selected_price_type = Some(PriceType::Open)
                }
            }); 
            ui.separator();
            
            if *file_specified && is_weekday(prediction_day){
                if ui.add(egui::Button::new("Click to Sim")).clicked(){
                    gbm::gbm(&picked_file_path, *t_start_date, 
                        *t_end_date, 
                        selected_steps, 
                        paths, 
                        selected_price_type,
                        predicted_price,
                        mu,
                        sigma,
                        sigma_sq,
                        plotting_vecs,
                        real_price,
                        step_size,
                        starting_price
                        );
                }
            }
        });
        egui::TopBottomPanel::top("results from sim").frame(egui::Frame::default()
            .inner_margin(MARGIN)
            .fill(Color32::GRAY)
            ).show(ctx, |ui|{
                ui.columns(2, |c|{
                    let percent_error = ((*predicted_price - *real_price)/ *real_price).abs() * 100.0;
                    c[0].label(RichText::new("Mu: ").color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[0].label(RichText::new("Sigma: ").color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[0].label(RichText::new("Sigma Squared: ").color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[0].label(RichText::new("Days Predicted: ").color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[0].separator();
                    c[0].label(RichText::new("Starting Price: ").color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[0].label(RichText::new("Predicted Price: ").color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[0].label(RichText::new("Real Price: ").color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[0].label(RichText::new("Percent Error: ").color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[1].label(RichText::new(format!("{}",*mu)).color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[1].label(RichText::new(format!("{}",*sigma)).color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[1].label(RichText::new(format!("{}",*sigma_sq)).color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[1].label(RichText::new(format!("{}",*step_size)).color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[1].separator();
                    c[1].label(RichText::new(format!("{}",*starting_price)).color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[1].label(RichText::new(format!("{}",*predicted_price)).color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[1].label(RichText::new(format!("{}",*real_price)).color(Color32::BLACK).font(FontId::proportional(18.0)));
                    c[1].label(RichText::new(format!("{}",percent_error)).color(Color32::BLACK).font(FontId::proportional(18.0)));

                }); 
            });
            egui::CentralPanel::default().frame(egui::Frame::default().fill(Color32::GRAY).inner_margin(MARGIN)).show(ctx,|ui|{
                //getting the vectors from the sim into the correct form for egui's plotting style
                let new_plot_vecs:Vec<Vec<[f64;2]>> = plotting_vecs
                    .iter()
                    .map(|iv| iv
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| [(i + 1) as f64,x])
                        .collect())
                    .collect::<Vec<Vec<[f64;2]>>>();

                let mut plot = Plot::new("GBM GRAPH")
                    .view_aspect(3.0)
                    .auto_bounds_y()
                    .include_y(0.0);
                let mut lines:Vec<Line> = Vec::new();
                for path in new_plot_vecs.iter(){
                    let plot_points: PlotPoints = PlotPoints::new(path.to_vec());
                    let line = Line::new(plot_points)
                        .color(Color32::from_rgba_premultiplied(
                            rand::random::<u8>(),
                            rand::random::<u8>(),
                            rand::random::<u8>(),
                            255,
                        ));
                    lines.push(line);
                }
                plot.show(ui,|p_ui|{
                    for i in lines{
                        p_ui.line(i);
                    }
                });
            });
            
}

fn is_weekday(date: NaiveDate) -> bool {
    let weekday = date.weekday();
    weekday != Weekday::Sat && weekday != Weekday::Sun
}

// fn is_holiday(date: NaiveDate) -> bool {
//     // add your holiday logic here, for example:
//     date.month() == 12 && date.day() == 25 // Christmas Day
// }

const MARGIN:Vec2 = egui::vec2(7.0,7.0);