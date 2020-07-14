// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    // underscores can help in readability

    let width = 4_i32; // type representation
    let height = 7i32; // type representation
    let depth = 1_000; // value readability with underscore, will be ignored

    let area = area_of(width, height);
    println!("Area is {}", area);

    println!("Volume is {}", volume(width, height, depth));
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}

fn area_of(x: i32, y: i32) -> i32 {
    x * y
}
