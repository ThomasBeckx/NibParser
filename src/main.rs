use std::{env, fs};
mod cast;
mod data;
mod display;
mod raw_model;

use raw_model::nib::NibFile;

use data::Context;

use crate::display::JSON;

fn main() {
    println!("----------------------------------------");
    println!("- Nib parser");
    println!("- Using specifications defined in \"https://github.com/matsmattsson/nibsqueeze/blob/master/NibArchive.md\"");
    println!("----------------------------------------");
    println!();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Input file required!")
    }

    let input = fs::read(args.get(1).unwrap()).unwrap();

    let nib = NibFile::from_buffer(input).unwrap();

    let raw_objects = nib.get_objects().unwrap();
    let raw_keys = nib.get_keys().unwrap();
    let raw_classes = nib.get_classes().unwrap();
    let raw_values = nib.get_values().unwrap();

    println!("Successfully parsed all binary data");

    let context = Context {
        objects: raw_objects,
        keys: raw_keys,
        values: raw_values,
        classes: raw_classes,
    };

    let o = context.parse();

    println!("{}", o.to_json());
}
