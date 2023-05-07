use app::main_page::start;
use std::env;
use egui::{FontDefinitions};
use eframe::egui;
use chrono::NaiveDate;
mod functions{
    pub mod gbm;
}
mod app{
    pub mod main_page;
}
use crate::app::main_page::PriceType;

fn setup(ctx: &egui::Context){
    let mut fonts: FontDefinitions = FontDefinitions::default();
    fonts.font_data.insert(
        "MyFont".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/TimesNewRoman.ttf")));
    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "MyFont".to_owned());
    fonts.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("MyFont".to_owned());
    ctx.set_fonts(fonts);
//add images here if needed
}
fn main()->Result<(),eframe::Error>{
    env::set_var("RUST_BACKTRACE", "1");
    let ops = eframe::NativeOptions{
        initial_window_size:Some(egui::Vec2{x: 1200.0, y: 900.0}),
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "Geometric Brownian Motion",
        ops, 
        Box::new(|cc|Box::new(Sim::new(cc)))
    )
}
struct Sim{
    file_path:String,
    file_specified:bool,
    t_start_date:NaiveDate,
    t_end_date:NaiveDate,
    selected_steps:i64,
    paths:i64,
    selected_price_type:Option<PriceType>,
    predicted_price:f64,
    mu:f64,
    sigma:f64,
    sigma_sq:f64,
    plotting_vecs:Vec<Vec<f64>>,
    real_price:f64,
    step_size:f64,
    starting_price:f64,
}
impl Sim{
    fn new(cc: &eframe::CreationContext<'_>)->Self{
        setup(&cc.egui_ctx);
        Self {  
            file_path:String::new(),
            file_specified:false,
            t_start_date : NaiveDate::from_ymd_opt(2018,1,2).unwrap(),
            t_end_date : NaiveDate::from_ymd_opt(2020,1,2).unwrap(),
            selected_steps:0,
            paths:0,
            selected_price_type : None,
            predicted_price:0.0,
            mu:0.0,
            sigma:0.0,
            sigma_sq:0.0,
            plotting_vecs:Vec::new(),
            real_price:0.0,
            step_size:0.0,
            starting_price:0.0,
        }
    }
}

impl eframe::App for Sim{
    fn update(&mut self, ctx:&egui::Context, _frame:&mut eframe::Frame){
        start(ctx, &mut self.file_path, &mut self.file_specified, &mut self.t_start_date, &mut self.t_end_date, &mut self.selected_steps, &mut self.paths, &mut self.selected_price_type,
            &mut self.predicted_price,
            &mut self.mu,
            &mut self.sigma,
            &mut self.sigma_sq,
            &mut self.plotting_vecs,
            &mut self.real_price,
            &mut self.step_size,
            &mut self.starting_price);
    }
}

