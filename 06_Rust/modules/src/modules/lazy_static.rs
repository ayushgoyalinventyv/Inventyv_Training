use lazy_static::lazy_static;
use std::sync::Mutex;

enum Request {
    Get {endpoint: String},
    Post {endpoint: String, payload_size: u32},
    Delete {id:u32}
}

lazy_static! {
    static ref GET_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref POST_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref DELETE_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref TOTAL_COUNTER: Mutex<u32> = Mutex::new(0);
}   

fn handle_request(req: Request) -> String{

    let mut total_counter = TOTAL_COUNTER.lock().unwrap();

    *total_counter+=1;

    match req{

        Request::Get { endpoint } => {
            let mut counter = GET_COUNTER.lock().unwrap();
            *counter+=1;
            format!("Get Request From {endpoint}")
        },

        Request::Post { endpoint , payload_size } => {
            let mut counter = POST_COUNTER.lock().unwrap();
            *counter+=1;
            format!("Post Request From {endpoint} with Payload Size of {payload_size}")
        },

        Request::Delete { id } => {
            let mut counter = DELETE_COUNTER.lock().unwrap();
            *counter+=1;
            format!("Request Deleted For Id {id}")
        }
    }
}

pub fn main() {
    handle_request(Request::Get { endpoint: String::from("Ayush Bhai") });

    handle_request(Request::Post { endpoint: String::from("Yashraj Bhai"), payload_size: 10 });

    handle_request(Request::Delete { id: 15 });

    
    let get_counter = GET_COUNTER.lock().unwrap();

    let post_counter = POST_COUNTER.lock().unwrap();

    let delete_counter = DELETE_COUNTER.lock().unwrap();
    
    let total_counter = TOTAL_COUNTER.lock().unwrap();
        

    println!("Get Count : {get_counter}");
    println!("Post Count : {post_counter}");
    println!("Delete Count : {delete_counter}");
    println!("Total Count : {total_counter}");
}
