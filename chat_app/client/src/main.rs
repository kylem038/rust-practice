// Import dependencies
// Need std io including errorkind, read, write and self
use std::io::{self, ErrorKind, Read, Write};
// Need std net tcpstream
use std::net::TcpStream;
// Need std sync mpsc extending self and TryRecvError
use std::sync::mpsc::{self, TryRecvError};
// Need std thread
use std::thread;
// Need std time duration
use std::time::Duration;

// Same constants as we had in the server
const LOCAL_HOST: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    // Create mutable variable client equal to TcpStream and try to connect to local host, catch failure
    let mut client = TcpStream::connect(LOCAL_HOST).expect("Stream failed to connect");
    // Set client to non-blocking and catch failure
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    // Create a channel (duple containing tx and rx) equal to mpsx extending channel and passing through strings
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a thread (literally) passing in a move closure loop
    thread::spawn(move || loop {
        // Create mutable buff equal to mutable vector containing 0, msg_size
        let mut buff = vec![0; MSG_SIZE];
        // Match on client read exact passing in ref to mutable buff
        match client.read_exact(&mut buff) {
            // If we get Ok pass in _ and return...
            Ok(_) => {
                // Create reassignable msg var equal to iterable buff calling take_while checking to see if references inside are = to 0
                // Collect the result into a vector
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                // Print msg
                println!("message received {:?}", msg);
            },
            // Need to check for blocking and non-blocking errors
            // Create Err taking in ref to err, check if error kind would block and return empty function
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            // Create an Err taking in _ and printing that connection was terminated
            Err(_) => {
                println!("connection to server severed");
                break;
            }
        }

        // Next match on rx try recv
        match rx.try_recv() {
            // Create OK passing in msg
            Ok(msg) => {
                // Create mutable variable buff which is equal to the message, cloned and into bytes;
                let mut buff = msg.clone().into_bytes();
                // Resize the buff into msg_size, 0
                buff.resize(MSG_SIZE, 0);
                // write all to the client passing in reference to buff, catch failure
                client.write_all(&buff).expect("writing to socket failed");
                // Print the message
                println!("message sent {:?}", msg);
            },
            // Check if TryRecv is empty if so just return
            Err(TryRecvError::Empty) => (),
            // Check if error is disconnected and if so break
            Err(TryRecvError::Disconnected) => break
        }

        // Have thread sleep for 100ms
        thread::sleep(Duration::from_millis(100));
    });

    // Outside of thread println saying input message
    println!("Input message:");
    // Loop
    loop {
        // Create new string equal to a mutable var named buff
        let mut buff = String::new();
        // using io stdin to read_line, taking in ref to buff - catch failure
        io::stdin().read_line(&mut buff).expect("reading Stdin failed");
        // create msg equal to buff, trim it and to_string it
        let msg = buff.trim().to_string();
        // Handle quiting out (writing out :quit) or server (tx) sending an error using if statement
        if msg == ":quit" || tx.send(msg).is_err() { break }
    }
    println!("Bye!");
}
