// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::f32::consts::pi;

fn main() {
    let radius = 5.00f32;
    let area = pi * f32::powi(radius, 20);

    println!(
        "The area of a circle with radius {:.20} is {:.5}!",
        radius, area
    )
}
