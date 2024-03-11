use std::ptr::replace;
use regex::Regex;
use std::sync::Mutex;
mod expressions_eval;
mod loop_conditional_parser;

enum Tokens {
    Comments,
    Declaration, /// For variables and constants
    MainFunction,
    IfElseLoops,
    Expressions,
    Print, // Doesn't support formats yet
    MathFuncs, // Includes pi, e, G
}

#[derive(Clone)]
pub struct Matrix {
    pub name:String,
    pub m:i8,
    pub n:i8
}

pub static mut MATRICES: Mutex<Vec<Matrix>> = Mutex::new(vec![]);
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
            //Tokens::Declaration => ("", Regex::new(r"(const|let) ([a-zA-Z][a-zA-Z0-9]+|[a-zA-Z]) *: *([a-zA-Z][a-zA-Z0-9]+) *= *((?s).*?);").unwrap()),
            Tokens::Declaration => ("", Regex::new(r"(const|let) ([a-zA-Z][a-zA-Z0-9, ]+|[a-zA-Z]) *: *([a-zA-Z][a-zA-Z0-9]+)<?([0-9]+|:)?,? *([0-9]+|:)?>? *=? *((?s).*?);").unwrap()),
            /* if(condition) {
                    code here
               } else {
                    more code
               }
             */
            Tokens::IfElseLoops => ("", Regex::new(r"(if|elif|else|for|while) *\((.*)\) *\n* *\{").unwrap()),
            // var1 = expression;
            Tokens::Expressions => ("", Regex::new(r"([a-zA-Z][a-zA-Z0-9]+|[a-zA-Z]) *= *(.*);").unwrap()),
            // print(thing);
            Tokens::Print => ("", Regex::new(r"print\((.*)\);").unwrap()),
            // use math::{pi, e, ...}
            Tokens::MathFuncs => ("", Regex::new(r"use math::\{(.*)}").unwrap()),
        }
    }
    fn apply_replacement(&self, captures: &regex::Captures) -> String {
        match self {
            Tokens::MainFunction => {
                let content = captures.get(1).map_or("", |m| m.as_str());
                format!("Program main\nimplicit none{}stop\nend program", content)
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
                let mut m = captures.get(4).map_or("", |m| m.as_str()).trim();
                let mut n = captures.get(5).map_or("", |m| m.as_str()).trim();
                let mut value = captures.get(6).map_or("", |m| m.as_str());
                let keyword:&str;
                let mut mtx:String = String::from("");
                let mut is_matrix:String = String::from("");
                let mut value_string:String = String::from("");
                let mut order ;
                if constorvar == "const" {
                    keyword = ", parameter";
                } else {
                    keyword = "";
                }
                if m != "" {
                    if n!= "" {
                        mtx = format!("<{},{}>", m, n);
                        is_matrix = format!(", dimension ({}, {})", m, n);
                    } else {
                        mtx = format!("<{}>", m);
                        is_matrix = format!(", dimension ({})", m);
                    }
                }
                if value != "" {
                    if m!= "" {
                        unsafe {
                            MATRICES.get_mut().unwrap().push(Matrix {
                                name: String::from(name),
                                m: m.parse().unwrap(),
                                n: {
                                    if n == "" { 1 } else {
                                        if n == ":" {
                                            0
                                        } else {
                                            n.parse().unwrap()
                                        }
                                    }
                                }
                            });
                        }
                        value_string = value.replace("[", "").replace("]", "")
                            .replace("\n", "").replace(" ", "").replace("|", ",");
                        if n != "" {
                            order = ", order=(/2,1/)";
                        } else {
                            order = "";
                        }
                        value_string = format!(" = reshape((/{}/), shape({}){})", value_string, name, order);
                    }
                } else {
                    if m != "" && n != "" {
                        value_string = format!("({},{})", m, n);
                    } else if n == "" && m != "" {
                        value_string = format!("({})", m)
                    }
                }
                if m == "" && n == ""{
                    if value != "" {
                        value_string = format!(" = {}", value)
                    } else {
                        value_string = "".to_string();
                    }
                }
                if m == ":" || n == ":" {
                    is_matrix = ", allocatable".to_string();
                }
                if data_type.trim() == "bool" {
                    value_string = value_string.replace(".true.", "true")
                        .replace(".false.", "false");
                }
                match data_type {
                    "int" => format!("integer{}{} :: {}{}", keyword, is_matrix, name, value_string),
                    "f4" => format!("real{}{} :: {}{}", keyword, is_matrix, name, value_string),
                    "f8" => format!("real*8{}{} :: {}{}", keyword, is_matrix, name, value_string),
                    "str" => format!("character(len = {}) :: {}{}", value_string.len(), name, value_string),
                    "bool" => format!("logical :: {}{}", name, value_string),
                    _ => format!("{} :: {} = {}", data_type, name, value), // Handle other data types
                }
            },
            Tokens::IfElseLoops => {
                let which = captures.get(1).map_or("", |m| m.as_str());
                let mut condition:String = String::from(captures.get(2).map_or("", |m| m.as_str()));
                // Replace the logical operators
                condition = condition.replace("===", ".eqv.").replace("!==", ".neqv.")
                    .replace("==", ".eq.").replace("!=", ".neq.")
                    .replace("&&", ".and.").replace("||", ".or.").replace("!", ".not.")
                    .replace("true", ".true.").replace("false", ".false.");
                // Format the loops well for the loop_conditional_parser applied at the end
                format!("{} ({}) {}", which, condition, "{")
            }
            Tokens::Expressions => {
                let recieve = captures.get(1).map_or("", |m| m.as_str());
                let mut exp = String::from(captures.get(2).map_or("", |m| m.as_str()));
                exp = exp.replace("true", ".true.").replace("false", ".false.");
                //let exp:String = expressions_eval::func_from_exp(exp, matrices);
                println!("Exp: {}", exp);
                format!("{} = {}", recieve, exp)
            }
            Tokens::Print => {
                let thing = captures.get(1).map_or("", |m| m.as_str());
                format!("print*, {}", thing)
            }
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
    let mut modified_doc:String = String::from(doc);

    let tokens = vec![
        Tokens::Comments,
        Tokens::Declaration,
        Tokens::MainFunction,
        Tokens::IfElseLoops,
        Tokens::Expressions,
        Tokens::Print,
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

    // Format If's/Loops to the FORTRAN syntax
    modified_doc = loop_conditional_parser::loop_conditional_replacer(modified_doc);
    // Replace special keywords
    modified_doc = modified_doc.replace("break;", "exit")// break loop function
        .replace("", ""); // Add more here

    modified_doc
}
