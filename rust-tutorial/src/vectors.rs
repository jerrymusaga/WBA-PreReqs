
pub fn run(){
    let mut numbers = vec![1,2,3,4,5];
    println!("Arrays:{:?}", numbers);
    //reassign val
    numbers[2] = 20;
    //add on to vectors
    numbers.push(6);
    numbers.push(7);
    //pop off last val
    numbers.pop();
    //loop through vec vals
    for x in numbers.iter(){
        println!("Number:{x}");
    }
    //loop and mutate vals
    for x in numbers.iter_mut(){
        *x*=2;
    }
    println!("Numbers mutation:{:?}", numbers);
    println!("Vectors:{:?}", numbers);
    //get single value
    println!("single val:{}", numbers[0]);
    //get array len
    println!("vector length:{}", numbers.len());
    //arrays are stack allocated
    println!("vector occupies {} bytes", std::mem::size_of_val(&numbers));
    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

}