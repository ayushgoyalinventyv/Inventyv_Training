#[derive(Debug)]
struct Student{
    name : String,
    roll_no : u8,
    address : Address,
    marks : u8,
}

#[derive(Debug)]
struct Address{
    flat_no: String,
    building_name: String,
    area: String,
    city: String
}

impl Student {
    fn set_name(&mut self, name: String){
        self.name = name;
    }
    fn set_roll_no(&mut self, roll_no: u8){
        self.roll_no = roll_no;
    }
    fn set_address(&mut self, address: Address){
        self.address = address;
    }
    fn set_marks(&mut self, marks: u8){
        self.marks = marks;
    }

    fn get_name(&self) -> String{
        self.name.clone()
    }
    fn get_roll_no(&self) -> u8{
        self.roll_no
    }
    fn get_address(&self) -> &Address{
        &self.address
    }
    fn get_marks(&self) -> u8{
        self.marks
    }

    fn get_student_info(&self) -> String{
        format!("\nName : {} \nRoll No : {} \nAddress : {} {} {} {} \nMarks : {}",self.name,self.roll_no,self.address.flat_no, self.address.building_name, self.address.area, self.address.city,self.marks)
    }
}

fn create_student(name:String, roll_no:u8, address:Address, marks:u8) -> Student{
    Student { name, roll_no, address, marks }
}

pub fn main() {
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

    println!("Name : {}",s1.get_name());
    println!("Roll No : {}",s1.get_roll_no());
    println!("Address : {:#?}",s1.get_address());
    println!("Marks : {}",s1.get_marks());

    s1.set_name(String::from("Ayush Goyal"));
    s1.set_roll_no(22);
    s1.set_address(
        Address{
            flat_no: String::from("6th Floor"), 
            building_name: String::from("Takshila Appartment"),
            area: String::from("Bhatar"),
            city: String::from("Surat")
        }
    );
    s1.set_marks(99);

    println!("{}",s1.get_student_info());

    let s2=create_student(
        String::from("Yashraj"), 
        2, 
        Address{
            flat_no: String::from("6 Floor"), 
            building_name: String::from("Studio Complex"),
            area: String::from("Gota"),
            city: String::from("Ahmedabad")
        }, 
        100
    );

    println!("{}",s2.get_student_info());
    println!("{:#?}",s2);

}
