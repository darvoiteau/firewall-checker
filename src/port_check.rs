use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::{IpAddr, SocketAddr, TcpStream};
use tokio::time::Duration;
use tokio::sync::Semaphore;
use tokio::task;
use tokio::task::JoinHandle;
extern crate csv;
use std::fs::File;
use csv::Writer;


#[tokio::main]
pub async fn port_analysis(dst: Vec<String>, port: Vec<u16>, filename: String, time: u64, worker: usize){
    let nbelem = dst.len();
    let mut i: usize = 0;
    let port_state: Arc<Mutex<HashMap<u16, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut task_vec: Vec<JoinHandle<(String, HashMap<u16, String>)>> = Vec::new();
    let sem = Arc::new(Semaphore::new(worker));

    // Create file and CSV writer
    let file_export = File::create(filename);
    let file = match file_export {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Creation file error !: {}", e);
            return;
        }
    };
    let mut writer = Writer::from_writer(file);
    if let Err(e) = writer.write_record(&["Destination", "Port", "State"]) {
        eprintln!("Error! : {}", e);
    }

    while i < nbelem {
        let port_state_cln = Arc::clone(&port_state);
        let dst_cln = dst[i].clone();
        let port_cln = port.clone();
        let sem_clone = Arc::clone(&sem);

        let task = task::spawn(async move {
            let task_exec_permit = sem_clone.acquire().await.unwrap();
            // Contact the port and get the result
            let mut ports_result = HashMap::new();
            let ports_result2 = contact_port(&dst_cln, &port_cln, time);
            ports_result.extend(ports_result2);

            // Lock the global port state (if needed)
            let mut port_state_guard = port_state_cln.lock().unwrap();
            port_state_guard.extend(ports_result.iter().map(|(k, v)| (*k, v.clone())));

            // Sort ports from largest to smallest (or smallest to largest, as you prefer)
            let mut ports: Vec<_> = ports_result.iter().collect();
            ports.sort_by_key(|(k, _)| *k);

            println!("===================================== Result for: {} =====================================\n", &dst_cln);
            for (key, value) in &ports {
                println!("Port:   {key}         State:   {value}\n");
            }

            drop(task_exec_permit);
            // Return the result for this task
            (dst_cln, ports_result)
        });

        task_vec.push(task);
        i += 1;
    }

    // Await all tasks (this is where the results will be collected)
    for task in task_vec {
        let (dst, ports) = task.await.unwrap();  // Ensure you handle the result properly here
        // Sort the ports before writing to the CSV
        let mut ports_sorted: Vec<_> = ports.iter().collect();
        ports_sorted.sort_by_key(|(k, _)| *k); // Sort by port number (u16)

        // Write the sorted results to the CSV file
        for (key, value) in &ports_sorted {
            let key_in_string = key.to_string();
            if let Err(e) = writer.write_record(&[&dst, &key_in_string, value]) {
                eprintln!("Error writing to CSV! {}", e);
            }
        }
    }

    // Flush the writer to ensure all data is written
    writer.flush().expect("Failed to flush writer");
}

pub fn contact_port(dst: &String, port: &Vec<u16>, time: u64) -> HashMap<u16, String> {
    
    let mut port_status = HashMap::new();
    
    //Convert given user param to Ipv4Addr type
    let dst_addr: IpAddr = dst.parse().expect("Invalid destination IP address");
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