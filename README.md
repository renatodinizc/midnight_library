### BookstoreApi: Rust Book Management System

```
 ____              _        _                                  _ 
|  _ \            | |      | |                     /\         (_)
| |_) | ___   ___ | | _____| |_ ___  _ __ ___     /  \   _ __  _ 
|  _ < / _ \ / _ \| |/ / __| __/ _ \| '__/ _ \   / /\ \ | '_ \| |
| |_) | (_) | (_) |   <\__ \ || (_) | | |  __/  / ____ \| |_) | |
|____/ \___/ \___/|_|\_\___/\__\___/|_|  \___| /_/    \_\ .__/|_|
                                                        | |      
                                                        |_|    
```
A comprehensive Rust web application for managing books. This project is designed to demonstrate the capabilities of Rust in building efficient, safe web applications. It features book management functionalities including listing, adding, and retrieving book details. The system also includes health checking and configuration management, ensuring a robust and customizable solution.

### Installation and Setup

1. **Prerequisites:**
   - Rust and Cargo (Rust's package manager). Visit [the official Rust installation guide](https://www.rust-lang.org/tools/install) for instructions.
   - PostgreSQL database. Ensure it is running and accessible.

2. **Database Setup:**
   - Ensure Docker is installed and running on your system.
   - Run the `init_db.sh` script to automatically set up the PostgreSQL database:
     ```shell
     chmod +x init_db.sh
     ./init_db.sh
     ```
   This script checks for necessary dependencies (`psql` and `sqlx`), sets up environment variables for database credentials, launches a PostgreSQL Docker container if needed, and runs database migrations to prepare the `bookstore_api` database.

3. **Project Setup:**
   - Clone the repository and navigate into the project directory.
   - Build the project:
     ```shell
     cargo build
     ```
   - Run the application:
     ```shell
     cargo run
     ```

### Usage

After setting up the project, you can start interacting with the book management system. The application exposes endpoints for book operations and health checks. Use a tool like `curl` or Postman to interact with the API.

#### Examples:

- **Add a Book:**
  ```shell
  curl -X POST http://localhost:8080/books -d '{"title": "Book Title", "author": "Author Name", "genre": "Preferred Genre}'
  ```

- **List Books:**
  ```shell
  curl http://localhost:8080/books
  ```

- **Health Check:**
  ```shell
  curl http://localhost:8080/health_check
  ```

### Features

- **Book Management:** Add, list, and retrieve books.
- **Health Check Endpoint:** Verify the application status.
- **Configuration Management:** Customize application settings.

### Contributing

We welcome contributions! Please feel free to fork the repository, make your changes, and submit a pull request.

1. **Fork the Repository:** Click on the 'Fork' button at the top right of the page.
2. **Clone Your Fork:** `git clone https://github.com/renatodinizc/bookstore_api.git`
3. **Create a New Branch:** `git checkout -b new-feature`
4. **Make Your Changes:** Implement your feature or fix.
5. **Commit Your Changes:** `git commit -am 'Add some feature'`
6. **Push to the Branch:** `git push origin new-feature`
7. **Submit a Pull Request:** Open a pull request from your fork to the main project.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

---
