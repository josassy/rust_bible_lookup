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
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use csv::Reader;

fn main() {
    
    // Read in abbreviations, store in HashMap
    let mut abbreviations = HashMap::new();
    read_abbreviations("Bible_Abbreviations.csv".to_string(), &mut abbreviations).expect("could not read abbreviations");

    // Open the file
    let f = File::open("Bible.txt").expect("can't open bible file");
    let mut bible = std::io::BufReader::new(f);

    println!("Welcome to the Bible, brought to you by Rust!");
    println!("Type \"EXIT\" at any time to quit.\n");

    // Begin main program loop
    loop {

        // look for book
        let mut found = false;
        let mut book = String::new();
        let mut chapter = String::new();
        let mut verse = String::new();
        while !found {
            book.clear();
            print_fl("Enter Book: ");
            io::stdin().read_line(&mut book).unwrap();
            book = book.trim().to_uppercase();
            check_quit(&book);

            // if book in list of abbreviations, replace
            if abbreviations.contains_key(&book) {
                book = abbreviations.get(&book).unwrap().to_uppercase();
                println!("Replaced with {:?}", book);
            }

            // Reset buffer to beginning
            let _ = bible.seek(std::io::SeekFrom::Start(0));
            found = search_book(&mut bible, &book);

            // If book was not found, start over
            if !found {
                println!("Could not find book {:?}", book);
            }
        }

        // look for chapter
        let chapter_search_pos = bible.seek(std::io::SeekFrom::Current(0)).unwrap();
        found = false;
        while !found {
            chapter.clear();
            print_fl("Enter Chapter: ");
            io::stdin().read_line(&mut chapter).unwrap();
            chapter = chapter.trim().to_uppercase();
            check_quit(&chapter);
            if chapter == "RESET" {
                break;
            }
            let _ = bible.seek(std::io::SeekFrom::Start(chapter_search_pos));
            found = search_chapter(&mut bible, &chapter);
            if !found {
                println!("Could not find chapter {} in {}.", chapter, book);
                println!("Try again or enter \"RESET\" to restart query.");
            }
        }
        // if chapter was not found and we are out of loop, reset to beginning
        if !found {
            continue;
        }

        // look for verse
        let verse_search_pos = bible.seek(std::io::SeekFrom::Current(0)).unwrap();
        let mut verse_result = String::new();
        while verse_result.is_empty() {
            verse.clear();
            print_fl("Enter Verse: ");
            io::stdin().read_line(&mut verse).unwrap();
            verse = verse.trim().to_uppercase();
            check_quit(&verse);
            if verse == "RESET" {
                break;
            }
            let _ = bible.seek(std::io::SeekFrom::Start(verse_search_pos));
            verse_result = search_verse(&mut bible, &verse);
            if verse_result.is_empty() {
                println!("Could not find verse {} in {} {}.", verse, book, chapter);
                println!("Try again or enter \"RESET\" to restart query.");
            }
        }
        // if chapter was not found and we are out of loop, reset to beginning
        if verse_result.is_empty() {
            continue;
        }
        
        // print the found verse
        let output = pretty_print(&book, &chapter, &verse, &verse_result);
        println!("The verse you requested is:");
        println!("{}", output);

        // write to file
        let mut out_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("verses.txt")
            .expect("Unable to open output verses.txt file");
        writeln!(out_file, "{}", output).expect("Unable to write to verses.txt file");

        // ask if user wants another verse
        print_fl("Do you want to look up another verse? (y/n): ");
        let mut should_quit = String::new();
        io::stdin().read_line(&mut should_quit).unwrap();
        should_quit = should_quit.trim().to_uppercase();
        check_quit(&should_quit);
        if &should_quit[..1] != "Y" {
            break;
        }
    }
}


/**
 * Search the Bible.txt file for the desired book
 */
