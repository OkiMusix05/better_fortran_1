use std::ptr::replace;
use regex::Regex;

enum Tokens {
    Comments,
    Declaration, /// For variables and constants
    MainFunction,
    Expressions,
    Print, // Doesn't support formats yet
    MathFuncs, // Includes pi, e, G
    //If,
}

impl Tokens {
    fn pattern(&self) -> (&'static str, Regex) {
        match self {
            /*
            fn main() {
                code here
            }
             */
            Tokens::MainFunction => ("", Regex::new(r"fn main\(\) \{((?s).*)}").unwrap()),
            // // Comment
            Tokens::Comments => ("", Regex::new(r"//(.+)").unwrap()),
            // let name:type = value;
            Tokens::Declaration => ("", Regex::new(r"(const|let) ([a-zA-Z][a-zA-Z0-9]+|[a-zA-Z]) *: *([a-zA-Z][a-zA-Z0-9]+) *= *((?s).*?);").unwrap()),
            // var1 = expression;
            Tokens::Expressions => ("", Regex::new(r"([a-zA-Z][a-zA-Z0-9]+|[a-zA-Z]) *= *(.*);").unwrap()),
            // print(thing);
            Tokens::Print => ("", Regex::new(r"print\((.*)\);").unwrap()),
            /* if(condition) {
                    code here
               } else {
                    more code
               }
             */
            //Tokens::If => ("", Regex::new(r"if *\((.*)\) *\{((?s).*?)}").unwrap()),
            // use math::{pi, e, ...}
            Tokens::MathFuncs => ("", Regex::new(r"use math::\{(.*)}").unwrap()),
        }
    }
    fn apply_replacement(&self, captures: &regex::Captures) -> String {
        match self {
            Tokens::MainFunction => {
                let content = captures.get(1).map_or("", |m| m.as_str());
                format!("Program main\nimplicit none{}end program", content)
            }
            Tokens::Comments => {
                let comment = captures.get(1).map_or("", |m| m.as_str());
                let newline = captures.get(2).map_or("", |m| m.as_str());
                format!("!{}{}", comment, newline)
            }
            Tokens::Declaration => {
                let constorvar = captures.get(1).map_or("", |m| m.as_str());
                let name = captures.get(2).map_or("", |m| m.as_str());
                let mut data_type = captures.get(3).map_or("", |m| m.as_str());
                let mut value = captures.get(4).map_or("", |m| m.as_str());
                let keyword:&str;
                let mut mtx:String = String::from("");
                let mut is_matrix:String = String::from("");
                let mut data_type_string:String = String::from("");
                let mut value_string:String = String::from("");
                if constorvar == "const" {
                    keyword = ", parameter";
                } else {
                    keyword = "";
                }
                if data_type.contains("MAT") {
                    let size = Regex::new(r"([0-9]+)x([0-9]+)").unwrap();
                    if let Some(size_caps) = size.captures(&data_type) {
                        let width = size_caps.get(1).unwrap().as_str();
                        let height = size_caps.get(2).unwrap().as_str();
                        mtx = format!("{}x{}", width, height);
                        data_type_string = data_type.replace("MAT", "").replace(&mtx, "");
                        is_matrix = format!(", dimension ({}, {})", width, height);
                    }
                    value_string = value.replace("{", "").replace("}", "")
                        .replace("\n", "").replace(" ", "").replace("|", ",");
                    value_string = format!("reshape((/{}/), shape({}), order=(/2,1/))", value_string, name);
                } else {
                    data_type_string = data_type.to_string();
                    value_string = value.to_string();
                }
                match data_type_string.as_str() {
                    "int" => format!("integer{}{} :: {} = {}", keyword, is_matrix, name, value_string),
                    "f4" => format!("real{}{} :: {} = {}", keyword, is_matrix, name, value_string),
                    "f8" => format!("real*8{}{} :: {} = {}", keyword, is_matrix, name, value_string),
                    _ => format!("{} :: {} = {}", data_type, name, value), // Handle other data types
                }
            }
            Tokens::Expressions => {
                let recieve = captures.get(1).map_or("", |m| m.as_str());
                let exp = captures.get(2).map_or("", |m| m.as_str());
                format!("{} = {}", recieve, exp)
            }
            Tokens::Print => {
                let thing = captures.get(1).map_or("", |m| m.as_str());
                format!("print*, {}", thing)
            }
            /*Tokens::If => {

                let condition = captures.get(1).map_or("", |m| m.as_str());
                let content = captures.get(2).map_or("", |m| m.as_str());
                format!("if({}) then{}endif", condition, content)
            }*/
            Tokens::MathFuncs => {
                let imports_str = captures.get(1).map_or("", |m| m.as_str());
                let imports_list:Vec<&str> = if imports_str.is_empty() {
                    Vec::new()
                } else {
                    imports_str.split(",").collect()
                };
                //let mut const_declarations:&str = "real*8 ::";
                let mut const_decl_vec:Vec<&str> = vec![];
                for import in imports_list {
                    let imp = import.trim();
                    match imp {
                        "pi" => const_decl_vec.push("pi=4.D0*DATAN(1.D0)"),
                        "e" => const_decl_vec.push("e=EXP(1.0)"),
                        "G" => const_decl_vec.push("G = 6.67430E-11"),
                        _ => ()
                    }
                }
                let const_decl_str = "MODULE math\nreal*8, parameter :: ".to_owned() + const_decl_vec.join(", ").as_str() + "\nEND MODULE math";
                const_decl_str
            }
            _ => captures[0].to_string(), // For other tokens, return the whole match
        }
    }
}

pub(crate) fn parser(doc: &str) -> String {
    let mut modified_doc = String::from(doc);

    let tokens = vec![
        Tokens::Comments,
        Tokens::Declaration,
        Tokens::MainFunction,
        Tokens::Expressions,
        Tokens::Print,
        //Tokens::If,
        Tokens::MathFuncs
    ];

    for token in &tokens {
        let (replacement, pattern) = token.pattern();
        modified_doc = pattern.replace_all(&modified_doc, |caps: &regex::Captures| {
            token.apply_replacement(&caps)
        }).into_owned();
    }

    /// Add modules to functions that require it
    // Main
    if doc.contains("use math") {
        modified_doc = modified_doc.replace("implicit none", "use math\nimplicit none");
    }

    modified_doc
}
