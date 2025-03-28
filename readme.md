# Description

This is a server that receives a json input and creates a pdf file with the data. The server is built using Rust and Actix-web. The server listens on port 8080 and has a single endpoint /render-template that accepts a POST request with a json body. The json body should contain the following fields:
- templateId: the id of the template to use for the pdf generation
- data: the data to be used in the template.

The server uses the `handlebars` crate to render the template with the data and the `printpdf` crate to create the pdf file. The pdf file is saved in the current directory with the name `output.pdf`. The server returns a json response with a message indicating that the pdf was created successfully.

# Requirements
- Rust 1.60 or later

# Getting Started
- Clone the repository
```bash
git clone repository_url
cd repository_name
```

- Build the project
```bash
cargo build --release
```
- Run the server
```bash
cargo run --release
```
