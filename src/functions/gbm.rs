use std::{f64::consts::E};
use std::fs::File;
use csv::Reader;
use egui;
use chrono::{NaiveDate, DateTime, Local, Datelike};
extern crate csv;
fn gbm(file_path:&mut String){
    let user_data = PriceDate::read_csv(file_path, true);
}


#[derive(Debug)]
struct PriceDate{
    headers:csv::StringRecord,
    date:Vec<String>,
    open:Vec<f64>,
    high:Vec<f64>,
    low:Vec<f64>,
    close:Vec<f64>,
    adj_close:Vec<f64>,
    volumn:Vec<f64>
}
impl PriceDate{
    fn new()->PriceDate{
        PriceDate { headers: csv::StringRecord::new(), 
            date:Vec::new() , 
            open: Vec::new(), 
            high: Vec::new(), 
            low: Vec::new(), 
            close: Vec::new(), 
            adj_close: Vec::new(), 
            volumn: Vec::new() 
        }
    }
    fn read_csv(file_path:&String, has_headers:bool)->PriceDate{
        let file = std::fs::File::open(file_path).unwrap();
        let mut reader:Reader<File> = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);
        let mut price_date:PriceDate = PriceDate::new();
        for i in reader.records().into_iter(){
            let record = i.unwrap();
            price_date.push(&record);
        }
        return price_date;
    }
    fn push(&mut self, row:&csv::StringRecord){
        self.date.push(row[0].to_string());
        self.open.push(row[1].parse().unwrap());
        self.high.push(row[2].parse().unwrap());
        self.low.push(row[3].parse().unwrap());
        self.close.push(row[4].parse().unwrap());
        self.adj_close.push(row[5].parse().unwrap());
        self.volumn.push(row[6].parse().unwrap());
    }
}