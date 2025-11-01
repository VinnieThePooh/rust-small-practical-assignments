// individual lints, or even lint groups may be specified
// #![allow(dead_code, non_snake_case, unused_variables)]
// lints =  rustc lints + clippy lints
// cargo clippy - check
// cargo clippy --fix
#![deny(unused, nonstandard_style)]

fn main() {
    let foo = get_random_point();
    let bar = get_random_point();
    println!("{foo:?}");
    println!("{}", bar.x);
    println!("{}", bar.y);
}

#[derive(Debug)]
struct Point {
    x: i8,
    y: i8,
}

fn get_random_point() -> Point {
    Point {
        x: rand::random(),
        y: rand::random(),
    }
}