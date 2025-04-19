use argh::FromArgs;
mod args_format;
mod port_check;

struct Scan {
    port_number: Vec<u16>,
    destination_ip: Vec<String>,
}

#[derive(FromArgs)]
/// reach new args
struct Args {
    
    #[argh(option, short = 'p')]
    ///port(s) to check. Ex: 80-443 -> ports 80 from 443 will be checked. Ex: 80,443 -> ports 80 and 443 only will be checked.
    port: String,

    #[argh(option, short = 'd')]
    ///destination ip.  Ex:192.168.1.0/24 -> network destination. Ex: 192.168.1.1,192.168.1.2 -> multiple targets.
    destination: String,

    #[argh(option, short= 'o', default = r#"String::from("result.csv")"#)]
    ///filename to export result in CSV.
    output: String,

    #[argh(option, short= 't', default = "1000")]
    ///time between each paket.
    time: u64,


}

fn main() {
    //Catch params given by the user
    let options: Args = argh::from_env();
    let scan_port = port_init(&options);
    //Verify if the filename given by the user not content any unwanted char
    args_format::filename_check(&options.output.as_str());
    port_check::port_analysis(scan_port.destination_ip, scan_port.port_number, options.output, options.time);


}

//Init Struct Scan
fn port_init(options: &Args) -> Scan {
    let ports = args_format::port_format(&options.port);
    let dst = args_format::dst_format(&options.destination);
    let scan_port = Scan {
        port_number: ports,
        destination_ip: dst,

    };
    scan_port
    
   

}