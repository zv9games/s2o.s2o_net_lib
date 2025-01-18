use std::fs::File;
use std::io::{self, Read, Write};
use std::net::Ipv4Addr;
use std::path::Path;
use std::collections::HashSet;

pub fn analytics_menu() {
    loop {
        // Clear the screen
        clear_screen();

        println!("Analytics Menu:");
        println!("1. View Packet File");
        println!("9. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => view_packet_file(),
            9 => {
                println!("Exiting analytics menu...");
                break;
            },
            _ => println!("Invalid choice. Please try again."),
        }
    }
}



fn view_packet_file() {
    let file_path = "C:\\Temp\\NetworkCaptureParsed.cap";
    if !Path::new(file_path).exists() {
        println!("No packet file found at {}.", file_path);
        return;
    }

    println!("Reading packet file from {}...", file_path);

    let mut buffer = Vec::new();
    let mut unknown_protocol_count = 0;
    let mut unknown_protocols = HashSet::new();

    match File::open(file_path) {
        Ok(mut file) => {
            if let Err(e) = file.read_to_end(&mut buffer) {
                println!("Failed to read file: {}", e);
                return;
            }

            let mut offset = 0;
            while offset < buffer.len() {
                match parse_packet(&buffer[offset..]) {
                    Ok((packet, size)) => {
                        log_packet_info(&packet);
                        offset += size;
                    }
                    Err(protocol) => {
                        if protocol != 255 {  // Ensure specific error code for length issue is excluded
                            unknown_protocol_count += 1;
                            unknown_protocols.insert(protocol);
                        }
                        offset += 20;  // Move offset to next packet assuming a minimum packet size
                    }
                }
            }

            println!("Displayed packets from the packet file.");
            println!("Number of packets with unknown protocols: {}", unknown_protocol_count);
            println!("Unknown protocols encountered: {:?}", unknown_protocols);

            println!("\nPress Enter to return to the analytics menu...");
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).unwrap();
        }
        Err(e) => {
            println!("Failed to open file at {}: {}", file_path, e);
        }
    }
}




