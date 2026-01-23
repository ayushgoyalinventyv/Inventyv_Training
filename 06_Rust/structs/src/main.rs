struct Student{
    name : String,
    roll_no : u8,
    address : String,
    marks : u8,
}

impl Student {
    fn set_name(&mut self, name: String){
        self.name = name;
    }
    fn set_roll_no(&mut self, roll_no: u8){
        self.roll_no = roll_no;
    }
    fn set_address(&mut self, address: String){
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
    fn get_address(&self) -> String{
        self.address.clone()
    }
    fn get_marks(&self) -> u8{
        self.marks
    }

    fn get_student_info(&self) -> String{
        format!("\nName : {} \nRoll No : {} \nAddress : {} \nMarks : {}",self.name,self.roll_no,self.address,self.marks)
    }
}

fn create_student(name:String, roll_no:u8, address:String, marks:u8) -> Student{
    Student { name, roll_no, address, marks }
}

fn main() {
    let mut s1 = Student{
        name : String::from("Ayush"),
        roll_no : 1,
        address : String::from("Surat"),
        marks : 90
    };

    println!("Name : {}",s1.get_name());
    println!("Roll No : {}",s1.get_roll_no());
    println!("Address : {}",s1.get_address());
    println!("Marks : {}",s1.get_marks());

    s1.set_name(String::from("Ayush Goyal"));
    s1.set_roll_no(22);
    s1.set_address(String::from("Surat, Gujarat"));
    s1.set_marks(99);

    println!("{}",s1.get_student_info());

    let s2=create_student(
        String::from("Yashraj"), 
        2, 
        String::from("Ahmedabad"), 
        100
    );

    println!("{}",s2.get_student_info());

}
