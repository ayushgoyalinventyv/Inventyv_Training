use serde::Serialize;
use serde_json;

#[derive(Debug,Serialize)]
struct Student{
    name : String,
    roll_no : u8,
    address : Address,
    marks : u8,
}

#[derive(Debug,Serialize)]
struct Address{
    flat_no: String,
    building_name: String,
    area: String,
    city: String
}

fn main() {
    let s1 = Student{
        name : String::from("Ayush"),
        roll_no : 1,
        address : Address{
            flat_no: String::from("602"), 
            building_name: String::from("Takshila Appartment"),
            area: String::from("Bhatar"),
            city: String::from("Surat")
        },
        marks : 90
    };

    let json_string = serde_json::to_string(&s1).unwrap();

    println!("Structure Variable : \n\n{:#?}",s1);
    println!("Json Variable : \n\n{}",json_string);

}
