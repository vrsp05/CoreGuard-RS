// CoreGuard - A nuclear reactor monitoring API based on Rust.

/* Importing all the needed libraries of this program. */
use std::io;

/* Creating all the needed structs and enums of this program. */

// Creating our struct to represent the reactor's and its properties.
struct Reactor {

    // The unique identifier of the reactor.
    reactor_id: u32,
    // The current status of the reactor, which can be "Stable", "Warning", or "Critical".
    reactor_status: ReactorStatus,

} // End of the Reactor struct definition.

// Creating an enum to represent the different status of a reactor.
#[derive(Debug, PartialEq)]
enum ReactorStatus {
    Stable,
    Warning,
    Critical,
} // End of the ReactorStatus enum definition.

// The main function of the program.
fn main() {

    // Creating a new vector to store the registered reactors in the system.
    let mut plant_registry = Vec::new();

    // Creating the first reactor and adding it to the registry.
    let reactor_1 = Reactor {
        reactor_id: 101,
        reactor_status: ReactorStatus::Stable,
    };

    // Adding the first reactor to the plant registry.
    plant_registry.push(reactor_1);

    // Looping through the plant registry and printing the details of each reactor.
    for reactor in &plant_registry {
        println!("Reactor ID: {}, Status: {}", reactor.reactor_id, get_safety_status(&reactor.reactor_status));
    } // End of the loop/

    // Calling the find_reactor function to find a reactor with ID 101 and handling the result.
    match find_reactor(&plant_registry, 101)
    {   
        // If the reactor is found, print its details.
        Ok(reactor) => println!("Reactor found: ID {}, Status: {}", reactor.reactor_id, get_safety_status(&reactor.reactor_status)),
        Err(e) => println!("Search failed. System Status: {}", get_safety_status(&e)),
    }

} // End of the main function.

// Function to find a reactor in the plant registry by its ID.
fn find_reactor(plant_registry: &Vec<Reactor>, reactor_id: u32) -> Result<&Reactor, ReactorStatus>
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
    Err(ReactorStatus::Critical)

} // End of the find_reactor function definition.

// Fucntion that matches the ReactorStatus enum to print the status as a string.
fn get_safety_status(status: &ReactorStatus) -> &str
{
    match status
    {
        ReactorStatus::Stable => "Stable",
        ReactorStatus::Warning => "Warning",
        ReactorStatus::Critical => "Critical",
    }
} // End of the get_safety_status function definition.

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
        let reactor_1 = Reactor {
            reactor_id: 101,
            reactor_status: ReactorStatus::Stable,
        };

        // Adding the reactor to the plant registry.
        plant_registry.push(reactor_1);

        // Testing the find_reactor function to find the reactor with ID 101.
        let result = find_reactor(&plant_registry, 101);

        // Assert result.
        assert!(result.is_ok());

    
    } // End of the find_test_find_reactor test function definition.

} // End of the tests module.