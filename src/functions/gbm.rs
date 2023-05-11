use std::{f64::consts::E};
use std::fs::File;
use csv::Reader;
use chrono::NaiveDate;
use rand_distr::{Normal, Distribution};
use std::{thread,sync::{Arc, Mutex}};
use crate::app::main_page::PriceType;
extern crate csv;



pub fn gbm(file_path:&String, t_start_date:NaiveDate, t_end_date:NaiveDate, selected_steps:&mut i64, paths:&mut i64, selected_price:&mut Option<PriceType>,
            predicted_price:&mut f64,
            mu_hat:&mut f64,
            sigma_hat:&mut f64,
            sigma_sq_hat:&mut f64,
            plotting_vecs:&mut Vec<Vec<f64>>,
            real_price:&mut f64,
            step_size:&mut f64,
            starting_price:&mut f64 ){
    let price_data = PriceData::read_csv(file_path, true);
    
//need to check if date specified in t_start and t_end are actually in the data set and send a message to the main page.
    let start_index = price_data.date
        .iter()
        .position(|s| {
            *s == t_start_date
        }).unwrap_or_default();
    let end_index = price_data.date
        .iter()
        .position(|s| {
            *s == t_end_date
        })
        .unwrap_or_default();
    
    let training_prices:&[f64];
    match *selected_price{
        Some(PriceType::Open) => training_prices = &price_data.open[start_index..end_index],
        Some(PriceType::Low) => training_prices = &price_data.low[start_index..end_index],
        Some(PriceType::Close) => training_prices = &price_data.close[start_index..end_index],
        Some(PriceType::High) => training_prices = &price_data.high[start_index..end_index],
        Some(PriceType::Adjclose) => training_prices = &price_data.adj_close[start_index..end_index],
        None => training_prices = &[],
    }
    

    let mu: f64 = (training_prices.iter()
        .map(|&e| e.ln())
        .zip(training_prices.iter().skip(1).map(|&p| p.ln()))
        .map(|(e,p)| p - e)
        .collect::<Vec<_>>()
        .iter()
        .map(|e| *e)
        .sum::<f64>()) / (training_prices.len() - 1) as f64;

    let sigma_sq: f64 = (training_prices.iter()
        .map(|&e| e.ln())
        .zip(training_prices.iter().skip(1).map(|&p| p.ln()))
        .map(|(e,p)| p - e)
        .collect::<Vec<_>>()
        .iter()
        .map(|e| (*e - mu).powf(2.0))
        .sum::<f64>()) / (training_prices.len() - 2) as f64;

    let sigma:f64 = sigma_sq.sqrt();//std_dev
    let normalized_sigma:f64 = sigma * (*selected_steps as f64).sqrt();
    let nromalized_sigma_sq = normalized_sigma.sqrt();

    let delta_t:f64 = 1.0/ *selected_steps as f64;

    //starting simulation part
    let plot_vecs_size:usize = 50;//number of vectors that will be used for plotting purposes
 
    let mut plot_vec:Vec<Vec<f64>> = Vec::new();
    

    let initial_price:f64 = training_prices[training_prices.len()-1];//last value of the training data will be used as the first price that the simulation will build on

    for _j in 0..plot_vecs_size{
        let mut sim_vec:Vec<f64> = Vec::with_capacity(*selected_steps as usize + 1);
        sim_vec.push(initial_price);
        let mut i:usize = 1;
        while i <= *selected_steps as usize{
            let index:usize = i;
            let normal = Normal::new(mu, delta_t.sqrt()).unwrap();
            let rng_value:f64 = normal.sample(&mut rand::thread_rng());
            let value:f64 = sim_vec[index - 1];
            let operation:f64 = value * (E.powf(mu - (0.5 * normalized_sigma) * delta_t + (nromalized_sigma_sq * rng_value)));
            i+=1;
            sim_vec.push(operation);
        }
        plot_vec.push(sim_vec);
    }
    *plotting_vecs = plot_vec;
    let predicted_days = *selected_steps;
    //paralized calculation for the gbm final price (which is the only one that matters)
    let optimized_vec_mutex = Arc::new(Mutex::new(Vec::with_capacity(*paths as usize)));
    let paths_per_thread = (*paths as f64 / num_cpus::get() as f64).ceil() as usize;//finds the number of cpu cores that the calculation can run on and creates a specified chunk size
    let handles: Vec<_> = (0..num_cpus::get()).map(|i| {
        let optimized_vec_mutex_cloned = optimized_vec_mutex.clone();
        let start = i * paths_per_thread;
        let end = usize::min(start + paths_per_thread, *paths as usize);
        thread::spawn(move || {//spawns thread for each cpu core to run the below calculations
            let mut sim_vec: Vec<f64> = Vec::with_capacity(predicted_days as usize + 1);
            sim_vec.push(initial_price);
            let mut i: usize = 1;
            while i <= predicted_days as usize {
                let normal = Normal::new(mu, delta_t.sqrt()).unwrap();
                let rng_value: f64 = normal.sample(&mut rand::thread_rng());
                let value: f64 = sim_vec[i - 1];
                let operation: f64 = value * (E.powf(mu - (0.5 * normalized_sigma) * delta_t + (nromalized_sigma_sq * rng_value)));
                sim_vec.push(operation);
                i += 1;
            }
            let last_value = *sim_vec.last().unwrap();
            let mut optimized_vec_guard = optimized_vec_mutex_cloned.lock().unwrap();
            for _j in start..end {
                optimized_vec_guard.push(last_value);
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
    //sending the results to the Sim object variables so they're accessible on the main gui
    let optimized_vec = optimized_vec_mutex.lock().unwrap().clone();//collected vector with the final prices of the simulation 
    *predicted_price = (optimized_vec.iter().sum::<f64>()) / optimized_vec.len() as f64;
    *mu_hat = mu;
    *sigma_hat = sigma;
    *sigma_sq_hat = sigma_sq;
    *step_size = predicted_days as f64;
    *starting_price = initial_price;
    
    


    //getting actual price on predicted day
    match *selected_price{
        Some(PriceType::Open) => *real_price = price_data.open[end_index+ *selected_steps as usize],
        Some(PriceType::Low) => *real_price = price_data.low[end_index+ *selected_steps as usize],
        Some(PriceType::Close) => *real_price = price_data.close[end_index+ *selected_steps as usize],
        Some(PriceType::High) => *real_price = price_data.high[end_index+ *selected_steps as usize],
        Some(PriceType::Adjclose) => *real_price = price_data.adj_close[end_index+ *selected_steps as usize],
        None => *real_price = 0.0,
    }

}

//data structure to hold the csv file price data
pub struct PriceData{
    _headers:csv::StringRecord,
    date:Vec<NaiveDate>,
    open:Vec<f64>,
    high:Vec<f64>,
    low:Vec<f64>,
    close:Vec<f64>,
    adj_close:Vec<f64>,
    volumn:Vec<f64>
}
//methods for the pricedata struct
impl PriceData{
    fn new()->PriceData{
        PriceData { _headers: csv::StringRecord::new(), 
            date:Vec::new() , 
            open: Vec::new(), 
            high: Vec::new(), 
            low: Vec::new(), 
            close: Vec::new(), 
            adj_close: Vec::new(), 
            volumn: Vec::new() 
        }
    }
    //function that actually makes the price data object
    fn read_csv(file_path:&String, has_headers:bool)->PriceData{
        let file = std::fs::File::open(file_path).unwrap();
        let mut reader:Reader<File> = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);
        let mut price_date:PriceData = PriceData::new();
        for i in reader.records().into_iter(){
            let record = i.unwrap();
            price_date.create(&record);
        }
        return price_date;
    }
    //function that creates a row of data 
    fn create(&mut self, row: &csv::StringRecord) {
        let date_str = row[0].to_string();
        let date = NaiveDate::parse_from_str(&date_str, "%m/%d/%Y").unwrap();
        self.date.push(date);
        self.open.push(row[1].parse().unwrap());
        self.high.push(row[2].parse().unwrap());
        self.low.push(row[3].parse().unwrap());
        self.close.push(row[4].parse().unwrap());
        self.adj_close.push(row[5].parse().unwrap());
        self.volumn.push(row[6].parse().unwrap());
    }
}

