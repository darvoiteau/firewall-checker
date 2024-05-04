use std::collections::HashMap;
use std::net::{TcpStream, Ipv4Addr, SocketAddr};
use std::time::Duration;
extern crate csv;
use std::fs::File;
use csv::Writer;

pub fn port_analysis(dst: Vec<String>, port: Vec<u16>, filename: String, time: u64){
    let nbelem = dst.len();
    let mut i: usize = 0;
    let mut port_state = HashMap::new();
    let file_export = File::create(filename);
    let file = match file_export {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Creation file error !: {}", e);
            return;
        }
    };
    //Create a writer
    let mut writer = Writer::from_writer(file);
    //Write columns in the csv file
    if let Err(e) = writer.write_record(&["Destination", "Port", "State"]) {
        eprintln!("Error! : {}", e);
    }

    while i < nbelem {
        //Call contact port function to check if the destination port is accessible
        port_state = contact_port(&dst[i], &port, time);
        println!("===================================== Result for: {} =====================================\n", &dst[i]);
        for (key, value) in &port_state{
            //Print state (open or closed) for each port
            println!("Port:   {key}         State:   {value}\n");
            let key_in_string = key.to_string();
            //Write the state of each port in the csv file
            if let Err(e) = writer.write_record(&[&dst[i], &key_in_string, value]) {
                eprintln!("Error ! {}", e);
                return;
            }
        }
        i += 1;
        
    }
    //Flush the writer to ensure all data are written in the csv file.
    writer.flush().expect("Failed to flush writer");

}

pub fn contact_port(dst: &String, port: &Vec<u16>, time: u64) -> HashMap<u16, String> {
    
    let mut port_status = HashMap::new();
    
    //Convert given user param to Ipv4Addr type
    let dst_addr: Ipv4Addr = dst.parse().expect("Invalid destination IP address");
    let dst_port = port;
    let elem_nb = &port.len();

    let mut i: usize = 0;

    while &i < elem_nb {
    //Create the socket
    let port_socket = SocketAddr::new(dst_addr.into(), dst_port[i]);
    //Try to open the socket
    let stream_result = TcpStream::connect_timeout(&port_socket, Duration::from_millis(time));

        match stream_result {
            //If the socket is open, the port is accessible
            Ok(_stream) => {
              
                port_status.insert(dst_port[i], String::from("Open"));
                
            }
            //If the port is closed, the socket cannot be set-up and we have an error
            Err(_e) => {
                port_status.insert(dst_port[i], String::from("Closed"));
            }
        }
        i += 1;
    }
    port_status    
}