// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
use utils::dns_utils::win;
use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;

#[cfg(target_os = "windows")]
fn set_dns(safe: bool, primarydns: String, secondarydns: String) -> Result<(), String> {

    let mut errors = Vec::new();

    // Get all interfaces
    let all_interfaces: Vec<NetworkInterface> = match NetworkInterface::show() {
        Ok(interfaces) => interfaces,
        Err(err) => {
            eprintln!("Failed to retrieve all interfaces: {}", err);
            return Err("Failed to retrieve all interfaces".to_string());
        }
    };

    let ipv4_interfaces: Vec<&NetworkInterface> = all_interfaces
        .iter()
        .filter(|interface| {
            interface.addr.iter().any(|addr| matches!(addr, network_interface::Addr::V4(_)))
        })
        .collect();


    match ipv4_interfaces {
        interfaces if !interfaces.is_empty() => {
            // Change the DNS settings for each connected interface
            
            for interface in interfaces {
                // for addr in &interface.addr {
                //     match addr {
                //         network_interface::Addr::V4(v4_addr) => {
                //             println!("IPv4 Address: {} {:?}", v4_addr.ip, &interface.name);
                //         }
                //         network_interface::Addr::V6(v6_addr) => {
                //             println!("IPv6 Address: {} {:?}", v6_addr.ip, &interface.name);
                //         }
                //     }
                // }
                
                if safe { 

                    if let Err(err) = win::update_dns_settings(safe,&interface.name, primarydns.clone(), secondarydns.clone()) {
                        errors.push(format!(
                            "Error changing DNS settings for {}: {:?}",
                            &interface.name, err
                        ));
                    } else {
                        println!("DNS settings changed successfully for interface: {}", &interface.name);
                    }
            
            } else {

                if let Err(err) = win::update_dns_settings(safe, &interface.name, primarydns.clone(), secondarydns.clone()) {
                    
                    if err.contains("DNS not reset for interface") {
                        println!("{}", err);
                    } else {
                    errors.push(format!(
                        "Error changing DNS settings for {}: {:?}",
                        &interface.name, err
                    ));
                }
                } else {
                    println!("DNS settings changed successfully for interface: {}", &interface.name);
                }

            }
            } 

            if errors.is_empty() {
                return Ok(());
            } else {
                return Err(errors.join("\n"));
            }
        },
        _ => {
            eprintln!("Failed to find connected interfaces");
            return Err("Failed to find connected interfaces".to_string());
        }
    }

}



#[cfg(target_os = "macos")]
fn set_dns(safe: bool, primarydns: String, secondarydns: String) -> Result<(), String> {
    use std::process::Command;

    if safe {
        let dns_servers = [primarydns, secondarydns];

        let output = Command::new("networksetup")
            .arg("-listallnetworkservices")
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("Error retrieving network services: {:?}", output);
            return;
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();

        for line in lines {
            let interface_name = line.trim();
            if interface_name.is_empty() || interface_name == "An asterisk (*) denotes that a network service is disabled." {
                continue;
            }

            let mut args = vec!["-setdnsservers", interface_name];
            args.extend_from_slice(dns_servers);

            let output = Command::new("networksetup")
                .args(&args)
                .output()
                .expect("Failed to execute command");

            if !output.status.success() {
                eprintln!("Error setting DNS servers for {}: {:?}", interface_name, output);
            } else {
                println!("DNS servers set successfully for {}.", interface_name);
            }
        }

        println!("DNS servers set successfully for all interfaces.");

    }
    else {

        let output = Command::new("networksetup")
            .arg("-listallnetworkservices")
            .output()
            .expect("Failed to execute command");

        if !output.status.success() {
            eprintln!("Error retrieving network services: {:?}", output);
            return;
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();

        for line in lines {
            let interface_name = line.trim();
            if interface_name.is_empty() || interface_name == "An asterisk (*) denotes that a network service is disabled." {
                continue;
            }

            let output = Command::new("networksetup")
                .args(&["-setdnsservers", interface_name, "Empty"])
                .output()
                .expect("Failed to execute command");

            if !output.status.success() {
                eprintln!("Error resetting DNS settings for {}: {:?}", interface_name, output);
            } else {
                println!("DNS settings reset successfully for {}.", interface_name);
            }
        }

        println!("DNS settings reset successfully for all interfaces.");
}

}

#[cfg(target_os = "linux")]
fn set_dns(safe: bool, primarydns: String, secondarydns: String) -> Result<(), String> {
  // Linux-specific code to change DNS settings
  use std::fs::File;
  use std::io::prelude::*;
  use std::io::BufWriter;
  
  if safe {
    let dns_servers = "nameserver 208.67.222.123\nnameserver 208.67.220.123"; // OpenDNS FamilyShield servers

    let path = "/etc/resolv.conf";
    let file = File::create(path).expect("Failed to open file");
    let mut writer = BufWriter::new(file);
    writer.write_all(dns_servers.as_bytes()).expect("Failed to write DNS settings");
  } else {
    let dns_servers = ""; // Empty string to reset DNS settings

    let path = "/etc/resolv.conf";
    let file = File::create(path).expect("Failed to open file");
    let mut writer = BufWriter::new(file);
    writer.write_all(dns_servers.as_bytes()).expect("Failed to write DNS settings");
  }
}

#[tauri::command]
fn set_safe_dns(primarydns: String, secondarydns: String) -> Result<(), String>{

  return set_dns(true, primarydns, secondarydns).map_err(|e| {
    format!("Failed to update DNS settings: {}", e)
});

}

#[tauri::command]
fn set_default_dns(primarydns: String, secondarydns: String) -> Result<(), String>{

  return set_dns(false, primarydns, secondarydns).map_err(|e| {
    format!("Failed to update DNS settings: {}", e)
});

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_safe_dns, set_default_dns])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
