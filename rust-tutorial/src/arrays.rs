use core::num;

pub fn run(){
    let mut numbers: [i32;5] = [1,2,3,4,5];
    println!("Arrays:{:?}", numbers);
    //reassign val
    numbers[2] = 20;
    println!("Arrays:{:?}", numbers);
    //get single value
    println!("single val:{}", numbers[0]);
    //get array len
    println!("array length:{}", numbers.len());
    //arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

}