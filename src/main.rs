mod onemacro;
use clap::{arg, command};
use colored::*;
use onemacro::OneMacro;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn arg_parse() -> Option<(String, String)> {
    let mut in_filename = String::new();
    let mut out_filename = in_filename.to_string();
    let matches = command!()
        .arg(arg!([filename] "markdown file name to compile").required(true))
        .arg(arg!(-o --output <FILE> "Sets a custom output file").required(false))
        .get_matches();

    if let Some(filename) = matches.value_of("filename") {
        in_filename = filename.to_string();
    }

    let in_file_path = Path::new(matches.value_of("filename").expect("fail"));

    if let Some(filename) = matches.value_of("output") {
        out_filename = filename.to_string();
    } else {
        out_filename = in_filename.clone();
        out_filename.push_str(".md");
    }

    return Some((in_filename, out_filename));
}

fn main() {
    let (in_filename, out_filename) = arg_parse().expect("failed to parse args");

    let mut out_lines: Vec<String> = Vec::new();

    println!("filename = {}", in_filename);
    let content = match fs::read_to_string(in_filename) {
        Ok(content) => content,
        Err(err) => {
            println!("failed to get content with err {}", err);
            return;
        }
    };

    let macro_regex = Regex::new(r"![A-Z]*\(.*\)[ ]*\{.*\}").unwrap();
    let lines = content.lines();
    let mut macro_list: HashMap<String, onemacro::OneMacro> = HashMap::new();

    println!("WTF {:?}", out_filename);
    let mut file = File::create(&out_filename).expect("failed openfile");

    for line in lines {
        let mut line: String = String::from(line);
        println!("{}: {}", "LINE".bright_yellow(), line);
        line = onemacro::parseline(&line, &macro_list);

        if macro_regex.is_match(&line) {
            println!("   >>> MACRO DECLARATION!");
            let one_macro = match OneMacro::from_string(&line) {
                Some(onemacro) => macro_list.insert(onemacro.name.clone(), onemacro),
                None => continue,
            };
            println!("   MACRO: {:?}", one_macro);
        } else {
            writeln!(&mut file, "{}", line).expect("failed on write to file");
        }
        println!("{}: {}", "RESULT".bright_green(), line);
    }
    println!("\n\n   ====>>> MACRO LIST {:?}", macro_list);
}
