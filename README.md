# CoreGuard-RS: Nuclear Reactor Monitoring API

This is a safety-critical monitoring system simulation built in the Rust programming language. It manages a registry of nuclear reactor zones, allowing operators to add, track, and decommission reactors while ensuring data integrity through Rust's strict ownership and error-handling models. This project served as my introduction to systems programming with Rust.

## Instructions for Build and Use

Steps to build and/or run the software:

1. **Install Rust:** Ensure you have the Rust toolchain installed (check with cargo --version).
2. **Clone and Navigate:** Download the repository and open your terminal in the coreguard_rs directory.
3. **Build & Run:** Execute the command cargo run to compile and launch the API Command Center.
4. **Test:** Execute cargo test to run the automated unit tests verifying the safety logic.

Instructions for using the software:

1. **Add a Reactor:** Select option 2 and enter a unique numerical ID (e.g., 101).
2. **Monitor Status:** Select option 1 to view all active reactors and their safety levels (e.g., Stable).
3. **Decommission:** Select option 3 and provide a specific ID to remove a reactor from the registry.
4. **Error Handling:** Attempting to add a duplicate ID or remove a non-existent ID will trigger the safety-reporting system.

## Development Environment

To recreate the development environment, you need the following software and/or libraries:

1. **Rust Toolchain:** Version 1.70+ (includes rustc and cargo).
2. **Visual Studio Code:** Used as the primary IDE.
3. **rust-analyzer Extension:** Essential for real-time syntax checking and borrow-checker feedback.
4. **Standard Library (std):** Utilized for io (input/output), vec (collections), and string processing.

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust By Example](https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html)
* [Welcome to Comprehensive Rust 🦀](https://google.github.io/comprehensive-rust/)
* [W3 Schools Rust Tutorial](https://www.w3schools.com/rust/index.php)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] **Network Integration:** Transition from a Console API to a real RESTful API using the Axum or Rocket crates.
* [ ] **Physical Hardware Interaction:** Port the monitoring logic to an STM32 NUCLEO-L476RG using Rust's embedded-hal.
* [ ] **Persistence:** Implement file I/O to save the reactor registry to a .json or .csv file so data is not lost on shutdown.
