// Import packages
// Std io errorkind read and write
use std::io::{ErrorKind, Read, Write};
// std net tcplistener
use std::net::TcpListener;
// std sync mpsc
use std::sync::mpsc;
// std thread
use std::thread;

// Create constants for address and message size
const LOCAL_HOST: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

// Sleep function
fn sleep() {
    // Extend sleep on a thread, taking in an extension to std time during from millis 100
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
    // Create server variable which is a TCP listener bound to local host, catching failure also
    let server = TcpListener::bind(LOCAL_HOST).expect("Failed to bind to local host");
    // Set server to nonblocking and catch failure
    server.set_nonblocking(true).expect("Failed to turn on non-blocking mode");

    // Create an empty, mutable vector named clients (remember ! on vec)
    let mut clients = vec![];
    // Create a duple (tx, rx) equal to a mpsc channel sending strings
    let (tx, rx) = mpsc::channel::<String>();
    // loop
    loop {
        // If we get an Ok, accept mutable socket and address equal to server accept
        if let Ok((mut socket, addr)) = server.accept() {
            // print out client address that connected
            println!("Client {} connected", addr);

            // create a variable 
            let tx =  tx.clone();
            // Try to push onto the clients vec a socket, catch failure
            clients.push(socket.try_clone().expect("failed to clone client"));
            // Spawn a thread, taking in move => a closure with no arg => then loop
            thread::spawn(move || loop {
                // While looping create a mutable variable (buffer) equal to a mutable vector containing 0; and the message size
                let mut buffer = vec![0; MSG_SIZE];
                // Match while running an exact read of the socket and taking in a reference to the mutable buffer
                match socket.read_exact(&mut buffer) {
                    // if we get an Ok create lambda taking in (_)
                    Ok(_) => {
                        // In the lambda create a reassignable msg equal to the iterable of buffer and call take_while, taking in closure
                        // On the closure take in a ref to x returning when x != 0 - call collect on the result results and put it in a vector
                        let msg = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        // Create another msg setting it equal to a string, extending from utf8 taking in the previous message var
                        // Catch the failure
                        let msg = String::from_utf8(msg).expect("Invalid utf message");

                        // Print out the address and the msg (using {:?}) for message
                        println!("{}: {:?}", addr, msg);
                        // call send on tx and catch failure
                        tx.send(msg).expect("failed to send message to receiver");
                    },
                    // Need to catch both blocking and non-blocking errors
                    // Create an error, taking in an reference to err, where if the kind of the error would block just return empty ()
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    // Create an error, taking in (_) that just prints the closing addr and break out
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }
                // We don't want the program infintely looping looking for messages so let's limit this with a sleep function
                // Write sleep function
                sleep();
            });
        }

        // At this point if we haven't errored again check if we are Ok, taking in a msg - equal to the rx.try_recv()
        if let Ok(msg) = rx.try_recv() {
            // reassign clients to equal iterable of itself, calling filter_map and taking in lambda with accepts mut client
            clients = clients.into_iter().filter_map(|mut client| {
                // Create mutable buff variable equal to a clone of the msg and transfer into bytes
                let mut buff = msg.clone().into_bytes();
                // resize the buff to the msg size, 0.
                buff.resize(MSG_SIZE, 0);

                // use client to write_all of the reference to the buff, then map over it taking in _ for closure called client, then call ok
                client.write_all(&buff).map(|_| client).ok()
                // On the end of try_recv collect into vector
            }).collect::<Vec<_>>();
        }

        // Call sleep again
        sleep();
    }
}
