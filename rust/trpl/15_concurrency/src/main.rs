use concurrency::{
    thread_demo, 
    move_semantics,
    channel_demo,
    cloning_transmitters,
    mutex_demo,
    deadlock_demo,
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

    println!("\nA mutex demo:");
    mutex_demo();

    println!("\nFinally, here is a deadlock demo:");
    deadlock_demo();
}
