use std::io::Read;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::str;
use log::{error,info,debug};
mod objects;
use objects::client::Client;




fn main() {
    const server_ip_address:String = String::from("127.0.0.1:8000");
    let connection_listener = TcpListener::bind(server_ip_address).unwrap();

    

    let (tx,rx) = mpsc::channel();



    let pool_for_connections = thread::spawn(move || {
        loop {
        println!("Blocking to accept connections");
        let tcp_listener = connection_listener.accept();
        let stream_socketaddress = tcp_listener.unwrap();
        tx.send(stream_socketaddress);
        }
    });

    handle_connections(rx);

    pool_for_connections.join().unwrap();
}

pub fn handle_connections(rx:Receiver<(TcpStream,SocketAddr)>) -> (){
    let mut connection_threads: Vec<JoinHandle<_>> = Vec::new();
    let mut counter = 0;
    for connection in rx {
        let tcp_stream_clone = connection.0.try_clone();
        let socket_address_clone = connection.1.clone();
        connection_threads[counter] = thread::spawn(move || {
            // Check if the client is registered
            // Give an option to register
            // If doesn't want to register help client log off and kill the thread and resources of this client
            // Else keep accepting messages
            // When the client types quit close the connection and drop the stream

            let mut buf = [0;512];
            println!("Checking client connecting with IP address {} ",socket_address_clone);
            let mut tcp_stream_reader = tcp_stream_clone.unwrap();
            loop {
                match tcp_stream_reader.read(&mut buf) {
                    Ok(datasize) => {

                         
                        
                        let message_string  = str::from_utf8(&mut buf[..]).expect("cannot convert to utf-8").to_owned();
                        //extract first string separated by :
                        let message_vector:Vec<&str> = message_string.splitn(2, ":").collect();
                        
                        match message_vector[0] {
                            "REGISTER" => {
                                //register_user
                                let userpass:Vec<&str> = message_vector[1].split(";").collect();
                                let username = userpass[0].trim();
                                let password = userpass[1].trim();
                                let client  = Client {
                                    username:String::from(username),
                                    password:String::from(password)
                                };
                                

                                //check if username is not null empty or already exists
                                if client.username.is_empty() {
                                    //break out of the loop, kill the thread.
                                    //log the error with client connection details
                                    
                                } else {
                                    if client.password.is_empty() {
                                    //break out of the loop, kill the thread.
                                    //log the error with client connection details
                                    //tell the user the password is empty

                                    } else {
                                        //check if the user name already exists
                                        if user_exists(&client) {
                                        //tell the client the username already exists
                                        //tell the client to choose another username/email
                                        }else{
                                            //save client as a new client
                                            create_new_user(&client);
                                            //login the newly created user
                                            login_user(&client);
                                            //show a list of clients whom it can interact with

                                        }
                                    }
                                }
                            },
                            "LOGIN" => {
                                //login_user
                                //validate user from database
                                //store user in hashmap

                            },
                            "MESSAGE" => {
                                //check weather source user is there in hashmap
                                //check if target user is there in hashmap
                                //relay the message from source to target
                            },
                            "QUIT" => {
                                //quit from server
                                //remove user from hashmap
                            }
                            _ => {
                                //quit from server, invalidate session, drop client connection.
                            }
                        }
                        // match message_array {
                        //     Some(splitted_message) => {
                        //         let message = splitted_message.1;
                        //         let command = splitted_message.0;



                        //     },
                        //     None=> {
                        //         println!("Unkown format of the command sent from client");
                        //     }
                        // }                      

                    },
                    Err(error) => {
                        println!("Error while reading data from stream, seems to be closed");
                        println!("Error thrown by the OS stack :: {}", error);
                    }
                }
            }


        });
        counter += 1; 
    }

    for connection in connection_threads {
        connection.join().unwrap();
    }
}

pub fn user_exists(client:&Client) -> bool{
    //check the database if the username exists
    return true;
}

pub fn create_new_user(client:&Client){

}

pub fn login_user(client:&Client) -> () {

}

pub fn logout_user(client:&Client) -> () {

}
