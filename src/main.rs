extern crate regex;

use std::process::Command;
use std::vec::Vec;
use regex::Regex;

fn compare(arg1: u8, arg2: &u8) -> bool {
    arg1 == *arg2
}

fn main() {

    struct HSLWrapper {
        h: f32,
        s: f32,
        l: f32
    }

    let args = ["input.jpg", "-colors", "100", "-colorspace", "HSL", "-format", "%c", "-define", "histogram:unique-colors=true", "histogram:info:-"];
    let process = Command::new("convert")
        .args(&args)
        .output().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    let re = Regex::new(r"hsl\((.*)%,(.*)%,(.*)%\)").unwrap();

//hsl\((.*)%,(.*)%,(.*)%\)

    let mut commands = Vec::<String>::new();
    let mut hsl_values = Vec::<HSLWrapper>::new();

    let f: u8 = '\n' as u8;
    for group in process.stdout.split(|chr| compare(f, chr)) {
        let strx = String::from_utf8_lossy(group);


        for cap in re.captures_iter(&strx) {

            let current_hsl = HSLWrapper {
                    h: cap.at(1).unwrap().parse::<f32>().unwrap(),
                    s: cap.at(2).unwrap().parse::<f32>().unwrap(),
                    l: cap.at(3).unwrap().parse::<f32>().unwrap(),
            };

            hsl_values.push(current_hsl);
        }
    }

    for hsl in hsl_values {
        //let hsl_command = format!("xc:hsl({}%, {}%, {}%)", current_hsl.h, current_hsl.s, current_hsl.l);

        //commands.push(String::from("-size"));
        //commands.push(String::from("10x100"));
        //commands.push(hsl_command);

        println!("{:?}", hsl.l);
    }

    commands.push(String::from("+append"));
    commands.push(String::from("image_out.jpg"));

    let process1 = Command::new("convert")
        .args(&commands)
        .output().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
}
