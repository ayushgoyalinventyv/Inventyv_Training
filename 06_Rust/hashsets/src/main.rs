use std::collections::HashSet;

#[derive(Debug,Clone,Hash, PartialEq, Eq)]
struct Student{
    name : String,
    roll_no : u8,
    address : Address,
    marks : u8,
}

#[derive(Debug,Clone,Hash, PartialEq, Eq)]
struct Address{
    flat_no: String,
    building_name: String,
    area: String,
    city: String
}

impl Student {
    fn get_student_info(&self) -> String{
        format!("\nName : {} \nRoll No : {} \nAddress : {} {} {} {} \nMarks : {}",self.name,self.roll_no,self.address.flat_no, self.address.building_name, self.address.area, self.address.city,self.marks)
    }
}

fn main() {
    
    let mut student_details= HashSet::new();

    let student1= Student{
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

    let student2= Student{
        name : String::from("Perry"),
        roll_no : 2,
        address : Address{
            flat_no: String::from("610"), 
            building_name: String::from("Studio Complex"),
            area: String::from("Gota"),
            city: String::from("Ahmedabad")
        },
        marks : 90
    };

    let student1_copy = student1.clone();


    student_details.insert(student1);

    student_details.insert(student2);

    for student in student_details.iter(){
        println!("Student Details : \n{}\n",student.get_student_info());
    }

    // Clone Function

    let student_backup = student_details.clone();

    println!("Cloned Student Data :");
    for student in student_backup.iter(){
        println!("Student Details : {}\n",student.get_student_info());
    }

    // Unwrap Function

    // using unwrap method in get method , bcz it returns answer in some(value)
    // _ is used to tell the compiler that this variable is intentionally put and will not be using it in the code
    
    println!("Student Data Using Get and Unwrap Method:");
    for student in student_details.iter(){
        println!("{:?}\n",student_details.get(student).unwrap());
    }

    //println!("{:?}",student_details.try_reserve(100000000000));
    //print!("Hello")

    // try_reserve Method
    match student_details.try_reserve(1000000000000){
        Ok(_) => println!("Memory Allocation Successful..."),
        Err(e) => println!("Memory Allocation Failed with error : {e}")
    }

    // take Method -> used for changing values and replace the original one with any other value [ Default / None [Some] ]

    // let mut student_name = Some(String::from("Ayush"));
    // let name = student_name.take(); // take() requires &mut option ref
    // println!("{}",name.unwrap());
    // println!("{:?}",student_name);

    let taken_student_value = student_details.take(&student1_copy);
    println!("Taken Student Detail : {:?}",taken_student_value.unwrap());


    // retain Method -> it checks the condition, if return true then it keeps the (key,value) pair, otherwise remove it
    student_details.retain(|value| value.marks>35);

    println!("Student Data After calling Retain Method:");
    for student in student_details.iter(){
        println!("Student Details : {}\n",student.get_student_info());
    }
    


    // extend Method -> it is used to all multiple elements/ whole hashmap in a hashmap/collection

    let student3= Student{
        name : String::from("Sahil"),
        roll_no : 3,
        address : Address{
            flat_no: String::from("402"), 
            building_name: String::from("Complex Dep"),
            area: String::from("Vesu"),
            city: String::from("Surat")
        },
        marks : 94
    };

    let new_student = HashSet::from(
        [(student3)]
    );

    student_details.extend(new_student);


    println!("Student Data After Extend:");
    for student in student_details.iter(){
        println!("Student Details : {}\n",student.get_student_info());
    }



}
