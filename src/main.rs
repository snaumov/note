use clap::{App, Arg};
use config::{Config, FileSourceFile, File};
use std::{fs::{OpenOptions}};
use std::io::prelude::*;
use dirs::home_dir;

fn main() {
    let mut settings = Config::new();
    let mut settings_file = home_dir().expect("no home in system");
    settings_file.push(".notes/Settings.toml");
    let add_settings_res = settings.merge::<File<FileSourceFile>>(settings_file.into());
    if add_settings_res.is_err() { println!("Cannot find the config, the program might not work as expected")};

    let matches = App::new("Notes")
        .author("Stepan Naumov")
        .version("0.0.1")
        .about("Note taking cmd line utility")
        .arg(Arg::new("note").index(1).multiple_occurrences(true))
        .arg(Arg::new("topic").short('t').long("topic").takes_value(true))
        .get_matches();
    
    let topic = matches.value_of("topic").expect("You must provide topic");
    let path = settings.get_str(&format!("topic.{}", topic)).expect("No topic is found in settings");
    let text_to_write = matches.values_of("note").expect("no text to write").collect::<Vec<&str>>().join(" ");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(path)
        .expect("Cannot open the file for writing");

    writeln!(file, "{}", text_to_write).expect("Cannot write to the file");
    println!("Succesfuly saved the note!");
}
