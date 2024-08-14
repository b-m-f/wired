mod wired;

use clap::Parser;
use wired::{
    outputs::{conf, nix, qr},
    parser::Config,
};

#[derive(Parser, Debug)]
#[command(name = "wired")]
#[command(author = "b-m-f <maximilian@sodawa.com>")]
#[command(version = "1.0")]
#[command(about = "WireGuard network config generator", long_about = None)]
struct Args {
    /// Config file to parse
    #[arg(short = 'c', long)]
    config_file: String,

    /// Rotate all private keys
    #[arg(short, long, default_value_t = false)]
    rotate_keys: bool,

    /// Assign new IPs to clients
    #[arg(short = 'i', long, default_value_t = false)]
    rotate_ips: bool,
    // TODO: add config overwrite flag
}

fn main() {
    let args = Args::parse();

    let config: Config = match wired::files::read_config(&args.config_file) {
        Ok(content) => wired::parser::parse_config(content),
        // TODO: catch error here
        Err(e) => panic!("{}", e),
    };

    let config_dir = args.config_file.to_string().replace(".toml", "");

    let network_config = wired::parser::parse_network(&config);
    let server_configs = wired::parser::parse_servers(&config);
    let client_configs = wired::parser::parse_clients(&config);

    // TODO: do not overwrite by default
    wired::files::remove_previous_config_dir(&config_dir);
    wired::files::create_config_dir(&config_dir);

    // TODO: make sure to write by chosen type
    for server_config in &server_configs[..] {
        match server_config.output.as_str() {
            "conf" => {
                let path_string = format!("./{}/{}.conf", network_config.name, server_config.name);
                let finished_config =
                    conf::generate_server(&server_config, &client_configs, &network_config);
                wired::files::write_config(&path_string, &finished_config)
            }
            "nix" => {
                let config_path_string =
                    format!("./{}/{}.nix", network_config.name, server_config.name);
                let finished_config =
                    nix::generate_server(&server_config, &client_configs, &network_config);
                wired::files::write_config(&config_path_string, &finished_config);

                let privatekey_path_string =
                    format!("./{}/{}.key", network_config.name, network_config.name);
                wired::files::write_config(&privatekey_path_string, &server_config.privatekey);

                let presharedkey_path_string =
                    format!("./{}/{}.psk", network_config.name, network_config.name);
                wired::files::write_config(&presharedkey_path_string, &network_config.presharedkey)
            }
            _ => panic!("Unknown output format for server {}", server_config.name),
        }
    }
    for client_config in client_configs {
        match client_config.output.as_str() {
            "conf" => {
                let path_string = format!("./{}/{}.conf", network_config.name, client_config.name);
                let finished_config =
                    conf::generate_client(&client_config, &server_configs, &network_config);
                wired::files::write_config(&path_string, &finished_config);
            }
            "nix" => {
                let config_path_string =
                    format!("./{}/{}.nix", network_config.name, client_config.name);
                let finished_config =
                    nix::generate_client(&client_config, &server_configs, &network_config);
                wired::files::write_config(&config_path_string, &finished_config);

                let privatekey_path_string =
                    format!("./{}/{}.key", network_config.name, network_config.name);
                wired::files::write_config(&privatekey_path_string, &client_config.privatekey);

                let presharedkey_path_string =
                    format!("./{}/{}.psk", network_config.name, network_config.name);
                wired::files::write_config(&presharedkey_path_string, &network_config.presharedkey)
            }
            "qr" => {
                let path_string = format!("./{}/{}.png", network_config.name, client_config.name);
                let finished_config =
                    conf::generate_client(&client_config, &server_configs, &network_config);
                qr::create_qr(&path_string, &finished_config);
            }
            _ => panic!("Unknown output format for server {}", client_config.name),
        }
    }
    // TODO: generate statefile
    // TODO: add encryption via pass
}
