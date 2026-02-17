// CoreGuard - A nuclear reactor monitoring API based on Rust.

/* Importing all the needed libraries of this program. */
use std::io::{self, Write};
use std::vec::Vec;
use std::string::String;


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
    let mut all_plants_registry: Vec<Reactor>= Vec::new();

    // Printing a welcome message to the user.
    println!("Welcome to CoreGuard - A nuclear reactor monitoring API based on Rust!");

    // Loop for main menu.
    loop {

        // Printing the main menu options to the user.
        println!("\n--- CoreGuard API Command Center ---");
        println!("1. List Reactors");
        println!("2. Add Reactor");
        println!("3. Remove Reactor");
        println!("4. Exit");
        print!("Selection > ");
        io::stdout().flush().unwrap(); // Ensures the prompt shows up before input

        // Variable to store the user's choice.
        let mut choice = String::new();

        // Read the user input and handle it.
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        
        // Parsing the user's choice and handling invalid input.
        let choice: u32 = match choice.trim().parse() {
            
            // If the input is valid, return the parsed number.
            Ok(num) => num,

            // If the input is invalid, print an error message and continue the loop.
            Err(_) => {
                
                // Print error message for invalid input.
                println!("Invalid input. Please enter a number between 1 and 4.");
                continue;

            } // End of the error handling for invalid input.
                
        }; // End of the match statement for parsing user input.
        
        // Handling the user's choice using a match statement.
        match choice {
            
            // Logic to list all reactors in the plant registry.
            1 => {
                
                // Logic to list all reactors in the plant registry
                println!("\n--- Registered Reactors ---");

                // Loop throug the plant registry and print details of each reactor.
                for reactor in &all_plants_registry {
                    println!("Reactor ID: {}, Status: {}", reactor.reactor_id,
                     get_safety_status(&reactor.reactor_status));
                };
            }

            // Logic to add a new reactor to the plant registry.
            2 => {

                // Logic to ask for an ID and push a new Reactor to the Vec
                print!("Enter new Reactor ID: ");
                io::stdout().flush().unwrap(); // Ensures the prompt shows up before input
                
                // Variable to store the new reactor ID input from the user.
                let mut new_id = String::new();

                // Read the user input for the new reactor ID and handle it.
                io::stdin().read_line(&mut new_id).expect("Failed to read input");
                
                // Parsing the new reactor ID input and handling invalid input.
                let new_id: u32 = new_id.trim().parse().expect("Failed to parse ID");
                
                // Creating a new Reactor instance with the provided ID and a default status of "Stable".
                let new_reactor = Reactor {
                    reactor_id: new_id,
                    reactor_status: ReactorStatus::Stable,
                }; // End of the new reactor creation.
                
                // Adding the new reactor to the plant registry.
                all_plants_registry.push(new_reactor);

            } // End of the case for adding a new reactor.

            // Logic to find reactore by ID and remove it from the plant registry.
            3 => {

                // Prompt the user to enter the ID of the reactor.
                println!("Enter Reactor ID to decommission:");

                // Variable to store the reactor ID input from the user.
                let mut id_input = String::new();
                
                // Read the user input for the reactor ID and handle it.
                io::stdin().read_line(&mut id_input).expect("Failed to read input");

                // Parsing the reactor ID input and handling invalid input.
                let id_input: u32 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {

                        // Print error message for invalid input.
                        println!("Invalid input. Please enter a valid Reactor ID.");
                        continue;

                    } // End of the error handling for invalid input.

                }; // End of the match statement for parsing reactor ID input.

                // Attempt to find the reactor in the plant registry using the provided ID.
                match find_reactor(&all_plants_registry, id_input) {

                    // If the reactor is found, print a message and remove it from the plant registry.
                    Ok(reactor) => {

                        // If the reactor is found, remove it from the plant registry.
                        all_plants_registry.retain(|r| r.reactor_id != id_input);
                        println!("Reactor with ID {} has been decommissioned.", id_input);

                    } // If the reactor is not found, print an error message.

                    // If the reactor is not found, print an error message.
                    Err(_) => {

                        // If the reactor is not found, print an error message.
                        println!("Reactor with ID {} not found.", id_input);
                    
                    } // End of the match statement for finding and removing a reactor.
                
                } // End of the case for finding and removing a reactor.
            
            } // End of the case for removing a reactor.

            // Logic to exit the program when the user selects option 4.
            4 => {

                // Print a shutdown message and break the loop to exit the program.
                println!("Shutting down CoreGuard...");
                break;

            } // End of the case for exiting the program.

            // Handling invalid commands by printing an error message.
            _ => println!("Invalid command. Please try again."),

        } // End of the match statement for handling user choices.
    
    }; // End of the main menu loop.

} // End of the main function.


// FUNCTIONS DEFINITIONS


// Function to find a reactor in the plant registry by its ID and delete it if found.
fn find_reactor(plant_registry: &Vec<Reactor>, reactor_id: u32) -> 
Result<&Reactor, ReactorStatus>
{   
    // Loop through the plant registry to find the reactor with the specified ID.
    for reactor in plant_registry
    {   
        // If the reactor with the specified ID is found, return a reference to it.
        if reactor.reactor_id == reactor_id
        {   
            // Return the reactor was found and deleted successfully.
            return Ok(reactor);

        } // End of the if statement for finding the reactor.
        
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


// PROGRAM TESTS

#[cfg(test)]
mod test_1 {
    use super::*;

    // 1. Test adding and finding a reactor
    #[test]
    fn test_add_and_find() {
        let mut registry = Vec::new();
        let id = 505;
        registry.push(Reactor { reactor_id: id, reactor_status: ReactorStatus::Stable });
        
        let result = find_reactor(&registry, id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().reactor_id, id);
    }

    // 2. Test searching for a reactor that DOES NOT exist
    #[test]
    fn test_find_nonexistent_reactor() {
        let registry = Vec::new();
        let result = find_reactor(&registry, 999);
        assert!(result.is_err());
    }

    // 3. Test the removal logic
    #[test]
    fn test_remove_reactor() {
        let mut registry = Vec::new();
        registry.push(Reactor { reactor_id: 1, reactor_status: ReactorStatus::Stable });
        registry.push(Reactor { reactor_id: 2, reactor_status: ReactorStatus::Stable });

        // Simulate your removal logic:
        registry.retain(|r| r.reactor_id != 1);

        assert_eq!(registry.len(), 1);
        assert_eq!(registry[0].reactor_id, 2);
    }

    // 4. Test safety status string conversion
    #[test]
    fn test_status_strings() {
        assert_eq!(get_safety_status(&ReactorStatus::Critical), "Critical");
        assert_eq!(get_safety_status(&ReactorStatus::Stable), "Stable");
    }
}

// The test module for the program.
#[cfg(test)]
mod test_2 {

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
        let result = find_reactor(
            &plant_registry, 101);

        // Assert result.
        assert!(result.is_ok());

    
    } // End of the find_test_find_reactor test function definition.

} // End of the tests module.