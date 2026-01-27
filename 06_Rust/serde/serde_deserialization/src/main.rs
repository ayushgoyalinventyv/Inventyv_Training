use serde::Serialize;
use serde::Deserialize;
use serde_json;

#[derive(Debug,Serialize,Deserialize)]
struct Student{
    name : String,
    roll_no : u8,
    address : Address,
    marks : u8,
}

#[derive(Debug,Serialize,Deserialize)]
struct Address{
    flat_no: String,
    building_name: String,
    area: String,
    city: String
}

fn main() {
    let mut s1 = Student{
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

    s1 = serde_json::from_str(json_string.as_str()).unwrap();

    println!("First Method : \n\n{:#?}",s1);



    //  Second Way

    let new_student = r#"{"name": "Ayush Goyal", "roll_no": 25, "address": {"flat_no": "602", "building_name": "Takshila", "area": "bhatar", "city": "surat"}, "marks": 90}"#;
    
    let s2: Student = serde_json::from_str(new_student).unwrap();

    println!("Second Method : \n\n{:#?}",s2);

}
