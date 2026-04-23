fn main() {
    // let num1: i32 = 3;

    // let num2: f32 = 8.1;

    // let test: i32 = num1 * num2; // can't * an i32 and a f32

    // let modulo_test: i32 = 10;
    // let res = modulo_test % 211;
    // println!("res: {}", res);

    // let tup: (i32, f32) = (-45, 3.8);
    // println!("tup, {:?}", tup);

    // let mut tup2: (char, i32) = ('😻', 5);
    // println!("tup2 {:?}", tup2);
    // {
    //     tup2.1 += 6;
    //     println!("in cur {}", tup2.1);
    // }
    // println!("tup2 {:?}", tup2);

    let mut arr = [1, 2, 3, 4, 5];
    let mut zero_idx = arr[0];
    println!("mut pre {}", zero_idx);
    zero_idx += 11;
    arr[4] += 289;
    println!("mut aft {}", zero_idx);
    println!("arr {:?}", arr);

    
  let t = ([1; 2], [3; 4]); // ([1, 1], [3, 3, 3, 3])
  let (a, b) = t; // destructuring
  println!("{}", a[0] + t.1[0]); // will print 4.

}

// What is the difference between scalar and compound
// 4 Scalar, ints floats nums bools
// signed int can be neg, u or unsigned must be positive. like writing on paper, when a sign matters, sign it
// i8 = -128 to 127
// u8 = 0 to 255

// number literals can use _ in number. like 1000 or 1_000
// all floats are signed
