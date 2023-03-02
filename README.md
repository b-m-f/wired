# Wired

This software will read a `.toml` file and can currently generate WireGuard configurations for a Server-Client network:
- Clients can talk to all servers
- Servers can talk to all clients
- Clients can not talk to each other
- Servers can not talk to each other

The software is aware of previously generated configurations and will reuse old keys and IP's by default.

## How to use

```
WireGuard network config generator

Usage: wired [OPTIONS] --config-file <CONFIG_FILE>

Options:
  -c, --config-file <CONFIG_FILE>  Config file to parse
  -r, --rotate-keys                Rotate all private keys
  -i, --rotate-ips                 Assign new IPs to clients
  -h, --help                       Print help
  -V, --version                    Print version

```
All configs will be inside an output directory called `CONFIG_FILE`.
In order to keep the Keys and IPs the same over consecutive executions make sure that the `CONFIG_FILE.toml` and output directory stay together.

# Config file

## [global]
|What|Type|Description|Required|
|---|---|---|---|
|network|CIDR|Network to assign IP addresses from|required|


## [servers.NAME]
|What|Type|Description|Required|
|---|---|---|---|
|ip|1-255|IPv4 of the server inside the network|required|
|port|0-65536|Port on which WireGuard is listening|required|
|endpoint|IPv4 or IPv6 or Hostname|Address where this server is reachable|required|
|persistent_keepalive|int|Peers will send a keep-alive packet every **X** seconds|optional|
|dns|IPv4 or IPv6|set custom DNS server|optional|

## [clients.NAME]

|What|Type|Description|Required|
|---|---|---|---|
|dns|IP|set custom DNS server|optional|

## Why TOML?

Because `cargo.toml` and [TOML is designed to map unambiguously to a hash table](https://toml.io/en/)

# IP Assignment
Done fully automatically for all clients.
Must be set for servers, which can in turn be used to create DNS records.

# Key management
All keys are autmatically created and kept over consecutive exections if the output directory is in the same directory as the binary.

## Rotating keys

Invoke with `--rotate-keys` or `-r`.

# Development

## New network topologies

It is possible to support all kinds of network topologies using the base `.toml` file.
If you want to implement one just open an issue and I can help you with any questions.

### Just want me to add it?
Sure thing, open an issue with the desired topology and I will add it for 0.5 in [ETH](https://ethereum.org/en/).
If that is too expensive feel free to ask for help via [Gitcoin](https://bounties.gitcoin.co/explorer).

## Testing
Tests are written using [bats](https://github.com/bats-core/bats-core).
The **bats** framework is included in the repository via git submodules.
These will be fetched automatically when executing the tests, but can be fetched manually with `git submodule update --recursive`.

They are executed in a containerized environment and depend on:

- [podman](https://podman.io/)
- [buildah](https://buildah.io/)


