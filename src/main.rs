use regex::Regex;
use std::env;
use std::fs;
use std::collections::HashMap;
mod onemacro;
use onemacro::OneMacro;

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
    let mut macro_list : HashMap<String, OneMacro::OneMacro> = HashMap::new();

    for line in lines {
        let mut line : String = String::from(line);
        println!("line: {}", line);

        OneMacro::process_string(&mut line, &macro_list);
        
        if macro_regex.is_match(&line) {
            println!("   >>> MACRO DECLARATION!");
            let one_macro = match OneMacro::from_string(&line) {
                Some(onemacro) => macro_list.insert(onemacro.name.clone(), onemacro),
                None => continue
            };
        }

    }
    println!("\n\n   ====>>> MACRO LIST {:?}", macro_list);
}
