fn main() {
    // print
    println!("Hello, world!");
    // variables
    let mut a = 45;
    println!("1 :{a}");
    println!("2 :{}",a);
    a = 100;
    println!("3 :{}",a);

    let mut name = "My name is sagar";
    println!("{name}");

    name = "My name is sagar123";
    println!("{name}");
    printmy_number(10);
    let fullname= "sagar shirtode";
    print_name(fullname);
}
 // functions
 fn printmy_number(val : i32){
    println!("4 : {}",val)
}
fn print_name(val : &str){
    println!("My full name is  : {val}")
}
