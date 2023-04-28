pub fn run(){
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    let name = "Jerry";
    let status = "100%";

    // println!("{:?}", args);

    if command == "hello"{
        println!("Hi {} how are you?", name);
    }else if command == "status"{
        println!("Status is {status}");
    }else {
        println!("That is not a valid command");
    }
}