```
|\/|. _| _ . _ |_ _|_  | .|_  _ _  _
|  ||(_|| ||(_|| | |   |_||_)| (_|| \/
             _|                     /
```
### A Rust Library Management System

A comprehensive Rust web application for managing a library. This project is designed to demonstrate the capabilities of Rust in building efficient, safe web applications. It features library management functionalities including listing, adding, and retrieving book and authors details. The system also includes health checking and configuration management, ensuring a robust and customizable solution.

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
   This script checks for necessary dependencies (`psql` and `sqlx`), sets up environment variables for database credentials, launches a PostgreSQL Docker container if needed, and runs database migrations to prepare the `midnight_library` database.

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

After setting up the project, you can start interacting with the book management system. The application exposes endpoints for book and author operations and health checks. Use a tool like `curl` or Postman to interact with the API.

#### Examples:

- **Add an Author:**
  ```shell
    curl -X POST http://localhost:8080/books -d '{"name": "Herman Melville", "nationality": "American"}'
    # { "author_id": "e457c912-5a04-4bfc-abeb-5a0e2fe91a72", "message": "Author created successfully!" }
  ```

- **List Books:**
  ```shell
  curl http://localhost:8080/books
  #[
  #  {
  #      "author": "Eiichiro Oda",
  #      "created_at": "2024-03-10T10:22:58.244130Z",
  #      "genre": "Shounen",
  #      "id": "a56de2a8-61d3-43f4-b66b-b454c2b54589",
  #      "title": "One Piece"
  #  },
  #  {
  #      "author": "Akira Toriyama",
  #      "created_at": "2024-03-10T14:28:44.178201Z",
  #      "genre": "Shounen",
  #      "id": "82648e74-3fb4-4fe2-a4a2-5f6db5d20d3b",
  #      "title": "Dragon Ball"
  #  },
  #]
  ```
- **Show details of an Author:**
  ```shell
  curl http://localhost:8080/authors/a56de2a8-61d3-43f4-b66b-b454c2b54589
  #{
  #  "created_at": "2024-03-10T10:22:58.244130Z",
  #  "id": "a56de2a8-61d3-43f4-b66b-b454c2b54589",
  #  "name": "Eiichiro Oda",
  #  "nationality": "Japanese"
  #}
  ```

- **Delete a Book:**
  ```shell
    curl -X POST http://localhost:8080/authors/delete -d '{"id": "f6eed69c-d93a-48ff-b80b-dfdf4df061fa"}'
    # { "message": "Book deleted successfully!" }
  ```

- **Health Check:**
  ```shell
  curl http://localhost:8080/health_check
  ```

### Features

- **Book Management:** Add, list, show details and retrieve books.
- **Author Management:** Add, list, show details and retrieve authors.
- **Health Check Endpoint:** Verify the application status.
- **Configuration Management:** Customize application settings.

### Contributing

We welcome contributions! Please feel free to fork the repository, make your changes, and submit a pull request.

1. **Fork the Repository:** Click on the 'Fork' button at the top right of the page.
2. **Clone Your Fork:** `git clone https://github.com/renatodinizc/midnight_library.git`
3. **Create a New Branch:** `git checkout -b new-feature`
4. **Make Your Changes:** Implement your feature or fix.
5. **Commit Your Changes:** `git commit -am 'Add some feature'`
6. **Push to the Branch:** `git push origin new-feature`
7. **Submit a Pull Request:** Open a pull request from your fork to the main project.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

---
