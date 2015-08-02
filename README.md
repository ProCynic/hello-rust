# Hello Rust

A simple hello world toy web server to teach myself rust.
Also I want to evaluate the suitability of rust and the rust ecosystem for
building a JSON API webserver in a MVC style.

## Database
 A note on the database.  This code assumes that there exists a postgres
 database on `localhost:5432` with username `hello_rust` and password
 `hello_rust`.  This can be achieved via docker with the command
 `docker run -d -p 5432:5432 -e "POSTGRES_USER=hello_rust" -e "POSTGRES_PASSWORD=hello_rust" postgres`.
