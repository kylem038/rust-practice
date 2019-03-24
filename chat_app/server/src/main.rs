use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL_HOST: &str = "127.0.0.1.6000";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
    let server = TcpListener::bind(LOCAL_HOST).expect("Failed to bind to local host");
    server.set_nonblocking(true).expect("Failed to turn on non-blocking mode");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx =  tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));
            thread::spawn(move || loop {
                let mut buffer = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buffer) {
                    Ok(_) => {
                        let msg = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf message");

                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("failed to send message to receiver");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }
                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        sleep();
    }
}