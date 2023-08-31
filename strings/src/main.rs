fn main() {
    println!("Hello, world!");

    // &str uses no .to_string() trait to convert to string slice
    let data_1 = "initial contents";
    let s0: &str = data_1;
    println!("s0: {}", s0);

    // String uses .to_string() to convert string literal to String
    let data = "initial contents";
    let s: String = data.to_string();
    println!("s: {}", s);
    // or
    let ss = "initial contents".to_string();
    println!("ss: {}", ss);
    let sss = String::from("intitial contents");
    println!("sss: {}", sss);

    // concat using +
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;

    // concat using macro format!()
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s9 = format!("{s1}-{s2}-{s3}");
    println!("s9: {:#?}", s9);

    // indexing Strings is hard...
    for c in "Зд".chars() {
        println!("{c}");
    }

    // bytes example
    for b in "Зд".bytes() {
        println!("{b}");
    }
    

}
