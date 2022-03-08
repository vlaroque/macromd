
use colored::*;
use std::collections::HashMap;
use std::vec::*;
//mod onemacro;
use crate::onemacro::OneMacro;

enum ParsingState {
    Normal,
    MacroName,
    NoMacroName,
    WaintinClosePar,
}

fn push_if_need(vector: &mut Vec<String>, string: &mut String) {
    if !string.is_empty() {
        vector.push(string.clone());
        string.clear();
    }
}

pub fn parseline(string_to_parse: &String, macro_map: &HashMap<String, OneMacro>) -> String {
    let mut final_string: Vec<String> = vec![];
    let mut temp_string: String = String::new();
    let mut open_parentheses_count = 0;
    let mut state = ParsingState::Normal;
    let empty = OneMacro::new();
    let mut current_macro : &OneMacro = &empty;    //on va surement l'enlever pour que cet appel soit au moment de (
    let mut macro_name = String::new();

    println!("{} in parseline", "debug".red());

    for letter in string_to_parse.chars() {
        match state {
            ParsingState::Normal => {
                if letter.is_uppercase() {
                    state = ParsingState::MacroName;
                    push_if_need(&mut final_string, &mut temp_string);
                }
            }
            ParsingState::MacroName => {
                if letter == '(' {
                    if macro_map.contains_key(&temp_string) {
                        state = ParsingState::WaintinClosePar;
                        final_string.push(temp_string.clone()); //ici on isole le nom de la Macro
                        current_macro = macro_map.get(&temp_string).expect("macro dont exist");
                        println!("{} get macro {:?}", "debug".red(), current_macro);
                        temp_string.clear();
                        final_string.push("(".to_string());
                        open_parentheses_count = 1;
                        continue;
                    } else {
                        state = ParsingState::Normal;

                    }
                } else if !letter.is_uppercase() {
                    state = ParsingState::Normal;
                }
            }
            ParsingState::NoMacroName => {}
            ParsingState::WaintinClosePar => {
                if letter == '(' {
                    open_parentheses_count += 1
                } else if letter == ')' {
                    if open_parentheses_count == 1 {
                        state = ParsingState::Normal;
                        // C'est ici que l'on doit rappeler cette meme methode
                        temp_string = parseline(&temp_string, &macro_map);
                        temp_string = current_macro.apply(temp_string.clone()).expect("not a macro");
                        println!("{} get macro before apply {:?}", "debug".red(), current_macro);

                        push_if_need(&mut final_string, &mut temp_string);
                        final_string.push(")".to_string());
                        continue;
                    } else if open_parentheses_count > 1 {
                        open_parentheses_count -= 1;
                    } else {
                        panic!("never should happen")
                    }
                }
            }
        }
        temp_string.push(letter);
    }
    push_if_need(&mut final_string, &mut temp_string);
    println!("PARSER RESULT = {:?}", final_string);
    let final_string = final_string.concat();

    println!("{} out parseline", "debug".red());

    return final_string;
}
