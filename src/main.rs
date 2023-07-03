use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde::{Serialize, Deserialize};
use std::env;
use serde_json::{Value};


#[derive(Debug,Serialize, Deserialize,Clone)]
struct FunctionData {
    return_type: String,
    function_name: String,
    arguments: Vec<Argument>,
    function_body: String,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
struct Argument {
    arg_type: String,
    arg_name: String,
}

fn extract_arguments(args_str: &str) -> Result<Vec<Argument>,String> {

    if args_str.is_empty() {
        return Ok(Vec::new());
    }

    let mut arguments = Vec::new();
    for arg_str in args_str.split(',') {
        let parts: Vec<&str> = arg_str.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid argument syntax".into());
        }
        arguments.push(Argument {
            arg_type: parts[0].to_owned(),
            arg_name: parts[1].to_owned(),
        });
    }

    Ok(arguments)
}

fn parse_cpp_function_syntax(input: &str) -> Result<Vec<FunctionData>,String> {
    
    let mut res_data :Vec<FunctionData> = Vec::new();
    let mut data = FunctionData {
        return_type: String::new(),
        function_name: String::new(),
        arguments: Vec::new(),
        function_body: String::new(),
    };

    let mut in_return_type = true;
    let mut in_function_name = false;
    let mut in_argument_list = false;
    let mut in_function_body = false;


    let mut buffer = String::new();

    for c in input.chars() {
        match c {
            ' ' => {
                if in_return_type && !buffer.is_empty() {
                    data.return_type = buffer.trim().to_owned();
                    in_return_type = false;
                    in_function_name = true;
                    buffer.clear();
                }
                else{
                    buffer.push(c);
                }
            }
            '(' => {
                in_function_name = false;
                in_argument_list=true;
                data.function_name = buffer.trim().to_owned();
                buffer.clear();
            }
            ')' => {
                let val = extract_arguments(&buffer);
                match val{
                    Ok(arguments)=> data.arguments=arguments,
                    Err(e)=> return Err(e)
                }
                buffer.clear();
                in_argument_list = false;
            }
            '{' => {
                in_function_body = true;
            }
            '}' => {
                in_function_body = false;
                data.function_body=buffer.trim().to_owned();
                buffer.clear();
                in_return_type=true;
                res_data.push(data.clone());
                data.return_type=String::new();
                data.function_name = String::new();
                data.arguments= Vec::new();
                data.function_body= String::new();
            }
            '\n'=>(),
            _ => {
                if in_function_name || in_return_type || in_argument_list {
                    buffer.push(c);
                
                } else if in_function_body {
                    data.function_body.push(c);
                }
            }
        }
    }

    Ok(res_data)
}

fn main() ->std::io::Result<()> {
    let output_file_path = "output.json";
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args[1];
    let input_file = File::open(input_file_path)?;
    let reader = BufReader::new(input_file);
    let mut input = String::new();
    for line in reader.lines() {
        input.push_str(&line?);
    }

    let parsed_data = parse_cpp_function_syntax(&input);
    match parsed_data{
        std::result::Result::Ok(res_data)=> {
            let json_str = 
            format!(
                    r#"{{ "data": [{}] }}"#,
            res_data.iter().map(
                | data |
                format!(
                    r#"{{
                        "object": "function",
                        "name": "{}",
                        "largs": [{}]
                    }}"#,
                    data.function_name,
                    (data.arguments
                        .iter()
                        .map(|arg| format!(
                            r#"{{ "type": "{}", "arg": "{}" }}"#,
                            arg.arg_type, arg.arg_name
                        ))
                        .collect::<Vec<String>>()
                        .join(", "))
                )).collect::<Vec<String>>()
                .join(", "));
                println!("{:?}",json_str);
            let output_json:Value = serde_json::from_str(&json_str).unwrap();
            let mut output_file = File::create(output_file_path)?;
            write!(output_file, "{}", output_json)?;     
            Ok(())
        },
        std::result::Result::Err(e)=>{println!("{:?}",e);
        return Ok(());
    }
    }
}