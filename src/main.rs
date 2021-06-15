#[derive(Debug)]  
enum Employee {  
    Name(String),  
    Id(i32),  
    Profile(String),  
}  
fn main() {  
      
    let n = Employee::Name("Hema".to_string());  
    let i = Employee::Id(2);  
    let p = Employee::Profile("Computer Engineer".to_string());  
    println!(" {:?} s {:?} b {:?}", n,i,p);  
}  