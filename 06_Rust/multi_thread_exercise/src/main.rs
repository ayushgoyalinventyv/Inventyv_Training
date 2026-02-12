use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};
use chrono::{NaiveDateTime, Local, TimeZone};
use lazy_static::lazy_static;

#[derive(Debug)]
struct MultiThread{
    id:i32,
    recordAddedTime:String,
    threadId:String,
}

lazy_static! {
    static ref COUNTER: RwLock<i32> = RwLock::new(0);
}


fn main() {
    let data:Arc<RwLock<Vec<MultiThread>>> = Arc::new(RwLock::new(vec![]));

    // Thread 1 — Record Creator
    let data_clone = Arc::clone(&data);
    thread::spawn(move || {

        loop{
            
            let new_id = {
                let mut count = COUNTER.write().unwrap();
                *count += 1;
                *count
            };
            
            data_clone.write().unwrap().push(
                MultiThread { 
                    id: new_id,
                    recordAddedTime: Local::now()
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string(), 
                    threadId: format!("{:?}", thread::current().id()),
                }
            );

            thread::sleep(Duration::from_secs(10));
        }
        
    });

    // Thread 2 — State Printer
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        loop {
            println!("\n\n{:#?}", data_clone.read().unwrap());  
            thread::sleep(Duration::from_secs(1));  
        }
    });

    // Thread 3 — Even Record Cleaner
    let data_clone = Arc::clone(&data);
    thread::spawn(move || {
        loop {
            let now = Local::now();
            data_clone.write().unwrap().retain(|record| {

                // convert string → NaiveDateTime
                let stored_time = NaiveDateTime::parse_from_str(
                    &record.recordAddedTime,
                    "%Y-%m-%d %H:%M:%S"
                ).unwrap();

                // convert to Local time
                let stored_time = Local.from_local_datetime(&stored_time).unwrap();

                // keep record only if <= 20 seconds old
                now.signed_duration_since(stored_time) < chrono::Duration::seconds(20) && &record.id % 2 != 0
            });  
            thread::sleep(Duration::from_secs(1));  
        }
    });

    // Thread 4 — Odd Record Cleaner
    let data_clone = Arc::clone(&data);
    thread::spawn(move || {
        loop {
            let now = Local::now();
            data_clone.write().unwrap().retain(|record| {

                // convert string → NaiveDateTime
                let stored_time = NaiveDateTime::parse_from_str(
                    &record.recordAddedTime,
                    "%Y-%m-%d %H:%M:%S"
                ).unwrap();

                // convert to Local time
                let stored_time = Local.from_local_datetime(&stored_time).unwrap();

                // keep record only if <= 20 seconds old
                now.signed_duration_since(stored_time) < chrono::Duration::seconds(20) && &record.id % 2 == 0
            });  
            thread::sleep(Duration::from_secs(1));  
        }
    });

    // Thread 5 — Even Counter
    let data_clone = Arc::clone(&data);
    thread::spawn(move || {
        loop {
            let reader = data_clone.read().unwrap();
            let even_count = reader.iter()
                .filter(|record| record.id % 2 == 0)
                .count();
            println!("Even count: {}", even_count);
            thread::sleep(Duration::from_secs(1));  
        }
    });

    // Thread 6 — Odd Counter
    let data_clone = Arc::clone(&data);
    thread::spawn(move || {
        loop {
            let reader = data_clone.read().unwrap();
            let odd_count = reader.iter()
                .filter(|record| record.id % 2 != 0)
                .count();
            println!("Odd count: {}", odd_count);
            thread::sleep(Duration::from_secs(1));  
        }
    });
    
    handle.join().unwrap();

    
}
