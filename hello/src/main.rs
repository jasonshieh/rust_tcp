use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;

fn handle_client(mut stream: TcpStream){

    let mut data = [0 as u8; 50]; // using 50 byte buffer for receiving data
    while match stream.read(&mut data) {
         Ok(size) => {
            // reply to client and echo str when the size of data over 0!
            if size > 0 {
                stream.write(&data[0..size]).unwrap();
                println!("received text: {}", str::from_utf8(&data).unwrap());
            }
            true
         },
         Err(_) => {
             // handle error
            println!("An error occured, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
         }
     }{}

}

fn main() {

    // bind a port to receive clients
    let listener = TcpListener::bind("0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for eacjh one println!("Server lisening on port 3333");
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // print when accept a coneection and print the ip address
                println!("New connection: {}", stream.peer_addr().unwrap());
                // start a new thread to handle the connection
                thread::spawn(move|| {
                    //connection succeeded
                    handle_client(stream)
                 });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }

    }
    // close the socket server
    drop(listener);
}
