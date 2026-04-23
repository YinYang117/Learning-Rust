rustc <then file name>  i.e.  main.rs

stands for rust compile this file (or build)

run the program by typing the file in cl

i.e.  ./main

after you compile, you have a file that is binary I think, and anything can run it even without having rust installed.
Cargo builds your code and your dependencies

can run   $cargo build
then file name   ./ ...

or can run    $cargo run

when ready for release:   $cargo build --release
will add in optimizations


$cargo update
will update dependancies, despite the cargo lock version locks.