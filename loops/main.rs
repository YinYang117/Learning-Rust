fn main () {
    let mut counter = 1;
    loop {
        counter += 1;
        println!("{}", counter);
        if counter == 10 {break};
    };
    println!("done");
}