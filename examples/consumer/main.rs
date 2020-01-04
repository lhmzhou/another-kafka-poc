extern crate kafka;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

use failure::Error;

fn main(){
    let broker = "localhost:8088".to_owned();
    let topic = "my-topic".to_owned();
    let group = "my-group".to_owned();
    
    if let Err(e) = consume_msgs(group, topic, vec![broker]) {
        println!("Failed consuming messages: {}", e);
    }
}


async fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), Error> {
    // add code here to customize the consumer behavior 
}