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
    let mut consumer = Consumer::from_hosts(brokers)
        .with_topic(topic)
        .with_group(group)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create().await?;
    
    // add code here to customize the consumer behavior 


}