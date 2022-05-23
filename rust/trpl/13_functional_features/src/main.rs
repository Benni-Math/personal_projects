use functional_features::{ generate_workout, generate_workout_rc, filtering_shoes };

fn main() {
    println!("Checkout the code!\n");

    generate_workout(4, 19);

    generate_workout_rc(26, 3);

    if let Err(e) = filtering_shoes() {
        eprintln!("Application error: {}", e);
    }

    println!("\nCheck the iterators.rs or run 'cargo test -- --show-output' to see more iterator stuff.\n")
}