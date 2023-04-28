pub fn run(){
    let name = "Jerry";
    let mut age = 25;
    println!("My name is {} and I am {}", name, age);
    age = 34;
    println!("My name is {} and I am {}", name, age);

    //Define constants
    const ID: u32 = 001;
    println!("Id:{}",ID);

    //assign multiple vars
    let (my_name, my_age) = ("Jerry", 44);
    println!("{} is {}", my_name, my_age);
}