use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::Result;
use rand::*;

//Function to generate a random number to find a line
fn randomnum() -> u16{return rand::thread_rng().gen_range(1..6000)}

fn readfile() -> Result<()>{
    
    //Open file and buffered reader
    let file = File::open("./KanjiFile.csv").expect("File not found");
    let mut reader = BufReader::new(file);
    let mut line = String::new();
   
    let mut count = 0;
    while count < randomnum(){
        //Read line of file to store in buffer
        reader.read_line(&mut line).expect("Cannot read");
        count += 1;
    }
    //Make the read line into a vector of values
    let kanji: Vec<&str> = line.split(";").collect();

    //Get length of the vector
    let length = &kanji.len();

    output(&kanji,&length).expect("Error outputting Data");

    Ok(())
    
}

//Output function to the terminal
fn output(kanji: &Vec<&str>,length: &usize) -> Result<()>{
    println!("Level: {:?}", &kanji[length - 13]);
    println!("Kanji: {:?}",&kanji[length -12]);
    println!("Hiragana: {:?}", &kanji[length -11]);
    println!("Meaning: {:?}", &kanji[length-10]);
    println!("Sentence Example: {:?}", &kanji[length - 2]);
    println!("Sentence English: {:?}", &kanji[length - 5]);
    println!("Sentence Missing: {:?}", &kanji[length - 1].trim_end());
    Ok(())
}

fn main() {
    readfile().expect("Didnt work");
}
