// CoreGuard - A nuclear reactor monitoring API based on Rust.

/* Importing all the needed libraries of this program. */
use std::io;

/* Creating all the needed structs of this program. */

// Creating our struct to represent the reactor's and its properties.
struct Reactor {

    // The unique identifier of the reactor.
    reactor_id: u32,
    // The current status of the reactor, which can be "Online", "Offline", or "Meltdown", etc.
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

fn find_reactor(plant_registry: &Vec<Reactor>, reactor_id: u32) -> Result<&Reactor, String>
{   
    // Loop through the plant registry to find the reactor with the specified ID.
    for reactor in plant_registry
    {   
        // If the reactor with the specified ID is found, return a reference to it.
        if reactor.reactor_id == reactor_id
        {
            return Ok(reactor);
        }
    } // End of the loop.

    // If the reactor is not found, return an error message.
    Err(String::from("Reactor not found"))

} // End of the find_reactor function definition.

// The test module for the program.
#[cfg(test)]
mod tests {

    // Importing all the needed libraries for testing.
    use super::*;

    #[test]
    fn find_test_find_reactor()
    {   
        // Creating a new vector.
        let mut plant_registry = Vec::new();

        // Creating a new reactor.
        let reacctor_1 = Reactor {
            reactor_id: 101,
            reactor_status: String::from("Stable"),
        };

        // Adding the reactor to the plant registry.
        plant_registry.push(reacctor_1);

        // Testing the find_reactor function to find the reactor with ID 101.
        let result = find_reactor(&plant_registry, 101);

        // Assert result.
        assert!(result.is_ok());

    
    } // End of the find_test_find_reactor test function definition.

} // End of the tests module.