pub fn run(){
    //default is i32
    let x = 1;
    //default is f64
    let y = 2.5;
    //add explicit type
    let z: i64 = 235335343;
    //find max
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64:{}", std::i64::MAX);

    //boolean
    let is_active = true;

    //get boolean from expression
    let is_greater = 10 > 5;

    //char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x,y,z,is_active, is_greater, a1, face));
}