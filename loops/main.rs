fn main () {
    let mut counter = 1;
    while counter == 1 {
        counter += 1;
        println!("{}", counter);
        if counter == 10 {break};
        println!("after return")
    };
    println!("done");
}