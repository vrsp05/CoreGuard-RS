// CoreGuard - A nuclear reactor monitoring API based on Rust.

/* Importing all the needed libraries of this program. */
// Use the standard library for input/output operations.
use std::io;

/* Creating all the needed structs of this program. */
// Creating our struct to represent the reactor's and its properties.
struct Reactor {

    // The unique identifier of the reactor.
    reactor_id: u32,
    // The curreent status of the reactor, which can be "Online", "Offline", or "Meltdown", etc.
    reactor_status: String,

} // End of the Reactor struct definition.

// The main function of the program.
fn main() {

    // Creating a new vector to store the registered reactors in the system.
    let mut plant_registry = Vec::new();

    // Creating the first reactor and adding it to the registry.
    let reactor_1 = Reactor {
        reactor_id: 101,
        reactor_status: String::from("Stable"),
    };

    // Adding the first reactor to the plant registry.
    plant_registry.push(reactor_1);

    // Looping through the plant registry and printing the details of each reactor.
    for reactor in &plant_registry {
        println!("Reactor ID: {}, Status: {}", reactor.reactor_id, reactor.reactor_status);
    } // End of the loop/

} // End of the main function.