fn parse_packet(data: &[u8]) -> Result<(Packet, usize), u8> {
    if data.len() < 20 {
        println!("Error: Data length is less than 20");
        return Err(255); // Use a specific error code for length issues that does not conflict with known protocols
    }

    let source_ip = Ipv4Addr::new(data[12], data[13], data[14], data[15]);
    let dest_ip = Ipv4Addr::new(data[16], data[17], data[18], data[19]);
    let protocol = data[9];

    println!(
        "Parsing packet: Source IP: {}, Destination IP: {}, Protocol: {}",
        source_ip, dest_ip, protocol
    );

    match protocol {
        0 => Ok((Packet::Hopopt { source_ip, dest_ip }, 20)),
        1 => {
            if data.len() < 28 {
                println!("Error: ICMP data length is less than 28");
                return Err(1);
            }
            Ok((Packet::Icmp { source_ip, dest_ip }, 28))
        }
        6 => {
            if data.len() < 40 {
                println!("Error: TCP data length is less than 40");
                return Err(6);
            }
            let source_port = u16::from_be_bytes([data[20], data[21]]);
            let dest_port = u16::from_be_bytes([data[22], data[23]]);
            Ok((Packet::Tcp { source_ip, dest_ip, source_port, dest_port }, 40))
        }
        17 => {
            if data.len() < 28 {
                println!("Error: UDP data length is less than 28");
                return Err(17);
            }
            let source_port = u16::from_be_bytes([data[20], data[21]]);
            let dest_port = u16::from_be_bytes([data[22], data[23]]);
            Ok((Packet::Udp { source_ip, dest_ip, source_port, dest_port }, 28))
        }
        2 => Ok((Packet::Igmp { source_ip, dest_ip }, 20)),
        7 => Ok((Packet::Cbt { source_ip, dest_ip }, 20)),
        41 => {
            if data.len() < 40 {
                println!("Error: IPv6 data length is less than 40");
                return Err(41);
            }
            Ok((Packet::Ipv6 { source_ip, dest_ip }, 40))
        }
        32 => Ok((Packet::Ah { source_ip, dest_ip }, 20)),
        168 => Ok((Packet::Special { source_ip, dest_ip }, 20)),
        106 => {
            if data.len() < 28 {
                println!("Error: Protocol 106 data length is less than 28");
                return Err(106);
            }
            Ok((Packet::Prot106 { source_ip, dest_ip }, 28))
        }
        

        

        110 => Ok((Packet::Prot110 { source_ip, dest_ip }, 20)),
        117 => Ok((Packet::Prot117 { source_ip, dest_ip }, 20)),
        174 => Ok((Packet::Prot174 { source_ip, dest_ip }, 20)),
        13 => Ok((Packet::Prot13 { source_ip, dest_ip }, 20)),
        124 => Ok((Packet::Prot124 { source_ip, dest_ip }, 20)),
        178 => Ok((Packet::Prot178 { source_ip, dest_ip }, 20)),
        83 => Ok((Packet::Prot83 { source_ip, dest_ip }, 20)),
        14 => Ok((Packet::Prot14 { source_ip, dest_ip }, 20)),
        84 => Ok((Packet::Prot84 { source_ip, dest_ip }, 20)),
        111 => Ok((Packet::Prot111 { source_ip, dest_ip }, 20)),
        46 => Ok((Packet::Prot46 { source_ip, dest_ip }, 20)),
        179 => Ok((Packet::Prot179 { source_ip, dest_ip }, 20)),
        230 => Ok((Packet::Prot230 { source_ip, dest_ip }, 20)),
        219 => Ok((Packet::Prot219 { source_ip, dest_ip }, 20)),
        47 => Ok((Packet::Prot47 { source_ip, dest_ip }, 20)),
        28 => Ok((Packet::Prot28 { source_ip, dest_ip }, 20)),
        203 => Ok((Packet::Prot203 { source_ip, dest_ip }, 20)),
        23 => Ok((Packet::Prot23 { source_ip, dest_ip }, 20)),
        221 => Ok((Packet::Prot221 { source_ip, dest_ip }, 20)),
        79 => Ok((Packet::Prot79 { source_ip, dest_ip }, 20)),
        137 => Ok((Packet::Prot137 { source_ip, dest_ip }, 20)),
        148 => Ok((Packet::Prot148 { source_ip, dest_ip }, 20)),
        87 => Ok((Packet::Prot87 { source_ip, dest_ip }, 20)),
        216 => Ok((Packet::Prot216 { source_ip, dest_ip }, 20)),
        99 => Ok((Packet::Prot99 { source_ip, dest_ip }, 20)),
        153 => Ok((Packet::Prot153 { source_ip, dest_ip }, 20)),
        62 => Ok((Packet::Prot62 { source_ip, dest_ip }, 20)),
        85 => Ok((Packet::Prot85 { source_ip, dest_ip }, 20)),
        58 => Ok((Packet::Prot58 { source_ip, dest_ip }, 20)),
        78 => Ok((Packet::Prot78 { source_ip, dest_ip }, 20)),
        5 => Ok((Packet::Prot5 { source_ip, dest_ip }, 20)),
        4 => Ok((Packet::Prot4 { source_ip, dest_ip }, 20)),
        91 => Ok((Packet::Prot91 { source_ip, dest_ip }, 20)),
        229 => Ok((Packet::Prot229 { source_ip, dest_ip }, 20)),
        49 => Ok((Packet::Prot49 { source_ip, dest_ip }, 20)),
        193 => Ok((Packet::Prot193 { source_ip, dest_ip }, 20)),
        19 => Ok((Packet::Prot19 { source_ip, dest_ip }, 20)),
        129 => Ok((Packet::Prot129 { source_ip, dest_ip }, 20)),
        189 => Ok((Packet::Prot189 { source_ip, dest_ip }, 20)),
        195 => Ok((Packet::Prot195 { source_ip, dest_ip }, 20)),
        43 => Ok((Packet::Prot43 { source_ip, dest_ip }, 20)),
        210 => Ok((Packet::Prot210 { source_ip, dest_ip }, 20)),
        95 => Ok((Packet::Prot95 { source_ip, dest_ip }, 20)),
        133 => Ok((Packet::Prot133 { source_ip, dest_ip }, 20)),
        123 => Ok((Packet::Prot123 { source_ip, dest_ip }, 20)),
        152 => Ok((Packet::Prot152 { source_ip, dest_ip }, 20)),
        102 => Ok((Packet::Prot102 { source_ip, dest_ip }, 20)),
        245 => Ok((Packet::Prot245 { source_ip, dest_ip }, 20)),
        147 => Ok((Packet::Prot147 { source_ip, dest_ip }, 20)),
        113 => Ok((Packet::Prot113 { source_ip, dest_ip }, 20)),
        122 => Ok((Packet::Prot122 { source_ip, dest_ip }, 20)),
        253 => Ok((Packet::Prot253 { source_ip, dest_ip }, 20)),
        143 => Ok((Packet::Prot143 { source_ip, dest_ip }, 20)),
		215 => Ok((Packet::Prot215 { source_ip, dest_ip }, 20)),
		233 => Ok((Packet::Prot233 { source_ip, dest_ip }, 20)),
		235 => Ok((Packet::Prot235 { source_ip, dest_ip }, 20)),
		150 => Ok((Packet::Prot150 { source_ip, dest_ip }, 20)),
		64 => Ok((Packet::Prot64 { source_ip, dest_ip }, 20)),
		224 => Ok((Packet::Prot224 { source_ip, dest_ip }, 20)),
		125 => Ok((Packet::Prot125 { source_ip, dest_ip }, 20)),
		206 => Ok((Packet::Prot206 { source_ip, dest_ip }, 20)),
		200 => Ok((Packet::Prot200 { source_ip, dest_ip }, 20)),
		61 => Ok((Packet::Prot61 { source_ip, dest_ip }, 20)),
		175 => Ok((Packet::Prot175 { source_ip, dest_ip }, 20)),
		171 => Ok((Packet::Prot171 { source_ip, dest_ip }, 20)),
		108 => Ok((Packet::Prot108 { source_ip, dest_ip }, 20)),
		252 => Ok((Packet::Prot252 { source_ip, dest_ip }, 20)),
		254 => Ok((Packet::Prot254 { source_ip, dest_ip }, 20)),
		234 => Ok((Packet::Prot234 { source_ip, dest_ip }, 20)),
		205 => Ok((Packet::Prot205 { source_ip, dest_ip }, 20)),
		71 => Ok((Packet::Prot71 { source_ip, dest_ip }, 20)),
		241 => Ok((Packet::Prot241 { source_ip, dest_ip }, 20)),
		30 => Ok((Packet::Prot30 { source_ip, dest_ip }, 20)),
		214 => Ok((Packet::Prot214 { source_ip, dest_ip }, 20)),
		188 => Ok((Packet::Prot188 { source_ip, dest_ip }, 20)),
		209 => Ok((Packet::Prot209 { source_ip, dest_ip }, 20)),
		141 => Ok((Packet::Prot141 { source_ip, dest_ip }, 20)),
		103 => Ok((Packet::Prot103 { source_ip, dest_ip }, 20)),
		15 => Ok((Packet::Prot15 { source_ip, dest_ip }, 20)),
		63 => Ok((Packet::Prot63 { source_ip, dest_ip }, 20)),
		48 => Ok((Packet::Prot48 { source_ip, dest_ip }, 20)),
		35 => Ok((Packet::Prot35 { source_ip, dest_ip }, 20)),
		246 => Ok((Packet::Prot246 { source_ip, dest_ip }, 20)),
		98 => Ok((Packet::Prot98 { source_ip, dest_ip }, 20)),
		118 => Ok((Packet::Prot118 { source_ip, dest_ip }, 20)),
		66 => Ok((Packet::Prot66 { source_ip, dest_ip }, 20)),
		212 => Ok((Packet::Prot212 { source_ip, dest_ip }, 20)),
		196 => Ok((Packet::Prot196 { source_ip, dest_ip }, 20)),
		24 => Ok((Packet::Prot24 { source_ip, dest_ip }, 20)),
		105 => Ok((Packet::Prot105 { source_ip, dest_ip }, 20)),
		97 => Ok((Packet::Prot97 { source_ip, dest_ip }, 20)),
		92 => Ok((Packet::Prot92 { source_ip, dest_ip }, 20)),
		140 => Ok((Packet::Prot140 { source_ip, dest_ip }, 20)),
		163 => Ok((Packet::Prot163 { source_ip, dest_ip }, 20)),
		72 => Ok((Packet::Prot72 { source_ip, dest_ip }, 20)),
		199 => Ok((Packet::Prot199 { source_ip, dest_ip }, 20)),
		255 => Ok((Packet::Prot255 { source_ip, dest_ip }, 20)),
		131 => Ok((Packet::Prot131 { source_ip, dest_ip }, 20)),
		172 => Ok((Packet::Prot172 { source_ip, dest_ip }, 20)),
		109 => Ok((Packet::Prot109 { source_ip, dest_ip }, 20)),
		82 => Ok((Packet::Prot82 { source_ip, dest_ip }, 20)),
		184 => Ok((Packet::Prot184 { source_ip, dest_ip }, 20)),
		77 => Ok((Packet::Prot77 { source_ip, dest_ip }, 20)),
		96 => Ok((Packet::Prot96 { source_ip, dest_ip }, 20)),
		180 => Ok((Packet::Prot180 { source_ip, dest_ip }, 20)),
		231 => Ok((Packet::Prot231 { source_ip, dest_ip }, 20)),
		181 => Ok((Packet::Prot181 { source_ip, dest_ip }, 20)),
		218 => Ok((Packet::Prot218 { source_ip, dest_ip }, 20)),
		89 => Ok((Packet::Prot89 { source_ip, dest_ip }, 20)),
		134 => Ok((Packet::Prot134 { source_ip, dest_ip }, 20)),
		60 => Ok((Packet::Prot60 { source_ip, dest_ip }, 20)),
		244 => Ok((Packet::Prot244 { source_ip, dest_ip }, 20)),
		112 => Ok((Packet::Prot112 { source_ip, dest_ip }, 20)),
		116 => Ok((Packet::Prot116 { source_ip, dest_ip }, 20)),
		39 => Ok((Packet::Prot39 { source_ip, dest_ip }, 20)),
		222 => Ok((Packet::Prot222 { source_ip, dest_ip }, 20)),
		236 => Ok((Packet::Prot236 { source_ip, dest_ip }, 20)),
		251 => Ok((Packet::Prot251 { source_ip, dest_ip }, 20)),
		93 => Ok((Packet::Prot93 { source_ip, dest_ip }, 20)),
		70 => Ok((Packet::Prot70 { source_ip, dest_ip }, 20)),
		127=> Ok((Packet::Prot127 { source_ip, dest_ip }, 20)),
		126 => Ok((Packet::Prot126 { source_ip, dest_ip }, 20)),
		86 => Ok((Packet::Prot86 { source_ip, dest_ip }, 20)),
		12 => Ok((Packet::Prot12 { source_ip, dest_ip }, 20)),
		192 => Ok((Packet::Prot192 { source_ip, dest_ip }, 20)),
		115 => Ok((Packet::Prot115 { source_ip, dest_ip }, 20)),
		223 => Ok((Packet::Prot223 { source_ip, dest_ip }, 20)),
		232 => Ok((Packet::Prot232 { source_ip, dest_ip }, 20)),
		65 => Ok((Packet::Prot65 { source_ip, dest_ip }, 20)),
		204 => Ok((Packet::Prot204 { source_ip, dest_ip }, 20)),
		201 => Ok((Packet::Prot201 { source_ip, dest_ip }, 20)),
		34 => Ok((Packet::Prot34 { source_ip, dest_ip }, 20)),
		9 => Ok((Packet::Prot9 { source_ip, dest_ip }, 20)),
		29 => Ok((Packet::Prot29 { source_ip, dest_ip }, 20)),
		176 => Ok((Packet::Prot176 { source_ip, dest_ip }, 20)),
		226 => Ok((Packet::Prot226 { source_ip, dest_ip }, 20)),
		104 => Ok((Packet::Prot104 { source_ip, dest_ip }, 20)),
		187 => Ok((Packet::Prot187 { source_ip, dest_ip }, 20)),
		185 => Ok((Packet::Prot185 { source_ip, dest_ip }, 20)),
		132 => Ok((Packet::Prot132 { source_ip, dest_ip }, 20)),
		160 => Ok((Packet::Prot160 { source_ip, dest_ip }, 20)),
		45 => Ok((Packet::Prot45 { source_ip, dest_ip }, 20)),
		80 => Ok((Packet::Prot80 { source_ip, dest_ip }, 20)),
		145 => Ok((Packet::Prot145 { source_ip, dest_ip }, 20)),
		3 => Ok((Packet::Prot3 { source_ip, dest_ip }, 20)),
		16 => Ok((Packet::Prot16 { source_ip, dest_ip }, 20)),
		10 => Ok((Packet::Prot10 { source_ip, dest_ip }, 20)),
		100 => Ok((Packet::Prot100 { source_ip, dest_ip }, 20)),
		247 => Ok((Packet::Prot247 { source_ip, dest_ip }, 20)),
		225 => Ok((Packet::Prot225 { source_ip, dest_ip }, 20)),
		18 => Ok((Packet::Prot18 { source_ip, dest_ip }, 20)),
		54 => Ok((Packet::Prot54 { source_ip, dest_ip }, 20)),
		52 => Ok((Packet::Prot52 { source_ip, dest_ip }, 20)),
		38 => Ok((Packet::Prot38 { source_ip, dest_ip }, 20)),
		55 => Ok((Packet::Prot55 { source_ip, dest_ip }, 20)),
		33 => Ok((Packet::Prot33 { source_ip, dest_ip }, 20)),
		162 => Ok((Packet::Prot162 { source_ip, dest_ip }, 20)),
		186 => Ok((Packet::Prot186 { source_ip, dest_ip }, 20)),
		88 => Ok((Packet::Prot88 { source_ip, dest_ip }, 20)),
		76 => Ok((Packet::Prot76 { source_ip, dest_ip }, 20)),
		20 => Ok((Packet::Prot20 { source_ip, dest_ip }, 20)),
		27 => Ok((Packet::Prot27 { source_ip, dest_ip }, 20)),
		170 => Ok((Packet::Prot170 { source_ip, dest_ip }, 20)),
		220 => Ok((Packet::Prot220 { source_ip, dest_ip }, 20)),
		53 => Ok((Packet::Prot53 { source_ip, dest_ip }, 20)),
		74 => Ok((Packet::Prot74 { source_ip, dest_ip }, 20)),
		50 => Ok((Packet::Prot50 { source_ip, dest_ip }, 20)),
		136 => Ok((Packet::Prot136 { source_ip, dest_ip }, 20)),
		36 => Ok((Packet::Prot36 { source_ip, dest_ip }, 20)),
		22 => Ok((Packet::Prot22 { source_ip, dest_ip }, 20)),
		119 => Ok((Packet::Prot119 { source_ip, dest_ip }, 20)),
		164 => Ok((Packet::Prot164 { source_ip, dest_ip }, 20)),
		120 => Ok((Packet::Prot120 { source_ip, dest_ip }, 20)),
		250 => Ok((Packet::Prot250 { source_ip, dest_ip }, 20)),
		142 => Ok((Packet::Prot142 { source_ip, dest_ip }, 20)),
		8 => Ok((Packet::Prot8 { source_ip, dest_ip }, 20)),
		130 => Ok((Packet::Prot130 { source_ip, dest_ip }, 20)),
		154 => Ok((Packet::Prot154 { source_ip, dest_ip }, 20)),
		197 => Ok((Packet::Prot197 { source_ip, dest_ip }, 20)),
		211 => Ok((Packet::Prot211 { source_ip, dest_ip }, 20)),
		158 => Ok((Packet::Prot158 { source_ip, dest_ip }, 20)),
		227 => Ok((Packet::Prot227 { source_ip, dest_ip }, 20)),
		194 => Ok((Packet::Prot194 { source_ip, dest_ip }, 20)),
		69 => Ok((Packet::Prot69 { source_ip, dest_ip }, 20)),
		73 => Ok((Packet::Prot73 { source_ip, dest_ip }, 20)),
		149 => Ok((Packet::Prot149 { source_ip, dest_ip }, 20)),
		57 => Ok((Packet::Prot57 { source_ip, dest_ip }, 20)),
		169 => Ok((Packet::Prot169 { source_ip, dest_ip }, 20)),
		56 => Ok((Packet::Prot56 { source_ip, dest_ip }, 20)),
		51 => Ok((Packet::Prot51 { source_ip, dest_ip }, 20)),
		101 => Ok((Packet::Prot101 { source_ip, dest_ip }, 20)),
		198 => Ok((Packet::Prot198 { source_ip, dest_ip }, 20)),
		165 => Ok((Packet::Prot165 { source_ip, dest_ip }, 20)),
		237 => Ok((Packet::Prot237 { source_ip, dest_ip }, 20)),
		11 => Ok((Packet::Prot11 { source_ip, dest_ip }, 20)),
		155 => Ok((Packet::Prot155 { source_ip, dest_ip }, 20)),
		157 => Ok((Packet::Prot157 { source_ip, dest_ip }, 20)),
		67 => Ok((Packet::Prot67 { source_ip, dest_ip }, 20)),
		249 => Ok((Packet::Prot249 { source_ip, dest_ip }, 20)),
		81 => Ok((Packet::Prot81 { source_ip, dest_ip }, 20)),
		114 => Ok((Packet::Prot114 { source_ip, dest_ip }, 20)),
		228 => Ok((Packet::Prot228 { source_ip, dest_ip }, 20)),
		243 => Ok((Packet::Prot243 { source_ip, dest_ip }, 20)),
		94 => Ok((Packet::Prot94 { source_ip, dest_ip }, 20)),
		240 => Ok((Packet::Prot240 { source_ip, dest_ip }, 20)),
		139 => Ok((Packet::Prot139 { source_ip, dest_ip }, 20)),
		128 => Ok((Packet::Prot128 { source_ip, dest_ip }, 20)),
		31 => Ok((Packet::Prot31 { source_ip, dest_ip }, 20)),
		238 => Ok((Packet::Prot238 { source_ip, dest_ip }, 20)),
		68 => Ok((Packet::Prot68 { source_ip, dest_ip }, 20)),
		44 => Ok((Packet::Prot44 { source_ip, dest_ip }, 20)),
		135 => Ok((Packet::Prot135 { source_ip, dest_ip }, 20)),
		156 => Ok((Packet::Prot156 { source_ip, dest_ip }, 20)),


		
		
		
		_ => {
            println!("Unknown protocol encountered: {}", protocol);
            Err(protocol)
        },
    }
}



