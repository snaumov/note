use clap::{App, Arg};
use config::{Config, FileSourceFile, File as ConfigFile};
use std::{fs::{self, OpenOptions, File}, path::Path};
use std::io::{prelude::*, self};
use std::env;
use dirs::home_dir;
use edit;
use tempfile::NamedTempFile;

fn set_editor(editor: Option<String>) -> impl Fn() -> () {
    let default_editor = env::var("EDITOR");
    if editor.is_some() { env::set_var("EDITOR", editor.unwrap()) };
    move || match &default_editor {
        Ok(editor) => { env::set_var("EDITOR", editor)},
        Err(_) => env::remove_var("EDITOR"),
    } 
}

fn update_text_file<P: AsRef<Path>>(data: &str, file_path: P, is_prepend: bool) -> io::Result<()> {
    match is_prepend {
        true => {
            let mut tmp = NamedTempFile::new()?;
            let mut src = File::open(&file_path)?;
            writeln!(tmp, "{}", data)?;
            io::copy(&mut src, &mut tmp)?;
            fs::remove_file(&file_path)?;
            fs::rename(&tmp.path(), &file_path)?;
        },
        false => {
            let mut file = OpenOptions::new()
                .append(true)
                .write(true)
                .open(file_path)?;
            writeln!(file, "{}", data)?;
        },
    }
    Ok(())
}

fn main() {
    let mut settings = Config::new();
    let mut settings_file = home_dir().expect("no home in system");
    settings_file.push(".note/Settings.toml");
    let add_settings_res = settings.merge::<ConfigFile<FileSourceFile>>(settings_file.into());
    if add_settings_res.is_err() { println!("Cannot find the config, the program might not work as expected")};


    let matches = App::new("Notes")
        .author("Stepan Naumov")
        .version("0.0.3")
        .about("Note taking cmd line utility")
        .arg(Arg::new("note").index(1).multiple_occurrences(true))
        .arg(Arg::new("topic").short('t').long("topic").takes_value(true).help("Topic to save the note for"))
        .arg(Arg::new("prepend").short('p').long("prepend").help("Set this flag to prepend the note to a topic. Default: false"))
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
            let is_prepend = matches.is_present("prepend");
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
                            text.strip_suffix("\n").unwrap_or(&text).to_string()
                        },
                        Err(_) => {
                            println!("Nothing to save");
                            return;
                        },
                    }
                }
            };

            update_text_file(&text_to_write, path, is_prepend).expect("Cannot update file");
            revert_editor();
            println!("Succesfuly saved the note!");
        },
        _ => unreachable!("this should not happen")
    }
    
}
