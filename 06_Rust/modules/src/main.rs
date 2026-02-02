mod modules;

fn main() {
    println!("============Lazy Static ============");
    modules::lazy_static::main();
    
    println!("============Loops ============");
    modules::loops::main();
    
    println!("============Mutability ============");
    modules::mutability::main();
    
    println!("============RwLock ============");
    modules::rwlock::main();
    
    println!("============Serde Serialization ============");
    modules::serde_serialization::main();
    
    println!("============Serde Deserialization ============");
    modules::serde_deserialization::main();
    
    println!("============Structs ============");
    modules::structs::main();
    
    println!("============Vectors ============");
    modules::vectors::main();
}
