use std::collections::HashMap;

#[derive(Debug)]
enum InventoryError{
    DuplicateId,
    InvalidId
}

trait DisplayItem {
    fn display(&self) -> String;
}

#[derive(Clone)]
struct Product{
    id: String,
    name: String,
    price: i32
}

impl DisplayItem for Product {
    fn display(&self) -> String {
        format!("Product Name: {} and Price : {}",self.name,self.price)
    }
}

struct Inventory<T>
where
T: DisplayItem + Clone,
{
    item: HashMap<String,T>
}



impl <T> Inventory<T>
where 
T: DisplayItem + Clone 
{
      fn add_item(&mut self, id: String, item:T) -> Result<(),InventoryError>{
            
            if id.trim().is_empty(){
                return Err(InventoryError::InvalidId);
            }

            if self.item.contains_key(&id){
                return Err(InventoryError::DuplicateId);
            }

            self.item.insert(id,item);
            Ok(())
      }  

      fn display_all(&self) -> String{
        if self.item.is_empty() {
            return "Inventory is empty.".to_string();
        }

        let mut result = String::new();
        for (id,item) in self.item.iter(){
            result.push_str(&format!("Id : {}\n {}\n\n",id,item.display()));
        } 
        result
      }  
}






fn main() {

    let mut products = HashMap::new();

    let product1 = Product{id:"1".to_string(),name:"Book".to_string(),price:100};
    let product2 = Product{id:"2".to_string(),name:"Pen".to_string(),price:200};

    products.insert(product1.id.clone(), product1.clone());
    products.insert(product2.id.clone(), product2.clone());
    
    let mut inv = Inventory{item:products};

    let product3 = Product{id:"3".to_string(),name:"Pencil".to_string(),price:20};

    let mut result = inv.add_item(product3.id.clone(), product3.clone());

    match result{
        Ok(_result) => println!("New Value Inserted Successfully..."),
        
        Err(error) => println!("Insertion Failed , {:?}",error)
    }

    result = inv.add_item(product3.id.clone(), product3.clone());

    match result{
        Ok(_result) => println!("New Value Inserted Successfully..."),
        
        Err(error) => println!("Insertion Failed , {:?}",error)
    }

    println!("{}",inv.display_all());



    
}
