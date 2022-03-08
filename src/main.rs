use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
mod onemacro;
//use onemacro::OneMacro;
use onemacro::OneMacro;
mod lineparser;
//use lineparser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            println!("failed to get content with err {}", err);
            return;
        }
    };
    println!("filename = {}", filename);

    let macro_regex = Regex::new(r"![A-Z]*\(.*\)[ ]*\{.*\}").unwrap();
    let lines = content.lines();
    let mut macro_list: HashMap<String, onemacro::OneMacro> = HashMap::new();

    for line in lines {
        let mut line: String = String::from(line);
        println!("{}: {}", "LINE".bright_yellow(), line);
        lineparser::parseline(&line, &macro_list);
        //OneMacro::process_string(&mut line, &macro_list);

        if macro_regex.is_match(&line) {
            println!("   >>> MACRO DECLARATION!");
            let one_macro = match OneMacro::from_string(&line) {
                Some(onemacro) => macro_list.insert(onemacro.name.clone(), onemacro),
                None => continue,
            };
            println!("   MACRO: {:?}", one_macro);
        }
        println!("{}: {}", "RESULT".bright_green(), line);
    }
    println!("\n\n   ====>>> MACRO LIST {:?}", macro_list);
}
