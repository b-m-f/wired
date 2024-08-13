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
        let finished_config =
            server_config.generate_string(&client_configs, &network_config.preshared_key);
        wired::files::write_config(&server_config.path_to_config, &finished_config)
    }
    for client_config in client_configs {
        let finished_config =
            client_config.generate_string(&server_configs, &network_config.preshared_key);
        wired::files::write_config(&client_config.path_to_config, &finished_config);

        if client_config.qr {
            wired::qr::create_qr(&client_config.path_to_config, &finished_config);
        }
    }
    // TODO: generate statefile
    // TODO: add encryption via pass
}
