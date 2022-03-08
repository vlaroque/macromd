use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::vec::*;

#[derive(Debug)]
pub struct OneMacro {
    pub name: String,
    pub arguments: Vec<String>,
    pub body: String,
}

impl OneMacro {
    pub fn apply(&self, arguments: String) -> Option<String> {
        let args = from_args_to_vec(&arguments);
        let mut returned_string = self.body.clone();

        println!("{} {:?} :: {:?}", "debug".red(), self, args);

        for (argument, param) in args.iter().zip(self.arguments.iter()) {
            returned_string = returned_string.replace(param, argument);
            println!(
                "!!! '{}' => '{}' == {}",
                param.red(),
                argument.green(),
                returned_string
            );
            returned_string = returned_string.replace(argument, &returned_string);
        }

        return Some(returned_string);
    }

    pub fn new() -> OneMacro {
        OneMacro {
            name: String::new(),
            arguments: vec![],
            body: String::new(),
        }
    }

    pub fn from_str(line: &str) -> Option<OneMacro> {
        let macro_regex_cap =
            Regex::new(r"!(?P<name>[A-Z]*)\((?P<args>.*)\)[ ]*\{(?P<corps>.*)\}").unwrap();
        let caps = macro_regex_cap.captures(line).unwrap();
        //let args_list = (&caps["args"]).to_string();
        //let args_list = args_list.split(",").collect::<Vec<&str>>();
        //let args_list = args_list.iter().map(|s| s.trim().to_string()).collect();
        let args_list = from_args_to_vec(&caps["args"]);
        Some(OneMacro {
            name: (&caps["name"]).to_string(),
            arguments: args_list,
            body: (&caps["corps"]).to_string(),
        })
    }

    pub fn from_string(line: &String) -> Option<OneMacro> {
        OneMacro::from_str(&line)
    }
}

fn from_args_to_vec(args: &str) -> Vec<String> {
    let args = args.to_string();
    let args = args.split(",").collect::<Vec<&str>>();

    args.iter().map(|s| s.trim().to_string()).collect()
}

pub fn process_string(string: &mut String, macro_list: &HashMap<String, OneMacro>) {
    //let regex = Regex::new(r"(?<!\!)(?P<name>[A-Z]*)\((?P<args>.*)\)").unwrap();
    let regex = Regex::new(r"([^!A-Z]|^)(?P<name>[A-Z]+)\((?P<args>.*?)\)").unwrap();

    loop {
        println!("new loop {}", string);
        let mut working_string = string.clone();
        let capture = match regex.captures(&working_string) {
            Some(cap) => cap,
            None => {
                println!("found none");
                break;
            }
        };
        let matching_string = &capture[0].trim();
        let macro_name = &capture["name"];
        let args: Vec<String> = from_args_to_vec(&capture["args"]);

        println!(
            "match {} name {} args {:?}",
            matching_string, macro_name, args
        );

        if !macro_list.contains_key(macro_name) {
            break;
        }

        let macro_pattern: &OneMacro = macro_list
            .get(macro_name)
            .expect("wtf macro should have been there");

        let mut macro_string = macro_pattern.body.clone();
        for (argument, param) in args.iter().zip(macro_pattern.arguments.iter()) {
            macro_string = macro_string.replace(param, argument);
            println!(
                "!!! '{}' => '{}' == {}",
                param.red(),
                argument.green(),
                macro_string
            );
            *string = working_string.replace(matching_string, &macro_string);
        }
        println!("end loop {}", string);
    }

    //        for caps in regex.captures_iter(&working_string) {
    //            let matching_string = &caps[0].trim();
    //            //println!("caps {:?}\n{:?}", caps, matching_string);
    //            let name = &caps["name"];
    //            let args: Vec<String> = from_args_to_vec(&caps["args"]);
    //            //println!("   - Occurence name='{}' args='{:?}'", name, args);
    //            // find associated macro in list
    //            if !macro_list.contains_key(name) {
    //                println!("!macro {} not contained in list", name);
    //                continue;
    //            }
    //            let matching_macro: &OneMacro = macro_list
    //                .get(name)
    //                .expect("wtf macro should have been there");
    //            //println!("Matched macro : {:?}", matching_macro);
    //
    //            let mut macro_string = matching_macro.body.clone();
    //            //println!("body = {}", macro_string);
    //            // make remplacement in string
    //            for (argument, param) in args.iter().zip(matching_macro.arguments.iter()) {
    //                macro_string = macro_string.replace(param, argument);
    //                println!(
    //                    "!!! '{}' => '{}' == {}",
    //                    param.red(),
    //                    argument.green(),
    //                    macro_string
    //                );
    //            }
    //            working_string = working_string.replace(matching_string, &macro_string);
    //            println!(
    //                "!!! '{}' => '{}' == {}",
    //                matching_string.red(),
    //                macro_string.green(),
    //                string
    //            );
    //        }
    //
    //string.to_string()
    //println!("     => captured {:?}", capture);
}
