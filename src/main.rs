extern crate yaml_rust;

use yaml_rust::{YamlLoader, YamlEmitter};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    let path    = Path::new("config/example.yml");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_)    => print!("{} contains:\n{}", display, s),
    }

    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc  = &docs[0]; // Multi document support, doc is a yaml::Yaml

    // println!("Start debug");
    // println!("{:?}", doc);
    // println!("End debug");

    assert_eq!(doc["game_one"]["name"].as_str().unwrap(), "FedEx Field Steelers @ Redskins, Week 1");
    assert_eq!(doc["game_two"]["latitude"].as_f64().unwrap(), 37.403);
    assert!(doc["INVALID_KEY"][100].is_badvalue());

    // In case you want to write a YAML
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }

    println!("{}", out_str);
}
