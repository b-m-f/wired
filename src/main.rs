mod wired;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wired")]
#[command(author = "b-m-f <programming@ehlers.berlin>")]
#[command(version = "1.1.0")]
#[command(about = "WireGuard network config generator", long_about = None)]
struct Args {
    /// Config file to parse
    #[arg(short, long)]
    config_file: String,

    /// Rotate all private keys
    #[arg(short, long, default_value_t = false)]
    rotate_keys: bool,

    /// Assign new IPs to clients
    #[arg(short = 'i', long, default_value_t = false)]
    rotate_ips: bool,
}

fn main() {
    let args = Args::parse();

    let config = match wired::files::read_config(&args.config_file) {
        Ok(content) => content,
        Err(e) => panic!("{}", e),
    };

    let config_dir = args.config_file.to_string().replace(".toml", "");

    let global_config =
        wired::global::parse_global_config(&config, &config_dir, args.rotate_keys, args.rotate_ips);
    let server_configs = wired::servers::parse_server_configs(config.servers, &global_config);
    let client_configs =
        wired::clients::parse_client_configs(config.clients, &server_configs, &global_config);

    wired::files::remove_previous_config_dir(&config_dir);
    wired::files::create_config_dir(&config_dir);

    for server_config in &server_configs[..] {
        let finished_config =
            server_config.generate_string(&client_configs, &global_config.preshared_key);
        wired::files::write_config(&server_config.path_to_config, &finished_config)
    }
    for client_config in client_configs {
        let finished_config =
            client_config.generate_string(&server_configs, &global_config.preshared_key);
        wired::files::write_config(&client_config.path_to_config, &finished_config);

        if client_config.qr {
            wired::qr::create_qr(&client_config.path_to_config, &finished_config);
        }
    }
}
