use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
mod onemacro;
use onemacro::OneMacro;
mod lineparser;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut out_filename = filename.to_string();
    out_filename.push_str(".md");
    let mut out_lines: Vec<String> = Vec::new();
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
    
    println!("WTF {:?}", out_filename);
    let mut file = File::create(&out_filename).expect("failed openfile");

    for line in lines {
        let mut line: String = String::from(line);
        println!("{}: {}", "LINE".bright_yellow(), line);
        line = lineparser::parseline(&line, &macro_list);

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
