use clap::{App, Arg};
use config::{Config, FileSourceFile, File};
use std::{fs::{OpenOptions}};
use std::io::prelude::*;
use std::env;
use dirs::home_dir;
use edit;

fn set_editor(editor: Option<String>) -> impl Fn() -> () {
    let default_editor = env::var("EDITOR");
    if editor.is_some() { env::set_var("EDITOR", editor.unwrap()) };
    move || match &default_editor {
        Ok(editor) => { env::set_var("EDITOR", editor)},
        Err(_) => env::remove_var("EDITOR"),
    } 
}

fn main() {
    let mut settings = Config::new();
    let mut settings_file = home_dir().expect("no home in system");
    settings_file.push(".notes/Settings.toml");
    let add_settings_res = settings.merge::<File<FileSourceFile>>(settings_file.into());
    if add_settings_res.is_err() { println!("Cannot find the config, the program might not work as expected")};


    let matches = App::new("Notes")
        .author("Stepan Naumov")
        .version("0.0.3")
        .about("Note taking cmd line utility")
        .arg(Arg::new("note").index(1).multiple_occurrences(true))
        .arg(Arg::new("topic").short('t').long("topic").takes_value(true))
        .subcommand(App::new("topics")
            .about("Shows list of topics"))
        .get_matches();

    let subcommands = matches.subcommand();

    match subcommands {
        Some(("topics", _)) => {
            // List all topics
            let topics = settings.get_table("topic").expect("No topics found in settings");
            for topic in topics.keys() {
                println!("{}", topic);
            }
        },
        None => {
            // TODO: Consider adding a subcommand for adding a note (ie "new")
            // If no subcommand is provided, it's a saving note situtation
            let topic = matches.value_of("topic").expect("You must provide topic");
            let path = settings.get_str(&format!("topic.{}", topic)).expect("No topic is found in settings");
            let revert_editor = set_editor(settings.get_str("general.editor").ok());
            let text_to_write = match matches.values_of("note") {
                Some(text) => text.collect::<Vec<&str>>().join(" "),
                None => {
                    match edit::edit("") {
                        Ok(text) => {
                            if text == "" {
                                println!("Nothing to save");
                                return;
                            }
                            text
                        },
                        Err(_) => {
                            println!("Nothing to save");
                            return;
                        },
                    }
                }
            };

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .write(true)
                .open(path)
                .expect("Cannot open the file for writing");

            writeln!(file, "{}", text_to_write).expect("Cannot write to the file");
            revert_editor();
            println!("Succesfuly saved the note!");
        },
        _ => unreachable!("this should not happen")
    }
    
}
