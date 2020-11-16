/*******************************************************************************
* Name: rust_bible_lookup.rs
* Author: Josiah Lansford
* Created: 16 November 2020
* Class: CS-3210 Programming Lang Survey, Prof. Dudenhofer
*
* Summary: Retrieves Bible verse according to reference given by user input.
* This verse is output on console and pretty-printed to file.
* Requires Bible.txt file and Bible_Abbreviations.csv in working directory 
* to retrieve Bible verses and find abbreviations.
*
* All test cases passed for this program.
*******************************************************************************/

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use csv::{Reader, StringRecord};

fn main() -> Result<(), Box<dyn Error>> {
    let abbreviations = read_abbreviations();
}

fn read_abbreviations(filename: String) -> Result<HashMap<String,String>, Box<dyn Error>> {
    let mut abbreviations = HashMap::new();
    let mut csvReader = Reader::from_path(&filename)?;
    for record in csvReader.records() {
        let record = record?;
        abbreviations.insert(&record.get(0), &record.get(1));
        println!("{:?}", record);
    }
    Ok(abbreviations)
}
