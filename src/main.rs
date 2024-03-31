extern crate clipboard;

use std::env;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use opencc_rs;
use opencc_rs::{find_max_utf8_length, format_thousand, Opencc};

fn main() {
    const RED: &str = "\x1B[1;31m";
    const GREEN: &str = "\x1B[1;32m";
    const YELLOW: &str = "\x1B[1;33m";
    const BLUE: &str = "\x1B[1;34m";
    const RESET: &str = "\x1B[0m";

    let mut config;
    let mut punct = false;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        config = args[1].clone();
        let config_vector = vec![
            "s2t", "t2s", "s2tw", "tw2s", "s2twp", "tw2sp", "tw2t", "t2tw", "s2hk", "hk2s", "hk2t",
            "t2hk", "t2jp", "jp2t",
        ];
        if !config_vector.contains(&config.as_str()) {
            config = "auto".to_string();
        }
        if args.len() > 2 {
            if args[2] == "punct" {
                punct = true;
            }
        }
    } else {
        config = "auto".to_string();
    }
    // Create a new clipboard context
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    // Attempt to read text from the clipboard
    match ctx.get_contents() {
        Ok(contents) => {
            // If successful, print the text to the console
            let opencc = Opencc::new();
            let input_code = opencc.zho_check(&contents);
            if config == "auto" {
                match input_code {
                    1 => config = "t2s".to_string(),
                    2 => config = "s2tw".to_string(),
                    _ => config = "none".to_string(),
                }
            }
            let output;
            if config != "none" {
                match punct {
                    true => output = opencc.convert_with_punctuation(&contents, &config),
                    false => output = opencc.convert(&contents, &config),
                }
            } else {
                output = contents.clone()
            }

            let display_input;
            let display_output;
            let display_input_code;
            let display_output_code;
            let etc;

            if input_code == 0 || config == "t2jp" || config == "jp2t" {
                display_input_code = "Non-zho 其它";
                display_output_code = "Non-zho 其它";
            } else if config.starts_with('s') {
                display_input_code = "Simplified Chinese 简体";
                display_output_code = "Traditional Chinese 繁体";
            } else if config.ends_with('s') || config.ends_with('p') {
                display_input_code = "Traditional Chinese 繁体";
                display_output_code = "Simplified Chinese 简体";
            } else {
                display_input_code = "Traditional Chinese 繁体";
                display_output_code = "Traditional Chinese 繁体";
            }

            if contents.len() > 600 {
                let max_utf8_length = find_max_utf8_length(&contents, 600);
                display_input = &contents[..max_utf8_length];
                etc = "...";
                display_output = &output[..max_utf8_length];
            } else {
                display_input = &contents;
                etc = "";
                display_output = &output;
            }

            println!("Opencc-Clip-rs Zho Converter version 1.0.0 Copyright (c) 2024 Bryan Lai");
            println!("Config: {}{}, {}{}", BLUE, config, punct, RESET);
            println!(
                "{}== Clipboard Input ({}) =={}\n{}{}{}{}",
                GREEN, display_input_code, RESET, YELLOW, display_input, etc, RESET
            );
            println!();
            println!(
                "{}== Converted Output ({}) =={}\n{}{}{}{}",
                GREEN, display_output_code, RESET, YELLOW, display_output, etc, RESET
            );

            match ctx.set_contents(output) {
                Ok(..) => {
                    let input_length = contents.chars().collect::<Vec<_>>().len();
                    println!(
                        "{}Converted output set to clipboard ({} chars).{}",
                        BLUE,
                        format_thousand(input_length),
                        RESET
                    );
                }
                Err(err) => {
                    eprintln!("{}Error set clipboard: {}{}", RED, err, RESET);
                }
            }
        }
        Err(err) => {
            // If an error occurs, print the error message
            // eprintln!("Error reading clipboard: {}", err);
            eprintln!("{}No text in clipboard: {}{}", RED, err, RESET);
        }
    }
}
