extern crate kafka;

fn main(){
    let broker = "localhost:8088".to_owned();
    let topic = "my-topic".to_owned();
    let group = "my-group".to_owned();
    
    if let Err(e) = consume_msgs(group, topic, vec![broker]) {
        println!("Failed consuming messages: {}", e);
    }
}