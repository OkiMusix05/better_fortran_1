#[derive(Clone)]
struct Line {
    num:u16,
    code:String
}
fn bracket_pair(txt:String) -> Result<Vec<Vec<Line>>, &'static str> {
    let mut stack:Vec<Vec<Line>> = Vec::new();
    let mut sorted:Vec<Vec<Line>> = Vec::new();
    let mut brackets_num:u16 = 0;
    let lines:Vec<&str> = txt.split("\n").collect();

    for (n, &line) in lines.iter().enumerate() {
        if line.contains("{") || line.contains("}") {
            brackets_num +=1
        }
        if line.contains("{") && !line.contains("elif") && !line.contains("else") {
            // Creates a new space left open
            stack.push(vec![Line{
                num: n as u16,
                code: String::from(line)
            }])
        }
        if line.contains("}") {
            if let Some(last_stack) = stack.last_mut() {
                // Pushes the code line into its block
                last_stack.push(Line {
                    num: n as u16,
                    code: String::from(line),
                });
                // Closes the block if the if-structure is correct and adds it to sorted
                if !line.contains("elif") && !line.contains("else") {
                    sorted.push(stack.pop().unwrap());
                }
            } else {
                // Handle the case when stack is empty
                return Err("Error: Found '}' without corresponding '{'");
            }
        }
    }
    if brackets_num%2 != 0 {
        return Err("Error: There is an unclosed '{'");
    }
    // Turns the array
    //sorted.reverse();
    // Returns the sorted array
    Ok(sorted)
}
pub(crate) fn loop_conditional_replacer(code_base:String) -> String {
    let mut stack:Vec<Vec<Line>> = bracket_pair(code_base.clone()).unwrap_or_else(|err| vec![vec![Line {
        num: 404,
        code: String::from(err)
    }]]);

    // Changes the syntax of the blocks to FORTRAN
    for mut block in &mut stack {
        let stack_header = {block.clone().first().unwrap().to_owned().code};
        for mut line in block.iter_mut() {
            // For if statements
            if stack_header.contains("if (") {
                if line.code.contains("elif (") {
                    line.code = line.code.replace("elif", "else if")
                        .replace("{", "then").replace("}", "");
                } else if line.code.contains("if") {
                    line.code = line.code.replace("{", "then");
                } else if line.code.contains("else") {
                    line.code = line.code.replace("}", "").replace("{", "");
                } else {
                    line.code = line.code.replace("}", "endif");
                }
            } // For while statements
            else if stack_header.contains("while (") {
                line.code = line.code.replace("while", "do while")
                    .replace("{", "").replace("}", "enddo");
            } // For for statements
            else if stack_header.contains("for (") {
                // This one requires a bit more regex
                line.code = line.code.replace("for", "do").replace("(", "")
                    .replace(")", "")
                    .replace("{", "").replace("}", "enddo");
            }
        }
    }

    // Merges the stack with the original code
    let mut code_split:Vec<&str> = code_base.split("\n").collect();
    let stack_array:Vec<Line> = {
        let mut array:Vec<Line> = vec![];
        for block in &stack {
            for line in block {
                array.push(line.clone());
            }
        }
        array
    }; // Single array with all changed elements
    let (mut num_stack, mut code_stack):(Vec<u16>, Vec<String>) = {
        let mut ns:Vec<u16> = vec![];
        let mut cs:Vec<String> = vec![];
        for line in stack_array {
            ns.push(line.num);
            cs.push(line.code.clone());
        }
        (ns, cs)
    };
    for (line_number, new_code) in num_stack.iter().zip(code_stack.iter()) {
        if let Some(line_number) = usize::try_from(*line_number).ok() {
            if line_number < code_split.len() {
                code_split[line_number] = new_code;
            } else {
                eprintln!("Line number {} out of bounds", line_number);
            }
        } else {
            eprintln!("Invalid line number: {}", line_number);
        }
    }
    // Returns the code translated conditional/loop code
    code_split.join("\n")

    //Prints the stack
    /*for block in stack {
        for line in block {
            println!("{} {}", line.num, line.code)
        }
        println!("-----");
    }*/
}