# Dependencies

- `wg` must be installed for key generation
  - see https://www.wireguard.com/install/

# Config File

The config file is written in [TOML](https://toml.io/en/) and has different sections:

## Network

The network section defines settings that are outside of the scope of single
peers.

- CIDRv4: string
- type: string
- presharedkey: optional string
- name: string


### name
Name of the network. This will be used to create a folder where all generated config files will be stored in.

### CIDR

The CIDR settings will be used to generate IPs in case they are ommited from
the peer configurations.

In the nix configuration routes will be created to reach the whole CIDR via the WireGuard interface.

> Only IPv4 is currently supported

### type
The type determines the connectivity of the network.
Per default it will be set to 'web'.

#### web

All clients can connect to all servers.
Client can not connect to each other.
Server can not connect to each other.

### presharedkey

Additional authentication mechanism via a Shared Key on all the Peers.

## Servers

The servers support the following fields:
- ip: optional string
- endpoint: string
- listenport: optional int
- persistentkeepalive: optional int
- allowedips: optional [string]
- fwmark: optional  string
- privatekey: optional string
- output: optional string

### PersistentKeepalive
The `persistentkeepalive` value will be added to all the client configurations.
Meaning that all clients will keep their connection to this server alive, by
pinging every `n` seconds. `n` being the specified value.

### AllowedIPs

The default depends on the chosen network type.
Any values specified will be added in addition to the defaults.

#### web
All clients IPs are added.

### Output
The output defaults to `wg-quick`.

### Nix

This will output a systemd-networkd configuration to be used with nix.
The keyfile is separated out, and should be integrated via your secret manager
of choice (agenix is an example).

Make sure to make the file readable by `systemd-network` on the target host.

## Clients
Clients support the following fields:

- ip: optional string
- privatekey: optional string
- listenport: optional int
- output: optional string
- dns: optional string

### Output
The output defaults to `wg-quick`.

Other available formats are:
- nix
- qr

## Example
```
[servers]
<!-- TODO: add fields -->
[clients]
```

## IP ranges
At the moment only IPv4 is supported.
