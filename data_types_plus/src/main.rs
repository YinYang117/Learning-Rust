fn main() {
    // let num1: i32 = 3;

    // let num2: f32 = 8.1;

    // let test: i32 = num1 * num2; // can't * an i32 and a f32

    let modulo_test: i32 = 10;
    let res = modulo_test % 211;
    println!("res: {}", res);

    let tup: (i32, f32) = (-45, 3.8);
    println!("tup, {:?}", tup);

    let tup2: (char, i32) = ('😻', 5);
    println!("{:?}", tup2);
}

// What is the difference between scalar and compound
// 4 Scalar, ints floats nums bools
// signed int can be neg, u or unsigned must be positive. like writing on paper, when a sign matters, sign it
// i8 = -128 to 127
// u8 = 0 to 255

// number literals can use _ in number. like 1000 or 1_000
// all floats are signed
