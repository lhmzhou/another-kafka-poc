extern crate kafka;
extern crate yaml_rust;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use yaml_rust::yaml;

mod pipe;

fn main() {
    // Open config file, turn it into a string for YAML parser
    let path = Path::new("config/sample.yml");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    // Parse YAML from file string
    let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
    let doc =  &docs[0];

    // Get consumer broker(s) and topic
    let consumer_broker = doc["consumer"]["broker"].as_str().unwrap().to_owned();
    let consumer_topic = doc["consumer"]["topic"].as_str().unwrap().to_owned();

    // Get producer broker(s) and topic
    let producer_broker = doc["producer"]["broker"].as_str().unwrap().to_owned();
    let producer_topic = doc["producer"]["topic"].as_str().unwrap();

    // Open the pipe from consumer to producer
    if let Err(e) = pipe::pipe_messages(consumer_topic, vec![consumer_broker], producer_topic, vec![producer_broker]) {
        println!("Failed piping messages: {}", e);
    }
}
