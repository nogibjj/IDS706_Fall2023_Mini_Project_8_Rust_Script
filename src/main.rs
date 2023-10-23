use std::fs::File;
use std::io::{BufRead, BufReader};

fn data_filter(df: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut filtered_data = Vec::new();
    for row in df.iter() {
        println!("{:?}", row[5]);
        if row[5].trim_start().parse::<f32>().unwrap_or(0.0) >= 80.0 {
            filtered_data.push(row.clone());
        }
    }
    filtered_data
}

fn main() {
    let file = File::open("./grades.csv").unwrap();
    let reader = BufReader::new(file);
    let mut dataframe = Vec::new();
    for line in reader.lines() {
        let row: Vec<String> = line.unwrap().split(",").map(|s| s.to_string()).collect();
        dataframe.push(row);
    }
    let filtered_data = data_filter(&dataframe);
    for row in filtered_data.iter() {
        println!("{:?}", row);
    }
    let series: Vec<f32> = dataframe
        .iter()
        .map(|row| row[5].trim_start().parse::<f32>().unwrap_or(0.0))
        .collect();
    println!("Mean: {}", series.iter().sum::<f32>() / series.len() as f32);
    println!("Median: {}", series[series.len() / 2]);
    let mean = series.iter().sum::<f32>() / series.len() as f32;
    let variance = series.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / series.len() as f32;
    println!("Standard Deviation: {}", variance.sqrt());
}
