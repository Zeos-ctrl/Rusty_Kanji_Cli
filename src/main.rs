use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::Result;
use rand::*;
use soloud::*;

struct Kanjidata {
    number: String,
    level: String,
    kanjichar: String,
    hiragana: String,
    meaning: String,
    spoken_kanji: String,
    word_type: String,
    sentence_1: String,
    sentence_2: String,
    sentence_eng: String,
    sentence_spoken: String,
    kanji_hira: String,
    sentence_3: String,
    sentence_missing: String,
}
//Function to generate a random number to find a line
fn randomnum() -> u16{return rand::thread_rng().gen_range(1..6000)}

fn getkanji() -> Kanjidata{
    
    //Open file and buffered reader
    let file = File::open("./KanjiFile.csv");

    let file = match file{
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening file: {:?}", error)
        },
    };

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
    
    let info = Kanjidata { 
        number: kanji[length-14].to_string(),
        level: kanji[length-13].to_string(),
        kanjichar: kanji[length-12].to_string(),
        hiragana: kanji[length-11].to_string(),
        meaning: kanji[length-10].to_string(),
        spoken_kanji: kanji[length-9].to_string(),
        word_type: kanji[length-8].to_string(),
        sentence_1: kanji[length-7].to_string(),
        sentence_2: kanji[length-6].to_string(),
        sentence_eng: kanji[length-5].to_string(),
        sentence_spoken: kanji[length-4].to_string(),
        kanji_hira: kanji[length-3].to_string(),
        sentence_3: kanji[length-2].to_string(),
        sentence_missing: kanji[length-1].to_string(),
    };

    return info;
    
}

//Output function to the terminal
fn output(kanji: &Kanjidata) -> Result<()>{
    println!("List Number: {:?}", &kanji.number);
    println!("Level: {:?}",&kanji.level);
    println!("Kanji: {:?}",&kanji.kanjichar);
    println!("Mixed Character: {:?}", &kanji.kanji_hira);
    println!("Hiragana: {:?}", &kanji.hiragana);
    println!("Meaning: {:?}", &kanji.meaning);
    println!("Word type: {:?}", &kanji.word_type);
    println!("Sentence Example: {:?}", &kanji.sentence_3);
    println!("Sentence English: {:?}", &kanji.sentence_eng);
    println!("Sentence Missing: {:?}", &kanji.sentence_missing);
    Ok(())
}

fn playsound(sound: &str) -> Result<()>{

    //Removes the begining part of the file path being [ sound:....]
    let fronttrim: Vec<&str> = sound.split(":").collect();
    //Formats the filepath from the vector
    let filepath  = format!("{}{}","./soundpack/",fronttrim[1]
                            .to_string()
                            .trim_end_matches(']'));

   //Using the crate Soloud this opens and plays the vocal sound using the
   //example displayed on the crate page
    let soloud = Soloud::default().expect("Failed to start Soloud");
    let mut audio = audio::Wav::default();
    audio.load(&std::path::Path::new(&filepath)).expect("File not loaded");

    soloud.play(&audio);
    while soloud.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    

    Ok(())
}

fn main() {
    //Add Menu
    //Add Error Handling
    let kanji = getkanji();

    let output = output(&kanji);
    let _output = match output{
        Ok(output) => output,
        Err(error) => {
            panic!("Output Function Failed: {:?}", error)
        },
    };

    let sound = playsound(&kanji.spoken_kanji);
    let _sound = match sound{
        Ok(sound) => sound,
        Err(error) => {
            panic!("Failed To play sound: {:?}", error)
        },
    };

}
