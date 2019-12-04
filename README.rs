# Hi-CGI
#For GCI
use std::io;
use std::fs::File;
fn main(){
    let introduction = {"Hello, GCI"};
    let size = get_length(&introduction);
    println!("Hello, GCI");
    println!("{} has {} characters", introduction, size);
    let mut enter = String::new  ();
    
    println!("What's your name:");
    
    match io::stdin().read_line(&mut enter){
        Ok(_)=>{
            println!("Wonderful name, you have there {},", enter.to_uppercase());
        },
        Err(e) => println!("Sorry, {}", e)
    }
}

fn get_length(l: & str)-> usize{
     l.chars().count()
}
