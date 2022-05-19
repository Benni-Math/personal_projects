use concurrency::{
    thread_demo, 
    move_semantics,
    channel_demo,
    cloning_transmitters,
};
fn main() {
    println!("\nUsing two threads:");
    thread_demo();

    println!("\nMoving a value to another thread (ownership):");
    move_semantics();

    println!("\nHere we use a channel to connect our threads:");
    channel_demo();

    println!("\nSending messages from multiple threads:");
    cloning_transmitters();
}
