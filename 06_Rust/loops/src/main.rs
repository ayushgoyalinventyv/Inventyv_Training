use std::io;

fn main() {
    let pass = String::from("pass@1234");
    let mut login_attempt = 0;

    'main: loop {

        let mut password = String::new();
        println!("Plz Enter Password : ");
        
        io::stdin()
        .read_line(&mut password)
        .expect("Failed To Fetch...");

        password = String::from(password.trim());

        login_attempt+=1;

        if password == pass { 

            println!("\nCorrect Password, Welcome...!");

            let mut upper =0;
            let mut lower =0;
            let mut digits =0;
            let mut special =0;

            for ch in password.chars(){
                if ch.is_ascii_lowercase(){
                    lower+=1;
                } else if ch.is_ascii_uppercase() {
                    upper+=1;
                } else if ch.is_ascii_digit() {
                    digits+=1;
                } else {
                    special+=1;
                }
            }

            println!("\nPassword strength:");
            println!("Uppercase letters: {}", upper);
            println!("Lowercase letters: {}", lower);
            println!("Digits: {}", digits);
            println!("Special Characters: {}", special);

            break;

        } else {
            if login_attempt<3 { 
                println!("Incorrect Password, {} Attempts Left.",3-login_attempt);
                continue 'main;
            }        
            else {
                println!("Maximum 3 Attempts Done, Account Locked...!");
                break;
            }
        }

    }

}
