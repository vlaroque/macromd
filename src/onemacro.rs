pub mod OneMacro {
    use std::vec::*;
    use regex::Regex;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct OneMacro {
        pub name: String,
        pub arguments: Vec<String>,
        pub body: String,
    }

    pub fn new() -> OneMacro {
        OneMacro {
            name : String::new(),
            arguments : vec![],
            body : String::new()
        }
    }

    pub fn from_str(line: &str) -> Option<OneMacro> {
        let macro_regex_cap =
            Regex::new(r"!(?P<name>[A-Z]*)\((?P<args>.*)\)[ ]*\{(?P<corps>.*)\}").unwrap();
        let caps = macro_regex_cap.captures(line).unwrap();
        let args_list = (&caps["args"]).to_string();
        let args_list = args_list.split(",").collect::<Vec<&str>>();
        let args_list = args_list.iter().map(|s| s.trim().to_string()).collect();
        Some( OneMacro {
            name: (&caps["name"]).to_string(),
            arguments: args_list,
            body: (&caps["corps"]).to_string(),
        })
    }

    pub fn from_string(line: &String) -> Option<OneMacro> {
        from_str(&line)
    }
    
    pub fn process_string(string : &mut String, macro_list : &HashMap<String, OneMacro>) {
        //let regex = Regex::new(r"(?<!\!)(?P<name>[A-Z]*)\((?P<args>.*)\)").unwrap();
        let regex = Regex::new(r"([^!A-Z]|^)(?P<name>[A-Z]+)\((?P<args>.*?)\)").unwrap();
        let capture = match regex.captures(string) {
            Some(cap) => cap,
            None => return
        };

        for caps in regex.captures_iter(string){
            println!("Occurence name='{}' args='{}'", &caps["name"], &caps["args"]);
        }

        //println!("     => captured {:?}", capture);
    }
}
