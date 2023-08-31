fn main() {
    println!("Hello, world!");

    // Vectors
    let _v: Vec<i32> = Vec::new();
    let _vv = vec![1, 2, 3];
    let mut vvv = Vec::new();

    vvv.push(5);
    vvv.push(6);
    vvv.push(7);
    vvv.push(8);
    println!("vvv: {:#?}", vvv);

    let third: &i32 = &vvv[2];
    println!("third: {}", third);

    let third_2: Option<&i32> = vvv.get(2);
    println!("third_2: {:#?}", third_2);
    match third_2 {
        Some(third_2) => println!("The third_2 element is {third_2}"),
        None => println!("There is no third element"),
    }
    println!("{:#?}", third_2)

}
