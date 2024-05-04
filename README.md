# Firewall-Checker
A simple tool developed in Rust that lets you check whether a tcp port is open.
Ideal for ensuring that a server has no more ports open than necessary. 
Ideal for making sure your firewall has no more ports open than necessary.

## Features

- Check if tcp port is open
- Ability to scan several destination IPs at once
- Export results as csv

## Installation

Instructions d'installation du projet.

```bash
git clone https://github.com/darvoiteau/firewall-checker.git
cd firewall-checker
cargo build --release
cd target/release
./firewallchecker --help

**** or ****

Download the release here to get the binary executable: <a href="https://github.com/darvoiteau/firewall-checker/releases">firewall-checker releases</a>

chmod +x firewallchecker-xxxxx
./firewallchecker-xxxxx --help


## Usage
Options:
  -p, --port        port(s) to check. Ex: 80-443 -> ports 80 from 443 will be
                    checked. Ex: 80,443 -> ports 80 and 443 only will be
                    checked.
  -d, --destination destination ip.  Ex:192.168.1.0/24 -> network destination.
                    Ex: 192.168.1.1,192.168.1.2 -> multiple targets.
  -o, --output      filename to export result in CSV.
  -t, --time        time between each paket.
  --help            display usage information

### --port
Set single destination port: -p 80 or --port 80
Set multiple destination port: -p 80,443,... or --port 80,443,... -> port 80 and 443 will be checked
Set range destination port: -p 80-443 or -- port 80-443 -> port from 80 to 443 will be checked

### --destination
Set single destination ip: -d 192.168.1.1 or --destination 192.168.1.1
Set multiple destination ip: -d 192.168.1.1,192.168.1.2 or --destination 192.168.1.1,192.168.1.2
Set a destination network ip: -d 192.168.1.0/24 or --destination 192.168.1.0/24

### --time
This option set the duration (in ms) between each packet sent to check tcp port. By default it is 1000 ms so 1 sec.
Set the duration between each packet: -t 500 or --time 500 -> each packet will be sent each 500 ms.
