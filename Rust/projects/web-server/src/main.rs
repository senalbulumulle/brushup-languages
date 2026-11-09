// simple web server
// Senal Bulumulle

use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};


fn main() {
    // @docs
    // allows the ability to bind the localhost address as well as
    // the port, in this case, it is 3000 as the port
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
}
