use app::main_page::start;
use egui::{Margin, Color32, RichText, FontDefinitions};
use eframe::egui;
mod functions{
    pub mod gbm;
}
mod app{
    pub mod main_page;
}
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
}
impl Sim{
    fn new(cc: &eframe::CreationContext<'_>)->Self{
        setup(&cc.egui_ctx);
        Self {  
            file_path:String::new(),
        }
    }
}

impl eframe::App for Sim{
    fn update(&mut self, ctx:&egui::Context, _frame:&mut eframe::Frame){
        start(ctx, &mut self.file_path);
    }
}