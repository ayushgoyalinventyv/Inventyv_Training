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
        
        Student::print_student_info(self.get_both_student_info());
    }
    
    fn set_roll_no(&mut self, roll_no: u8){
        self.roll_no = roll_no;
        Student::print_student_info(self.get_both_student_info());
    }
    fn set_address(&mut self, address: Address){
        self.address = address;
        Student::print_student_info(self.get_both_student_info());
    }
    fn set_marks(&mut self, marks: u8){
        self.marks = marks;
        Student::print_student_info(self.get_both_student_info());
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

    fn print_student_info(student:String){
        println!("{student}");
    }

    fn get_both_student_info(&self) -> String{

        format!("
        Reference Variable
        Name : {} 
        Roll No : {} 
        Address : {} {} {} {} 
        Marks : {}

        Original Variable
        Name : {} 
        Roll No : {} 
        Address : {} {} {} {} 
        Marks : {}",
        self.name,self.roll_no,
        self.address.flat_no, self.address.building_name, self.address.area, self.address.city,
        self.marks,
        (*self).name,(*self).roll_no,
        (*self).address.flat_no, (*self).address.building_name, (*self).address.area, (*self).address.city,
        (*self).marks
    )}
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

    let s2 = &mut s1;

    println!("Name : {}",s2.get_name());
    println!("Roll No : {}",s2.get_roll_no());
    println!("Address : {:#?}",s2.get_address());
    println!("Marks : {}",s2.get_marks());

    s2.set_name(String::from("Ayush Goyal"));
    s2.set_roll_no(22);
    s2.set_address(
        Address{
            flat_no: String::from("6th Floor"), 
            building_name: String::from("Takshila Appartment"),
            area: String::from("Bhatar"),
            city: String::from("Surat")
        }
    );
    s2.set_marks(99);


}
