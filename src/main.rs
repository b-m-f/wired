mod wired;

use clap::Parser;

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

    let config = match wired::files::read_config(&args.config_file) {
        Ok(content) => content,
        // TODO: catch error here
        Err(e) => panic!("{}", e),
    };

    let config_dir = args.config_file.to_string().replace(".toml", "");

    let network_config =
        wired::network::parse_network_config(&config, args.rotate_keys, args.rotate_ips);
    let server_configs = wired::servers::parse_server_configs(config.servers, &network_config);
    let client_configs =
        wired::clients::parse_client_configs(config.clients, &server_configs, &network_config);

    // TODO: do not overwrite by default
    wired::files::remove_previous_config_dir(&config_dir);
    wired::files::create_config_dir(&config_dir);

    // TODO: make sure to write by chosen type
    for server_config in &server_configs[..] {
        match server_config.output.as_str() {
            "conf" => {
                let path_string = format!("./{}/{}.conf", network_config.name, server_config.name);
                let finished_config =
                    server_config.generate_conf(&client_configs, &network_config.preshared_key);
                wired::files::write_config(&path_string, &finished_config)
            }
            "nix" => {
                let path_string = format!("./{}/{}.nix", network_config.name, server_config.name);
                let finished_config = server_config.generate_nix(&client_configs, &network_config);
                wired::files::write_config(&path_string, &finished_config)
            }
            _ => panic!("Unknown output format for server {}", server_config.name),
        }
    }
    for client_config in client_configs {
        match client_config.output.as_str() {
            "conf" => {
                let path_string = format!("./{}/{}.conf", network_config.name, client_config.name);
                let finished_config =
                    client_config.generate_conf(&server_configs, &network_config.preshared_key);
                wired::files::write_config(&path_string, &finished_config);
            }
            "nix" => {
                let path_string = format!("./{}/{}.nix", network_config.name, client_config.name);
                let finished_config = client_config.generate_nix(&server_configs, &network_config);
                wired::files::write_config(&path_string, &finished_config);
            }
            "qr" => {
                let path_string = format!("./{}/{}.png", network_config.name, client_config.name);
                let finished_config =
                    client_config.generate_conf(&server_configs, &network_config.preshared_key);
                wired::qr::create_qr(&path_string, &finished_config);
            }
            _ => panic!("Unknown output format for server {}", client_config.name),
        }
    }
    // TODO: generate statefile
    // TODO: add encryption via pass
}
