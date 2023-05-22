#[cfg(target_os = "windows")]
pub mod win {

    use std::process::Command;

    pub fn update_dns_settings(safe:bool, interface: &str, dns1: String, dns2: String) -> Result<(), String> {
        
        if safe {
            // Execute the netsh command to change the DNS settings of the interface
            let output = Command::new("netsh")
                .args(&[
                    "interface", "ipv4", "set", "dns", &interface, "static", &dns1, "primary",
                ])
                .output()
                .map_err(|e| format!("Failed to execute netsh command: {}", e))?;
        
            if !output.status.success() {
                let error_message = String::from_utf8_lossy(&output.stdout).to_string();
                return Err(format!("Failed to change DNS settings: {:?}", error_message));
            }
        
            // Add secondary DNS server if specified
            if !dns2.is_empty() {
                let output = Command::new("netsh")
                    .args(&[
                        "interface", "ipv4", "add", "dns", interface, &dns2, "index=2",
                    ])
                    .output()
                    .map_err(|e| format!("Failed to execute netsh command: {}", e))?;
        
                if !output.status.success() {
                    let error_message = String::from_utf8_lossy(&output.stdout).to_string();
                    return Err(format!("Failed to change DNS settings: {}", error_message));
                }
            }
        } else {

            let dhcp_status = check_dhcp_status(&interface);
            match dhcp_status {
                Ok(is_dhcp_enabled) => {
                    if is_dhcp_enabled {
                        println!("DHCP enabled for {}: {}", &interface, is_dhcp_enabled);
                        // Continue with the rest of your code
                    } else {
                        println!("DHCP not enabled for {}", &interface);
                        return Err(format!("DNS not reset for interface {} because DHCP is not enabled", &interface));
                    }
                },
                Err(error) => {
                    println!("Error checking DHCP status for {}: {:?}", &interface, error);
                    return Err(format!("Error checking DHCP status for {}: {:?}", &interface, error));
                }
            }


            let output = Command::new("netsh")
                    .args(&[
                        "interface",
                        "ipv4",
                        "set",
                        "dnsservers",
                        &format!("name=\"{}\"", &interface),
                        "source=dhcp",
                    ])
                    .output()
                    .map_err(|e| e.to_string())?;

            if !output.status.success() {
                return Err(format!("Failed to change DNS settings: {:?}", &output));
            }
        }
    
        Ok(())
    }

    fn check_dhcp_status(interface_name: &str) -> Result<bool, std::io::Error> {
        let output = Command::new("netsh")
            .args(&[
                "interface",
                "ipv4",
                "show",
                "config",
                &format!("{}", &interface_name),
            ])
            .output()?;
    
        let output_str =String::from_utf8_lossy(&output.stdout)
        .to_string();
        // Remove white spaces and join the words
        let cleaned_output = output_str.split_whitespace().collect::<Vec<&str>>().join("");
        
        Ok(cleaned_output.contains("DHCPenabled:Yes"))
    }

    
}