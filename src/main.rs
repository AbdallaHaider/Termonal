use std::io;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use std::error::Error;

use csv::Reader;

fn main() {
    let palavras_folder = "data/palavras.csv";
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(palavras_folder);
    
    let mut words: Vec<String> = Vec::new();
    
    for result in reader.records() {
        let record = result?;
        
        if let Some(word) = record.get(0) {
            words.push(word.to_string());
        }
    }
    }