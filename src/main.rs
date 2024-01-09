use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
    dump_path: std::path::PathBuf,
}

fn get_input(message: String) -> String {
    let mut input = String::new();

    println!("{}", message);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");

    return input;
}

fn main() {
    let args = Cli::parse();

    let file = std::fs::read_to_string(args.dump_path).unwrap();
    let entries: serde_json::Value = serde_json::from_str(&file).unwrap();

    let _enum_name = get_input("Which will be the Enum name?".to_string());

    match entries {
        serde_json::Value::Array(map) => {
            let mut primary_key = String::new();
            let mut fn_name_map: HashMap<String, String> = HashMap::new();

            let first_item = map.first().unwrap();

            match first_item {
                serde_json::Value::Object(element) => {
                    let keys: Vec<String> = element.keys().cloned().collect();

                    if primary_key.is_empty() {
                        primary_key = select_options(&keys);
                    }

                    if fn_name_map.keys().len() != (keys.len() - 1) {
                        for key in keys {
                            if key == primary_key {
                                continue;
                            }

                            let fn_name =
                                get_input(format!("Choose your function name for {}:", key));

                            if fn_name_map.contains_key(&key) {
                                continue;
                            }

                            fn_name_map.insert(key, fn_name);
                        }
                    }
                }
                _ => panic!("Array items must be an object"),
            }

            for item in map {
                match item {
                    serde_json::Value::Object(element) => {
                        let _pk_value = element.get_key_value(&primary_key);
                        todo!("How to identify key name?");
                    }
                    _ => panic!("Array items must be an object"),
                }
            }
        }
        _ => panic!("Json must be an array"),
    }
}

fn select_options(options: &Vec<String>) -> String {
    let mut i = 1;

    for option in options {
        println!("{} - {}", i, option);
        i += 1;
    }

    let input = get_input("Select your primary key:".to_string());

    let index: usize = input.trim().parse().unwrap();

    if index > options.len() + 1 {
        panic!("Invalid value");
    }

    return options[index - 1].to_string();
}
