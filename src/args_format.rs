use ipnet::IpNet;
use regex::Regex;

pub fn port_format(port: &String) -> Vec<u16>{
    let kind: u8;
    let mut user_port: Vec<u16> = Vec::new();
    //Depending the param, we will process the formatting differently
    kind = if port.contains('-'){
        1
    }
    else if port.contains(',') {
        2
    }
    else {
        0
    };

    if kind == 1 {
        let parts: Vec<&str> = port.split('-').collect();
        let mut port_list: Vec<u16> = Vec::new();
        for elem in parts{
            if let Ok(number) = elem.parse::<u16>(){
                port_list.push(number);
            }
            else {
                panic!("Wrong port parameter!");
            }
            
        }
        // We find and take all port between the first and last port given by the user
        if port_list[0] < port_list[1] {
            let mut i: u16 = port_list[0];
            while i <= port_list[1] {
                user_port.push(i);
                i +=1;
            }

        }
        else if port_list [0] > port_list[1] {
            let mut i: u16 = port_list[1];
            while i < port_list[0] {
                user_port.push(i);
                i +=1;
            }
        }
    }
    else if kind == 2 {
        //Simple split to take separately all ports given by the user
        let parts: Vec<&str> = port.split(',').collect();
        for elem in parts {
            if let Ok(number) = elem.parse::<u16>(){
                user_port.push(number);
            }
            else {
                panic!("Wrong port parameter!");
            }

        }
    }
    else if kind == 0 {
        if let Ok(number) = port.parse::<u16>(){
            user_port.push(number);

        }
        else {
            panic!("Wrong port parameter!");
        }
    }
    user_port

}

pub fn dst_format(dst: &String) -> Vec<String>{
    let kind: u8;
    let mut user_dst: Vec<String> = Vec::new();

    kind = if dst.contains(','){
        1
    }
    else if dst.contains('/') {
        2
    }
    else {
        0
    };

    if kind == 1 {
        user_dst = dst.split(',').map(String::from).collect();
    }
    else if kind == 2 {
            

        // Convert to IpNet
        let ip_network: IpNet = dst.parse().expect("Invalid network string");

        // Check if it is a valid network
        if let IpNet::V4(ipv4_net) = ip_network {
        
            for ip in ipv4_net.hosts() {
                // Convertir the Ip to string and insert it in the Vector
                let ip_string = ip.to_string();
                user_dst.push(ip_string);
            }
        }
        else if let IpNet::V6(ipv6_net) = ip_network {
            for ip in ipv6_net.hosts(){
                let ip_string = ip.to_string();
                user_dst.push(ip_string);
            }
        }



    }
    else if kind == 0 {
        
        user_dst.push(dst.clone());

    }
  user_dst
}

//Check if the filename given by the user not contain unwanted char
pub fn filename_check(foutput: &str){
    let unwaned_special_chars_re = Regex::new(r#"[=:*?"',;!{}\[\]()'<>|]+"#).unwrap();
    if unwaned_special_chars_re.is_match(foutput) {
        eprintln!("Error ! The given filename '{}' contain invalid special character", foutput);
        std::process::exit(1);
    }
}