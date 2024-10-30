# IDS706_individual_project_2
![CI](https://github.com/nogibjj/IDS706_individual_project_2/actions/workflows/CICD.yaml/badge.svg)

A Rust-based CLI application to manage student grades using SQLite.

## Project Overview

This project provides functionalities to:

- Connect to a SQLite database.
- Create tables with composite primary keys.
- Perform CRUD (Create, Read, Update, Delete) operations.
- Calculate average grades per student.
- Identify high achievers.

## Features

- **Rust Implementation**: Leveraging Rust's performance and safety.
- **SQLite Integration**: Efficient local data storage.
- **Comprehensive Testing**: Ensuring reliability and correctness.
- **CI/CD Pipeline**: Automated linting, testing, and building using GitLab CI/CD.
- **Optimized Binaries**: Downloadable optimized Rust binaries for deployment.

## Dependencies

- Rust (version >= 1.56)
- `rusqlite` crate for SQLite operations.

## Usage Instructions

1. **Clone the Repository**:
    ```bash
    git clone https://github.com/nogibjj/IDS706_individual_project_2.git
    cd IDS706_individual_project_2
    ```

2. **Build the Project**:
    ```bash
    cargo build
    ```

3. **Run the Application**:
    ```bash
    cargo run
    ```

4. **Generate Rust Binary**
   ```bash
   cargo build --release
   ```

## Running Tests

Execute the test suite using Cargo:
```bash
cargo test
```


## Explanation of the Rust Code

### Dependencies: 
   The code uses the rusqlite crate to interact with the SQLite database. Ensure it's included in your Cargo.toml with the appropriate version and features.

### Functions:
   `connect_to_db`: Establishes a connection to the SQLite database.

   `create_table`: Creates the grades table with a composite primary key (duke_id, assignment_id).

   `insert_data`: Inserts a new grade entry into the grades table.

   `read_data`: Retrieves all records from the grades table.

   `view_data`: Displays all grades in a formatted manner.

   `update_data`: Updates a specific grade entry based on duke_id and assignment_id.

   `delete_data`: Deletes a specific grade entry based on duke_id and assignment_id.

   `get_average_grade_per_student`: Calculates and displays the average grade for each student.

   `get_high_achievers`: Retrieves and displays entries where the grade is greater than 90.


### Main Function:
   * Connects to the database.
   * Creates the grades table if it doesn't exist.
   * Inserts sample data.
   * Displays data after insertion.
   * Updates a specific grade and displays data.
   * Deletes a specific grade and displays data.
   * Retrieves and displays average grades per student.
   * Retrieves and displays high achievers.

### Error Handling: 
   Rust's Result type is used extensively to handle potential errors gracefully. The ? operator propagates errors up the call stack.

## Use of LLM
   I use Copilot to help me generate code comments and debug during the writing process.

## Usage of Binary Artifact
   You can download the artfifact and using such command to run it:
   ```bash
   ./IDS706_individual_project_2
   ```