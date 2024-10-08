mod wired;

use clap::Parser;
use wired::{
    command,
    crypto::{get_preshared_key, get_private_key},
    outputs::{conf, nix, qr},
    parser::Config,
};

#[derive(Parser, Debug)]
#[command(name = "wired")]
#[command(author = "b-m-f <maximilian@sodawa.com>")]
#[command(version = "2.0")]
#[command(about = "WireGuard network config generator", long_about = None)]
struct Args {
    /// Config file to parse
    #[arg(short = 'c', long)]
    config_file: String,

    /// Rotate all private keys
    #[arg(short = 'r', long, default_value_t = false)]
    rekey: bool,

    /// Remove existing configs
    #[arg(short = 'f', long, default_value_t = false)]
    force: bool,

    /// Assign new IPs to clients
    #[arg(short = 'i', long, default_value_t = false)]
    rotate_ips: bool,
    // TODO: add config overwrite flag
}

fn main() {
    let args = Args::parse();

    let config: Config = match wired::files::read_config(&args.config_file) {
        Ok(content) => match wired::parser::parse_config(content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let mut network_config = wired::parser::parse_network(&config).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let mut server_configs = wired::parser::parse_servers(&config).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    let mut client_configs = wired::parser::parse_clients(&config).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    // Rekeying
    if args.rekey {
        network_config.presharedkey = match get_preshared_key() {
            Err(e) => {
                eprintln!("Error when trying to create new presharedkey for network: {e}");
                std::process::exit(1);
            }
            Ok(key) => key,
        };
        for server in server_configs.iter_mut() {
            server.privatekey = match get_private_key() {
                Err(e) => {
                    eprintln!("Error when trying to create new privatekey for server: {e}");
                    std::process::exit(1);
                }
                Ok(key) => key,
            };
        }
        for client in client_configs.iter_mut() {
            client.privatekey = match get_private_key() {
                Err(e) => {
                    eprintln!("Error when trying to create new privatekey for client: {e}");
                    std::process::exit(1);
                }
                Ok(key) => key,
            };
        }
    }

    let config_dir = network_config.name.clone();
    if args.force {
        let _ = wired::files::remove_previous_config_dir(&config_dir);
    }
    match wired::files::create_config_dir(&config_dir) {
        Err(e) => {
            eprintln!("Error when trying to create config dir {e}. Use --force to overwrite");
            std::process::exit(1);
        }
        Ok(_) => {}
    };

    for server_config in &server_configs[..] {
        match server_config.output.as_str() {
            "conf" => {
                let path_string = format!("./{}/{}.conf", network_config.name, server_config.name);
                let finished_config =
                    conf::generate_server(&server_config, &client_configs, &network_config);
                match wired::files::write_config(&path_string, &finished_config) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                };
            }
            "nix" => {
                let config_path_string =
                    format!("./{}/{}.nix", network_config.name, server_config.name);
                let finished_config =
                    nix::generate_server(&server_config, &client_configs, &network_config);
                match wired::files::write_config(&config_path_string, &finished_config) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                };

                let privatekey_path_string =
                    format!("./{}/{}.key", network_config.name, server_config.name);
                match wired::files::write_config(&privatekey_path_string, &server_config.privatekey)
                {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                };

                let presharedkey_path_string =
                    format!("./{}/{}.psk", network_config.name, network_config.name);
                match wired::files::write_config(
                    &presharedkey_path_string,
                    &network_config.presharedkey,
                ) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                eprintln!(
                    "Unknown output format {} for server {}",
                    server_config.output, server_config.name
                );
                std::process::exit(1);
            }
        }
    }
    for client_config in &client_configs {
        match client_config.output.as_str() {
            "conf" => {
                let path_string = format!("./{}/{}.conf", network_config.name, client_config.name);
                let finished_config =
                    conf::generate_client(&client_config, &server_configs, &network_config);
                match wired::files::write_config(&path_string, &finished_config) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                }
            }
            "nix" => {
                let config_path_string =
                    format!("./{}/{}.nix", network_config.name, client_config.name);
                let finished_config =
                    nix::generate_client(&client_config, &server_configs, &network_config);
                match wired::files::write_config(&config_path_string, &finished_config) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                }

                let privatekey_path_string =
                    format!("./{}/{}.key", network_config.name, client_config.name);
                match wired::files::write_config(&privatekey_path_string, &client_config.privatekey)
                {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                }

                let presharedkey_path_string =
                    format!("./{}/{}.psk", network_config.name, network_config.name);
                match wired::files::write_config(
                    &presharedkey_path_string,
                    &network_config.presharedkey,
                ) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                }
            }
            "qr" => {
                let path_string =
                    format!("./wired/{}/{}.png", network_config.name, client_config.name);
                let finished_config =
                    conf::generate_client(&client_config, &server_configs, &network_config);
                qr::create_qr(&path_string, &finished_config);
            }
            _ => {
                eprintln!("Unknown output format for server {}", client_config.name);
                std::process::exit(1);
            }
        }
    }
    let statefile_content =
        wired::state::create_statefile(&network_config, &server_configs, &client_configs);
    let path_string = format!("./{}.statefile", network_config.name);
    match wired::files::write_statefile(&path_string, &statefile_content) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
    // Auto encrypt secrets for nix configs if colmena:pass is choosen
    // Check README for this, can fail easily - that is why its last
    // Let use know they can do it manually using the written files
    // TODO: Document proper setup and not to check in secrets
    //
    // FIXME: maybe improve this if needed
    // Just recreate PSK each time for simplicity
    let network_name = network_config.name.clone();
    for server in server_configs {
        if server.encryption == "colmena:pass" {
            match command::encrypt_with_pass(
                format!("wired/{network_name}/{network_name}.psk"),
                network_config.presharedkey.clone(),
            ) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error when trying to auto encrypt secrets with pass: {e}");
                    std::process::exit(1);
                }
            };
            let server_name = server.name;
            match command::encrypt_with_pass(
                format!("wired/{network_name}/{server_name}.key"),
                server.privatekey.clone(),
            ) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error when trying to auto encrypt secrets with pass: {e}");
                    std::process::exit(1);
                }
            };
            eprintln!("Successfully encrypted all server secrets with pass");
        }
    }
    for client in client_configs {
        if client.encryption == "colmena:pass" {
            match command::encrypt_with_pass(
                format!("wired/{network_name}/{network_name}.psk"),
                network_config.presharedkey.clone(),
            ) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error when trying to auto encrypt secrets with pass: {e}");
                    std::process::exit(1);
                }
            };
            let client_name = client.name;
            match command::encrypt_with_pass(
                format!("wired/{network_name}/{client_name}.key"),
                client.privatekey.clone(),
            ) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error when trying to auto encrypt secrets with pass: {e}");
                    std::process::exit(1);
                }
            };
            eprintln!("Successfully encrypted all client secrets with pass");
        }
    }
}
