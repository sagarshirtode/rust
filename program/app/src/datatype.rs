fn main(){
    println!("Data types in rust");
    // Scalar Types
    // Integer, String, Boolean, Char, Floating

    // Integer (8but,16,32,64,128, arch / signed(i) or unsign(u))
    let age = 28;
    println!("{age}");

    let is_active = true;
    let class = "A";
    let lang = "rust";
    let weight = 59.9;
    println!("Class {class} is {is_active} lan:{lang} weight : {weight}");

    // Compound types Array , dic, tuples store multiple values
    let temp_list = (70.2, 40.5,12.1, 90.4);
    println!("temp_list= {:?}",temp_list);


}