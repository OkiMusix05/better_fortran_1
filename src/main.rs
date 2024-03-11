mod parser;

use std::io::{self, Write};
use std::path::{Path};
use std::fs::{self, File};
use std::process::{self, Command};
use crate::parser::VARIABLES;

fn main() {
    println!("BTF Path:");
    let mut input_path = String::new();
    let mut btfcode: String = String::from("");

    /// Reads the path specified by the user
    match io::stdin().read_line(&mut input_path) {
        Ok(_) => {
            let input_path = input_path.trim();
            let input_path = Path::new(input_path);
            if input_path.is_file() {
                if let Some(ext) = input_path.extension() {
                    if ext == "btf" {
                        match fs::read_to_string(input_path) {
                            Ok(contents) => {
                                btfcode = contents;
                            }
                            Err(error) => {
                                eprintln!("Error reading file: {}", error);
                                process::exit(1);
                            }
                        }
                    } else {
                        println!("Specified file is not a .btf file.");
                        process::exit(1);
                    }
                } else {
                    println!("Specified file has no extension.");
                    process::exit(1);
                }
            } else {
                println!("Invalid file path.");
                process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
            process::exit(1);
        }
    }
    /// FORMAT ELIF'S AND ELSE'S BEFORE SENDING THEM
    let lines:Vec<&str> = btfcode.split("\n").collect();
    let mut new_lines:Vec<String> = vec![];
    for l in 0..lines.len() {
        if !lines[l].contains("elif") && !lines[l].contains("else") {
            new_lines.push(lines[l].to_string());
            continue
        }
        if (lines[l].contains("elif") || lines[l].contains("else")) && !lines[l].contains("}") {
            let s = format!("{} {}", lines[l-1].trim_end(), lines[l].trim());
            new_lines.pop();
            new_lines.push(s);
            continue
        } else {
            new_lines.push(lines[l].to_string());
        }
    }
    btfcode = new_lines.join("\n");
    //println!("{}", btfcode);

    /// Error Handling PRE-translation
    if !btfcode.contains("fn main") {
        panic!("There is no main function");
    }

    /// Translation
    let ft_code = parser::parser(btfcode.as_str());
    println!("{}", ft_code);
    /// Error handling post-translation
    /*if ft_code.contains("let") {
        panic!("There are incorrect variable declarations");
    }
    if ft_code.contains("const") {
        panic!("There are incorrect constant declarations");
    }*/
    unsafe {
        /*let matrices = parser::MATRICES.lock().unwrap().clone();
        for matrix in matrices {
            println!("{}: {},{}", matrix.name, matrix.m, matrix.n);
        }*/
        //println!("{:?}", parser::VARIABLES);
    }

    /// Writing to output and compiling
    let output_file = "output.f90";
    //let mut file = File::create(output_file).expect("Failed to create file");
    //file.write_all(ft_code.as_bytes()).expect("Failed to write to file");
    //let run_command = "gfortran {}.f90 -o {} && '{}'";
}