pub fn run(){
    let person: (&str, &str, i16) = ("Jerry", "Mars", 23);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}