use std::io::Read;

pub fn evaluate(code: &Vec<String>) -> () {
    let operations: String = filter(&code);
    let mut memory: Vec<char> = vec!['0'; 3000];
    let mut data_pointer: usize = 512;
    let mut i: usize = 0;
    let mut counter: i32 = 0;
    while i < operations.chars().count() {
        let current_char: char = operations.chars().nth(i).unwrap();
        match current_char {
            '>' => data_pointer += 1,
            '<' => data_pointer -= 1,
            '+' => {
                memory[data_pointer] = char::from_u32((memory[data_pointer] as u32) + 1).unwrap();
            }
            '-' => {
                memory[data_pointer] = char::from_u32((memory[data_pointer] as u32) - 1).unwrap()
            }
            '.' => println!("{}", memory[data_pointer]),
            ',' => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin()
                    .read_exact(&mut input)
                    .expect("failed to read stdin");
                memory[data_pointer] = input[0] as char;
            }
            '[' => {
                if memory[data_pointer] == '0' {
                    counter += 1;
                    while operations.chars().nth(i).unwrap() != ']' || counter != 0 {
                        i += 1;
                        if operations.chars().nth(i).unwrap() == '[' {
                            counter += 1;
                        } else if operations.chars().nth(i).unwrap() == ']' {
                            counter -= 1;
                        }
                    }
                }
            }
            ']' => {
                if memory[data_pointer] != '0' {
                    counter += 1;
                    while operations.chars().nth(i).unwrap() != '[' || counter != 0 {
                        i += 1;
                        if operations.chars().nth(i).unwrap() == ']' {
                            counter += 1;
                        } else if operations.chars().nth(i).unwrap() == '[' {
                            counter -= 1;
                        }
                    }
                }
            }
            _ => (),
        }
        i += 1;
    }
}

fn filter(code: &Vec<String>) -> String {
    let mut operations: String = String::new();
    let valid_characters = ['.', ',', '[', ']', '<', '>', '+', '-'];
    // Concatenate all Strings in code then remove invalid characters
    for s in code {
        operations += s
    }
    operations.retain(|ref c| valid_characters.contains(c));
    return operations;
}