fn search_book(file: &mut impl BufRead, book: &String) -> bool {
    let mut line = String::new();
    loop {
        line.clear();
        let len = file.read_line(&mut line).unwrap();
        // if line empty reached end of file
        if line == "" {
            return false;
        }

        line = line.trim().to_uppercase();

        // found book
        if len >= 13 &&
           &line[..11] == "THE BOOK OF" && 
           &line[12..] == &book[..].to_uppercase() {
            return true;
        }
    }
}


/**
 * Search the Bible.txt file for the desired chapter
 */
fn search_chapter(file: &mut impl BufRead, chapter: &String) -> bool {
    let mut line = String::new();
    loop {
        line.clear();
        let _ = file.read_line(&mut line).unwrap();
        
        // if line empty or reach "THE BOOK OF", went too far.
        if line == "" || line.contains("THE BOOK OF") {
            return false;
        }

        line = line.trim().to_uppercase();

        // split line into tokens
        let split_line: Vec<&str> = line.split(' ').collect();

        if split_line.len() > 1 {
            let prefix = split_line.get(0).unwrap();
            let number = split_line.get(1).unwrap();
            // If the line begins with PSALM or CHAPTER and the number matches, we found it.
            if ["PSALM", "CHAPTER"].contains(prefix) && number.trim() == chapter {
                return true;
            }
        }
    }
}


/**
 * Search the Bible.txt file for the desired verse
 */
fn search_verse(file: &mut impl BufRead, verse: &String) -> String {
    let mut line = String::new();
    let mut result = String::new();
    loop {
        line.clear();
        let len = file.read_line(&mut line).unwrap();
        line = line.trim().to_uppercase();
        
        // If string empty or we reach any of these tokens, went too far
        if line == "" {
            return result;
        }
        for pattern in ["CHAPTER", "PSALM", "THE BOOK OF"].iter() {
            if len >= pattern.len() && &line[..pattern.len()] == *pattern {
                return result;
            }
        }

        // found verse
        if len >= verse.len() && &line[..verse.len()] == verse {
            result = line[verse.len()+1..].trim().to_string();
            return result;
        }
    }
}


/**
 * Helper function to load the abbreviations csv file into a HashMap
 * @param filename: The name of the csv file
 * @param hash_map: mutable reference to the dict to populate
 */
fn read_abbreviations(filename: String, hash_map: &mut HashMap<String,String>) -> Result<(), Box<dyn Error>> {
    let mut csv_reader = Reader::from_path(&filename)?;
    for record in csv_reader.records() {
        let record = record?;
        hash_map.insert(record.get(0).unwrap().to_uppercase(), record.get(1).unwrap().to_string());
    }
    Ok(())
}


/**
 * Helper function to check if the program should exit after a prompt
 * @param input_str: The user's input to check
 */
fn check_quit(input_str: &String) {
    if input_str.to_uppercase().contains("EXIT") || input_str.to_uppercase().contains("QUIT") {
        std::process::exit(0)
    }
}


/**
 * Helper function to pretty_print a string onto multiple lines, delimited at 
 * 80 chars. Will break on a word division.
 */
fn pretty_print(book: &String, chapter: &String, verse: &String, verse_result: &String) -> String {
    let mut output_str = format!("{} {}:{} {}", book, chapter, verse, verse_result);
    let mut working_index = 0;
    while &output_str[working_index..].len() > &80 {
        working_index += 80;
        loop {
            // If found space, replace it with newline
            // Rust is a total pain with string indexing, so we are going with this
            if &output_str[working_index..working_index+1] == " " {
                let mut replace_result: String = output_str[..working_index].to_string();
                replace_result.push('\n');
                replace_result.push_str(&output_str[working_index+1..]);
                output_str = replace_result;
                working_index += 1;
                break;
            }
            else {
                working_index -= 1;
            }
        }
    }
    return output_str;
}


/**
 * Helper function to print and flush buffer. 
 * Useful for printing and receiving input on the same line.
 */
fn print_fl(input: &str) {
    print!("{}", &input);
    let _ = io::stdout().flush();
}