use app_dirs::*;
use std::env::{args, current_dir};
use std::fs::{read, File};
use std::io::Write;
use std::path::Path;

const SECRET_KEY: &[u8] = b"Putting in a super basic encryption pass so our saves are a little harder to edit than just opening a text or hex editor.  Need a secret key or some such... so here's some nonsense.\n";
const VERSION: &[u8] = b"version: 0\n";

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Insufficient amount of arguments. Use `help` for more information");
        return;
    };

    if args[1] == "help" {
        println!("gungeon-save: decode <optional on windows: path> | encode <optional on windows: path> | help");
        return;
    };

    if args[1] == "decode" {
        if args.len() == 2 && !cfg!(windows) {
            println!("Please provide the full path.");
            return;
        }

        let dir;

        if args.len() == 2 {
            let mut directory = match get_data_root(AppDataType::UserData) {
                Ok(mut d) => {
                    d.pop();
                    d.push("LocalLow");
                    d.push("Dodge Roll");
                    d.push("Enter The Gungeon");
                    d
                }
                Err(_) => {
                    println!("An error occured trying to get the path automatically. Please provide it manually.");
                    return;
                }
            };

            directory.push("SlotA.save");
            dir = directory
        } else {
            dir = Path::new(&args[2]).to_path_buf()
        }

        if !dir.exists() {
            println!("File not found, please provide the path manually.");
            return;
        }

        let mut version = read(dir).expect("File is corrupted.");
        let content = version.split_off(11);

        if version != VERSION {
            println!("Version does not match.");
            return;
        };

        let mut decoded: Vec<u8> = Vec::with_capacity(content.len());
        let mut index = 0;
        for x in content {
            decoded.push(SECRET_KEY[index] ^ x);
            index += 1;
            if index >= SECRET_KEY.len() - 1 {
                index = 0;
            };
        }

        let mut current_dir = current_dir().expect("Failed to get current directory.");
        current_dir.push("SlotA.txt");
        let mut file = File::create(&current_dir).expect("Failed to create decoded file.");

        file.write_all(&decoded).expect("Failed to write to file");
        println!("Decoded file at {:?}", current_dir);
    };
    if args[1] == "encode" {
        let mut current_dir = current_dir().expect("Failed to get current directory.");

        let mut dir;
        if args.len() == 2 {
            dir = current_dir.clone();
            dir.push("SlotA.txt")
        } else {
            dir = Path::new(&args[2]).to_path_buf()
        }

        if !dir.exists() {
            println!("File not found, please provide an existing file in the current directory.");
            return;
        }

        let content = read(&dir).expect("File is corrupted.");

        let mut encoded: Vec<u8> = Vec::with_capacity(VERSION.len() + content.len());
        let mut index = 0;
        for x in content {
            encoded.push(x ^ SECRET_KEY[index]);
            index += 1;
            if index >= SECRET_KEY.len() - 1 {
                index = 0;
            };
        }

        current_dir.push("SlotA.save");

        let mut file = File::create(&current_dir).expect("Failed to create decoded file.");
        file.write_all(VERSION).expect("Failed to write to file.");
        file.write_all(&encoded).expect("Failed to write to file.");

        println!("Encoded file at {:?}", current_dir);
    }
}
