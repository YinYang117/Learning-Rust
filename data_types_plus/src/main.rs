fn main() {
    let x = 5;
    println!("The val of x is {x}");
    let x = 6;
    println!("The val of x is {x}");
}

// this is called shadowing. The second variable is now the main x, 
// but if the second variable goes away (like in a sub scope) x returns to the prev val

fn main2() {
    let y: i32 = 10;
    {
        let y = 20;
        println!("the inner val of y is {}", y);
    }
    println!("the outer val y is {}", y);
}

