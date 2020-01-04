extern crate kafka;

use std::str;
use std::time::Duration;

use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

pub fn pipe_messages<'a>(consumer_topic: String, consumer_brokers: Vec<String>, producer_topic: &'a str, producer_brokers: Vec<String>) -> Result<(), KafkaError> {
    let mut producer = try!(
        Producer::from_hosts(producer_brokers)
             .with_ack_timeout(Duration::from_secs(1))
             .with_required_acks(RequiredAcks::One)
             .create()
    );

    let mut consumer = try!(
        Consumer::from_hosts(consumer_brokers)
            .with_topic(consumer_topic)
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
    );

    loop {
        let mss = try!(consumer.poll());

        for ms in mss.iter() {
            for m in ms.messages() {
                let msg = str::from_utf8(&m.value).unwrap();

                // do stuff to received data here
                println!("{}", msg);

                try!(producer.send(&Record {
                    topic: producer_topic,
                    partition: -1,
                    key: (),
                    value: msg,
                }));
            }
            let _ = consumer.consume_messageset(ms);
        }
        try!(consumer.commit_consumed());
    }
}
