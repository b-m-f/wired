# Wired

Wired is a WireGuard configuration generator.
It parses a TOML file and generates configurations of different formats.


## Getting started

- Create minimal configuration
- run wired, passing it the created configuration
- Config files will now be in a directory named after the network name that was
  provided
- Backup the generated statefile to recreated the exact same configs in the
  future.

## Documentation

- [Configuration File](./docs/configuration.md)
- [Statefile](./docs/statefile.md)
- [Behaviour](./docs/behaviour.md)

### Key management

You can omit all the key fields. In this case new keys will be generated.
Here are additional rules:

- Specified privatekeys will be kept
- Publickeys will always be regenerated from privatekeys
- Presharedkeys are always set to the globally defined one

