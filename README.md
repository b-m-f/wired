# Wired

This software will read a `.toml` file and generate WireGuard configurations for a Server-Client network:
- Clients can talk to all servers
- Server can talk to all clients
- Clients can not talk to each other
- Servers can not talk to each other

The software is aware of previously generated configurations and will reuse old keys by default.

## How to use

`wired NAME.toml`

All configs will be inside `NAME` directory

# Config file

## [global]
|What|Type|Description|Required|
|---|---|---|---|
|network|CIDR|Network to assign IP addresses from|required|


## [servers]
|What|Type|Description|Required|
|---|---|---|---|
|persistent_keepalive|int|Peers will send a keep-alive packet every **X** seconds|optional|

## [servers.specific]
|What|Type|Description|Required|
|---|---|---|---|
|ip|1-255|IPv4 of the server inside the network|required|
|ipv6|0001-ffff|IPv6 of the server inside the network|optional|
|port|0-65536|Port on which WireGuard is listening|required|
|endpoint|IPv4 or IPv6 or Hostname|Address where this server is reachable|required|
|persistent_keepalive|int|Peers will send a keep-alive packet every **X** seconds|optional|
|dns|IPv4 or IPv6|set custom DNS server|optional|

## [clients.specific]

|What|Type|Description|Required|
|---|---|---|---|
|dns|IPv4 or IPv6|set custom DNS server|optional|

## Why TOML?

Because `cargo.toml` and [TOML is designed to map unambiguously to a hash table](https://toml.io/en/)

# IP Assignment
Done fully automatically for all clients.
Must be set for servers, which can in turn be used to create DNS records.

# Key management
All keys are autmatically created and rotated on each execution.

# Merge Order

More specific settings merge on top of less specific ones.

## Example
```
[servers]
persistent_keepalive = 25

[servers.alpha]

[servers.beta]
persistent_keepalive = 30
```

Client will send `keepalive` packet to `alpha` every 25 seconds, and every 30 seconds to `beta`.


## Rotating keys

Invoke with `--rotate-keys` or `-r`.

## Why BSD?

Because I am a Software Developer from Berlin.

## Testing
Tests are written using [bats](https://github.com/bats-core/bats-core).
The **bats** framework is included in the repository via git submodules.
These will be fetched automatically when executing the tests, but can be fetched manually with `git submodule update --recursive`.

They are executed in a containerized environment and depend on:

- [podman](https://podman.io/)
- [buildah](https://buildah.io/)


