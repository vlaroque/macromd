use crate::onemacro::OneMacro;
use colored::*;
use std::collections::HashMap;
use std::vec::*;

#[derive(PartialEq)]
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
    let mut macro_name = String::new();

    for letter in string_to_parse.chars() {
        match state {
            ParsingState::Normal => {
                if letter.is_uppercase() {
                    state = ParsingState::MacroName;
                    push_if_need(&mut final_string, &mut temp_string);
                }
                else if letter == '!' {
                    state = ParsingState::NoMacroName;
                }
            }
            ParsingState::MacroName => {
                if letter == '(' {
                    if macro_map.contains_key(&temp_string) {
                        state = ParsingState::WaintinClosePar;
                        macro_name = temp_string.clone(); // ici on sauvegarde le nom de la macro
                        temp_string.clear();
                        // final_string.push("(".to_string()); //a sauvegarder au cas ou la macro foire
                        open_parentheses_count = 1;
                        continue;
                    } else {
                        state = ParsingState::Normal;
                    }
                } else if !letter.is_uppercase() && letter != '_' {
                    state = ParsingState::Normal;
                }
            }
            ParsingState::NoMacroName => {
                if !letter.is_uppercase() && letter != '_' {
                    state = ParsingState::Normal;
                }
            }
            ParsingState::WaintinClosePar => {
                if letter == '(' {
                    open_parentheses_count += 1
                } else if letter == ')' {
                    if open_parentheses_count == 1 {
                        state = ParsingState::Normal;
                        // Here we call recursively the same finction on argument to threat them before macro executing
                        temp_string = parseline(&temp_string, &macro_map);
                        let current_macro: &OneMacro =
                            macro_map.get(&macro_name).expect("macro dont exist");
                        macro_name.clear();

                        temp_string = current_macro
                            .apply(temp_string.clone())
                            .expect("not a macro");

                        push_if_need(&mut final_string, &mut temp_string);
                        //final_string.push(")".to_string());     // il faudra la rajouter si ca foire
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
    if state == ParsingState::WaintinClosePar {
        push_if_need(&mut final_string, &mut macro_name);
        final_string.push("(".to_string());
        push_if_need(&mut final_string, &mut temp_string);
    }
    push_if_need(&mut final_string, &mut temp_string);
    //println!("PARSER RESULT = {:?}", final_string);
    let final_string = final_string.concat();

    return final_string;
}