fn log_packet_info(packet: &Packet) {
    match packet {
        Packet::Hopopt { source_ip, dest_ip } => {
            println!(
                "HOPOPT Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Tcp { source_ip, dest_ip, source_port, dest_port } => {
            println!(
                "TCP Packet: Source IP: {}, Destination IP: {}, Source Port: {}, Destination Port: {}",
                source_ip, dest_ip, source_port, dest_port
            );
        }
        Packet::Udp { source_ip, dest_ip, source_port, dest_port } => {
            println!(
                "UDP Packet: Source IP: {}, Destination IP: {}, Source Port: {}, Destination Port: {}",
                source_ip, dest_ip, source_port, dest_port
            );
        }
        Packet::Icmp { source_ip, dest_ip } => {
            println!(
                "ICMP Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Igmp { source_ip, dest_ip } => {
            println!(
                "IGMP Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Cbt { source_ip, dest_ip } => {
            println!(
                "CBT Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Ipv6 { source_ip, dest_ip } => {
            println!(
                "IPv6 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		
		Packet::Prot108 { source_ip, dest_ip } => {
            println!(
                "Protocol 108 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot135 { source_ip, dest_ip } => {
            println!(
                "Protocol 135 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot188 { source_ip, dest_ip } => {
            println!(
                "Protocol 188 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot209 { source_ip, dest_ip } => {
            println!(
                "Protocol 209 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot252 { source_ip, dest_ip } => {
            println!(
                "Protocol 252 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot254 { source_ip, dest_ip } => {
            println!(
                "Protocol 254 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot214 { source_ip, dest_ip } => {
            println!(
                "Protocol 214 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot30 { source_ip, dest_ip } => {
            println!(
                "Protocol 30 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot241 { source_ip, dest_ip } => {
            println!(
                "Protocol 241 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot234 { source_ip, dest_ip } => {
            println!(
                "Protocol 234 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot71 { source_ip, dest_ip } => {
            println!(
                "Protocol 71 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot205 { source_ip, dest_ip } => {
            println!(
                "Protocol 205 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Ah { source_ip, dest_ip } => {
            println!(
                "AH Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Special { source_ip, dest_ip } => {
            println!(
                "Special Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot61 { source_ip, dest_ip } => {
            println!(
                "Protocol 61 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot175 { source_ip, dest_ip } => {
            println!(
                "Protocol 175 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot171 { source_ip, dest_ip } => {
            println!(
                "Protocol 171 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot215 { source_ip, dest_ip } => {
            println!(
                "Protocol 215 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot233 { source_ip, dest_ip } => {
            println!(
                "Protocol 233 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot235 { source_ip, dest_ip } => {
            println!(
                "Protocol 235 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot150 { source_ip, dest_ip } => {
            println!(
                "Protocol 150 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot224 { source_ip, dest_ip } => {
            println!(
                "Protocol 224 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot64 { source_ip, dest_ip } => {
            println!(
                "Protocol 64 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot125 { source_ip, dest_ip } => {
            println!(
                "Protocol 125 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot206 { source_ip, dest_ip } => {
            println!(
                "Protocol 206 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot200 { source_ip, dest_ip } => {
            println!(
                "Protocol 200 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot106 { source_ip, dest_ip } => {
            println!(
                "Protocol 106 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot110 { source_ip, dest_ip } => {
            println!(
                "Protocol 110 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot117 { source_ip, dest_ip } => {
            println!(
                "Protocol 117 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot174 { source_ip, dest_ip } => {
            println!(
                "Protocol 174 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot13 { source_ip, dest_ip } => {
            println!(
                "Protocol 13 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot124 { source_ip, dest_ip } => {
            println!(
                "Protocol 124 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot178 { source_ip, dest_ip } => {
            println!(
                "Protocol 178 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot83 { source_ip, dest_ip } => {
            println!(
                "Protocol 83 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot14 { source_ip, dest_ip } => {
            println!(
                "Protocol 14 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot84 { source_ip, dest_ip } => {
            println!(
                "Protocol 84 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot111 { source_ip, dest_ip } => {
            println!(
                "Protocol 111 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot46 { source_ip, dest_ip } => {
            println!(
                "Protocol 46 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot179 { source_ip, dest_ip } => {
            println!(
                "Protocol 179 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot230 { source_ip, dest_ip } => {
            println!(
                "Protocol 230 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot219 { source_ip, dest_ip } => {
            println!(
                "Protocol 219 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot47 { source_ip, dest_ip } => {
            println!(
                "Protocol 47 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot28 { source_ip, dest_ip } => {
            println!(
                "Protocol 28 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot203 { source_ip, dest_ip } => {
            println!(
                "Protocol 203 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot23 { source_ip, dest_ip } => {
            println!(
                "Protocol 23 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot221 { source_ip, dest_ip } => {
            println!(
                "Protocol 221 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot79 { source_ip, dest_ip } => {
            println!(
                "Protocol 79 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot137 { source_ip, dest_ip } => {
            println!(
                "Protocol 137 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot148 { source_ip, dest_ip } => {
            println!(
                "Protocol 148 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot87 { source_ip, dest_ip } => {
            println!(
                "Protocol 87 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot216 { source_ip, dest_ip } => {
            println!(
                "Protocol 216 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot99 { source_ip, dest_ip } => {
            println!(
                "Protocol 99 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot153 { source_ip, dest_ip } => {
            println!(
                "Protocol 153 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot62 { source_ip, dest_ip } => {
            println!(
                "Protocol 62 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot85 { source_ip, dest_ip } => {
            println!(
                "Protocol 85 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot58 { source_ip, dest_ip } => {
            println!(
                "Protocol 58 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot78 { source_ip, dest_ip } => {
            println!(
                "Protocol 78 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot5 { source_ip, dest_ip } => {
            println!(
                "Protocol 5 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot4 { source_ip, dest_ip } => {
            println!(
                "Protocol 4 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot91 { source_ip, dest_ip } => {
            println!(
                "Protocol 91 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
        Packet::Prot229 { source_ip, dest_ip } => {
            println!(
                "Protocol 229 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot95 { source_ip, dest_ip } => {
            println!(
                "Protocol 91 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot210 { source_ip, dest_ip } => {
            println!(
                "Protocol 210 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot43 { source_ip, dest_ip } => {
            println!(
                "Protocol 43 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot195 { source_ip, dest_ip } => {
            println!(
                "Protocol 195 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot189 { source_ip, dest_ip } => {
            println!(
                "Protocol 189 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot129 { source_ip, dest_ip } => {
            println!(
                "Protocol 129 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot49 { source_ip, dest_ip } => {
            println!(
                "Protocol 49 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot152 { source_ip, dest_ip } => {
            println!(
                "Protocol 152 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot123 { source_ip, dest_ip } => {
            println!(
                "Protocol 123 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot245 { source_ip, dest_ip } => {
            println!(
                "Protocol 245 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot102 { source_ip, dest_ip } => {
            println!(
                "Protocol 102 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot133 { source_ip, dest_ip } => {
            println!(
                "Protocol 133 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot19 { source_ip, dest_ip } => {
            println!(
                "Protocol 19 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot122 { source_ip, dest_ip } => {
            println!(
                "Protocol 122 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot113 { source_ip, dest_ip } => {
            println!(
                "Protocol 113 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot143 { source_ip, dest_ip } => {
            println!(
                "Protocol 143 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot147 { source_ip, dest_ip } => {
            println!(
                "Protocol 147 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot44 { source_ip, dest_ip } => {
            println!(
                "Protocol 44 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot68 { source_ip, dest_ip } => {
            println!(
                "Protocol 68 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot238 { source_ip, dest_ip } => {
            println!(
                "Protocol 238 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot31 { source_ip, dest_ip } => {
            println!(
                "Protocol 31 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot128 { source_ip, dest_ip } => {
            println!(
                "Protocol 128 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot139 { source_ip, dest_ip } => {
            println!(
                "Protocol 139 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot240 { source_ip, dest_ip } => {
            println!(
                "Protocol 240 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot94 { source_ip, dest_ip } => {
            println!(
                "Protocol 94 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot243 { source_ip, dest_ip } => {
            println!(
                "Protocol 243 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot228 { source_ip, dest_ip } => {
            println!(
                "Protocol 228 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot114 { source_ip, dest_ip } => {
            println!(
                "Protocol 114 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot81 { source_ip, dest_ip } => {
            println!(
                "Protocol 81 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot249 { source_ip, dest_ip } => {
            println!(
                "Protocol 249 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot67 { source_ip, dest_ip } => {
            println!(
                "Protocol 67 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot157 { source_ip, dest_ip } => {
            println!(
                "Protocol 157 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot155 { source_ip, dest_ip } => {
            println!(
                "Protocol 155 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot11 { source_ip, dest_ip } => {
            println!(
                "Protocol 11 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot237 { source_ip, dest_ip } => {
            println!(
                "Protocol 237 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot165 { source_ip, dest_ip } => {
            println!(
                "Protocol 165 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot198 { source_ip, dest_ip } => {
            println!(
                "Protocol 198 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot101 { source_ip, dest_ip } => {
            println!(
                "Protocol 101 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot51 { source_ip, dest_ip } => {
            println!(
                "Protocol 51 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot56 { source_ip, dest_ip } => {
            println!(
                "Protocol 56 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot169 { source_ip, dest_ip } => {
            println!(
                "Protocol 169 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot57 { source_ip, dest_ip } => {
            println!(
                "Protocol 57 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot149 { source_ip, dest_ip } => {
            println!(
                "Protocol 149 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot73 { source_ip, dest_ip } => {
            println!(
                "Protocol 73 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot69 { source_ip, dest_ip } => {
            println!(
                "Protocol 69 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot194 { source_ip, dest_ip } => {
            println!(
                "Protocol 194 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot227 { source_ip, dest_ip } => {
            println!(
                "Protocol 227 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot158 { source_ip, dest_ip } => {
            println!(
                "Protocol 158 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot211 { source_ip, dest_ip } => {
            println!(
                "Protocol 211 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot197 { source_ip, dest_ip } => {
            println!(
                "Protocol 197 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot154 { source_ip, dest_ip } => {
            println!(
                "Protocol 154 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot130 { source_ip, dest_ip } => {
            println!(
                "Protocol 130 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot8 { source_ip, dest_ip } => {
            println!(
                "Protocol 8 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot142 { source_ip, dest_ip } => {
            println!(
                "Protocol 142 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot250 { source_ip, dest_ip } => {
            println!(
                "Protocol 250 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot120 { source_ip, dest_ip } => {
            println!(
                "Protocol 120 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot164 { source_ip, dest_ip } => {
            println!(
                "Protocol 164 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot119 { source_ip, dest_ip } => {
            println!(
                "Protocol 119 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot22 { source_ip, dest_ip } => {
            println!(
                "Protocol 22 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot36 { source_ip, dest_ip } => {
            println!(
                "Protocol 36 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot136 { source_ip, dest_ip } => {
            println!(
                "Protocol 136 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot50 { source_ip, dest_ip } => {
            println!(
                "Protocol 50 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot74 { source_ip, dest_ip } => {
            println!(
                "Protocol 74 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot53 { source_ip, dest_ip } => {
            println!(
                "Protocol 53 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot220 { source_ip, dest_ip } => {
            println!(
                "Protocol 220 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot170 { source_ip, dest_ip } => {
            println!(
                "Protocol 170 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot27 { source_ip, dest_ip } => {
            println!(
                "Protocol 27 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot20 { source_ip, dest_ip } => {
            println!(
                "Protocol 20 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot76 { source_ip, dest_ip } => {
            println!(
                "Protocol 76 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot88 { source_ip, dest_ip } => {
            println!(
                "Protocol 88 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot186 { source_ip, dest_ip } => {
            println!(
                "Protocol 186 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot162 { source_ip, dest_ip } => {
            println!(
                "Protocol 162 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot33 { source_ip, dest_ip } => {
            println!(
                "Protocol 33 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot55 { source_ip, dest_ip } => {
            println!(
                "Protocol 55 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot38 { source_ip, dest_ip } => {
            println!(
                "Protocol 38 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot52 { source_ip, dest_ip } => {
            println!(
                "Protocol 52 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot54 { source_ip, dest_ip } => {
            println!(
                "Protocol 54 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot18 { source_ip, dest_ip } => {
            println!(
                "Protocol 18 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot225 { source_ip, dest_ip } => {
            println!(
                "Protocol 225 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot247 { source_ip, dest_ip } => {
            println!(
                "Protocol 247 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot100 { source_ip, dest_ip } => {
            println!(
                "Protocol 100 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot10 { source_ip, dest_ip } => {
            println!(
                "Protocol 10 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot16 { source_ip, dest_ip } => {
            println!(
                "Protocol 16 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot3 { source_ip, dest_ip } => {
            println!(
                "Protocol 3 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot145 { source_ip, dest_ip } => {
            println!(
                "Protocol 145 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot80 { source_ip, dest_ip } => {
            println!(
                "Protocol 80 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot45 { source_ip, dest_ip } => {
            println!(
                "Protocol 45 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot160 { source_ip, dest_ip } => {
            println!(
                "Protocol 160 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot132 { source_ip, dest_ip } => {
            println!(
                "Protocol 132 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot185 { source_ip, dest_ip } => {
            println!(
                "Protocol 185 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot187 { source_ip, dest_ip } => {
            println!(
                "Protocol 187 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot104 { source_ip, dest_ip } => {
            println!(
                "Protocol 104 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot226 { source_ip, dest_ip } => {
            println!(
                "Protocol 226 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot176 { source_ip, dest_ip } => {
            println!(
                "Protocol 176 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot29 { source_ip, dest_ip } => {
            println!(
                "Protocol 29 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot9 { source_ip, dest_ip } => {
            println!(
                "Protocol 9 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot34 { source_ip, dest_ip } => {
            println!(
                "Protocol 34 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot204 { source_ip, dest_ip } => {
            println!(
                "Protocol 204 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot65 { source_ip, dest_ip } => {
            println!(
                "Protocol 65 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot232 { source_ip, dest_ip } => {
            println!(
                "Protocol 232 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot223 { source_ip, dest_ip } => {
            println!(
                "Protocol 223 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot115 { source_ip, dest_ip } => {
            println!(
                "Protocol 115 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot192 { source_ip, dest_ip } => {
            println!(
                "Protocol 192 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot12 { source_ip, dest_ip } => {
            println!(
                "Protocol 12 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot86 { source_ip, dest_ip } => {
            println!(
                "Protocol 86 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot126 { source_ip, dest_ip } => {
            println!(
                "Protocol 126 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot127 { source_ip, dest_ip } => {
            println!(
                "Protocol 127 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot70 { source_ip, dest_ip } => {
            println!(
                "Protocol 70 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot93 { source_ip, dest_ip } => {
            println!(
                "Protocol 93 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot251 { source_ip, dest_ip } => {
            println!(
                "Protocol 251 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot201 { source_ip, dest_ip } => {
            println!(
                "Protocol 201 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot236 { source_ip, dest_ip } => {
            println!(
                "Protocol 236 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot222 { source_ip, dest_ip } => {
            println!(
                "Protocol 222 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot39 { source_ip, dest_ip } => {
            println!(
                "Protocol 39 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot116 { source_ip, dest_ip } => {
            println!(
                "Protocol 116 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot112 { source_ip, dest_ip } => {
            println!(
                "Protocol 112 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot244 { source_ip, dest_ip } => {
            println!(
                "Protocol 244 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot60 { source_ip, dest_ip } => {
            println!(
                "Protocol 60 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot134 { source_ip, dest_ip } => {
            println!(
                "Protocol 134 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot89 { source_ip, dest_ip } => {
            println!(
                "Protocol 89 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot218 { source_ip, dest_ip } => {
            println!(
                "Protocol 218 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot181 { source_ip, dest_ip } => {
            println!(
                "Protocol 181 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }Packet::Prot231 { source_ip, dest_ip } => {
            println!(
                "Protocol 231 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot180 { source_ip, dest_ip } => {
            println!(
                "Protocol 180 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot96 { source_ip, dest_ip } => {
            println!(
                "Protocol 96 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot77 { source_ip, dest_ip } => {
            println!(
                "Protocol 77 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot184 { source_ip, dest_ip } => {
            println!(
                "Protocol 184 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot82 { source_ip, dest_ip } => {
            println!(
                "Protocol 82 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot109 { source_ip, dest_ip } => {
            println!(
                "Protocol 109 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot172 { source_ip, dest_ip } => {
            println!(
                "Protocol 172 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot131 { source_ip, dest_ip } => {
            println!(
                "Protocol 131 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot255 { source_ip, dest_ip } => {
            println!(
                "Protocol 255 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot199 { source_ip, dest_ip } => {
            println!(
                "Protocol 199 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot72 { source_ip, dest_ip } => {
            println!(
                "Protocol 72 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot163 { source_ip, dest_ip } => {
            println!(
                "Protocol 163 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot140 { source_ip, dest_ip } => {
            println!(
                "Protocol 140 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot92 { source_ip, dest_ip } => {
            println!(
                "Protocol 92 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot97 { source_ip, dest_ip } => {
            println!(
                "Protocol 97 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot105 { source_ip, dest_ip } => {
            println!(
                "Protocol 105 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot24 { source_ip, dest_ip } => {
            println!(
                "Protocol 24 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot196 { source_ip, dest_ip } => {
            println!(
                "Protocol 196 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot212 { source_ip, dest_ip } => {
            println!(
                "Protocol 212 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot66 { source_ip, dest_ip } => {
            println!(
                "Protocol 66 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot156 { source_ip, dest_ip } => {
            println!(
                "Protocol 156 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot118 { source_ip, dest_ip } => {
            println!(
                "Protocol 118 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot98 { source_ip, dest_ip } => {
            println!(
                "Protocol 98 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot246 { source_ip, dest_ip } => {
            println!(
                "Protocol 246 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot35 { source_ip, dest_ip } => {
            println!(
                "Protocol 35 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot48 { source_ip, dest_ip } => {
            println!(
                "Protocol 48 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot63 { source_ip, dest_ip } => {
            println!(
                "Protocol 63 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot15 { source_ip, dest_ip } => {
            println!(
                "Protocol 15 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot103 { source_ip, dest_ip } => {
            println!(
                "Protocol 103 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot141 { source_ip, dest_ip } => {
            println!(
                "Protocol 141 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot253 { source_ip, dest_ip } => {
            println!(
                "Protocol 253 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
		Packet::Prot193 { source_ip, dest_ip } => {
            println!(
                "Protocol 193 Packet: Source IP: {}, Destination IP: {}",
                source_ip, dest_ip
            );
        }
				}
}
        
        


#[derive(Debug)]
enum Packet {
    Hopopt {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Tcp {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
        source_port: u16,
        dest_port: u16,
    },
    Udp {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
        source_port: u16,
        dest_port: u16,
    },
    Icmp {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Igmp {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Cbt {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Ipv6 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Ah {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Special {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot141 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot103 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot15 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot63 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot48 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot35 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot246 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot98 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot118 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot66 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot212 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot196 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot24 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot105 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot97 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot92 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot140 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot163 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot72 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot199 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot255 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot131 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot172 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot109 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot82 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot184 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot77 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot96 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot180 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot231 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot181 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot218 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot89 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot134 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot60 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot244 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot112 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot116 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot39 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot222 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot236 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot201 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot251 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot93 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot70 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot127 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot126 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot86 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot12 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot192 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot115 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot223 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot232 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot65 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot204 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot34 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot9 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot29 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot176 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot226 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot104 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot187 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot185 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot132 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot160 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot45 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot80 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot145 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot3 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot16 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot10 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot100 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot247 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot225 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot18 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot54 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot52 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot38 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot55 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot33 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot162 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot186 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot88 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot76 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot20 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot27 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot170 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot220 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot53 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot74 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot50 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot136 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot36 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot22 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot119 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot164 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot120 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot250 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot142 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot8 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot130 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot154 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot197 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot211 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot158 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot227 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot194 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot69 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot73 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot149 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot57 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot169 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot56 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot51 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot101 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot198 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot165 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot237 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot11 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot155 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot157 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot67 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot249 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot81 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot114 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot228 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot243 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot94 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot240 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot139 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot128 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot31 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot238 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot68 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot44 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot106 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot110 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot117 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot174 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot13 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot124 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot178 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot83 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot143 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot14 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot84 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot111 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot46 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot179 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot230 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot219 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot47 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot28 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot203 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot23 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot221 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot79 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot137 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot148 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot87 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot216 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot99 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot153 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot62 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot85 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot58 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot78 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot5 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot4 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot91 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot229 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot49 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot193 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot19 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
    Prot129 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot189 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot195 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot43 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot210 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot95 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot133 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot123 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot152 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot102 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot245 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot147 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot113 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot122 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot253 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot215 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot233 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot235 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot150 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot224 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot64 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot125 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot206 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot200 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot61 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot175 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot171 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot108 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot252 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot254 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot234 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot205 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot71 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot241 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot30 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot214 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot188 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot209 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot156 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
	Prot135 {
        source_ip: Ipv4Addr,
        dest_ip: Ipv4Addr,
    },
}


fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}
