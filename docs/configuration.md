# Configuration
## Minimal
```toml
[network]
name = "minimal"
type = "web"
cidrv4 = "10.100.1.0/24"

[servers]
[servers.server]
ip = "10.100.1.1"
listenport = 20202
endpoint = "1.1.1.1"
persistentkeepalive = 5

[clients]
[clients.client]
ip = "10.100.1.1"
dns = "10.10.10.1"
```

## Full
```toml
[network]
presharedkey = "qPQ/T+4dHydnvk8cZXh+zBpZqOmLvaoxbC0W6c2gwtg="
name = "full"
type = "web"
cidrv4 = "10.100.1.0/24"

[servers]
[servers.server]
ip = "10.100.1.1"
output = "nix"
encryption = "none"
privatekey = "MHYE0gQavBWsRvMNMOoYB/cL3YFoiiWpWAq5PjHMw0c="
listenport = 20202
endpoint = "1.1.1.1"
persistentkeepalive = 5
always-rotate-key = true

[clients]
[clients.client]
ip = "10.100.1.1"
output = "nix"
encryption = "none"
dns = "10.10.10.1"
privatekey = "8Fp1TVFMWY0qYufoGm6qFeJXrtzU3FodpoiCkdJfQ2k="
always-rotate-key = false
```

## Options
| Key | Type | Default | Description |
| --- | --- | --- | --- |
| `always-rotate-key` | boolean | `false` | If true, the keypair for the network or peer will always be recreated, even if it already exists in the statefile. |
