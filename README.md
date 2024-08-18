# Wired


Wired is a [WireGuard™](https://wireguard.com) configuration generator.
It parses TOML file network specifications and generates configurations of different formats.

Currently supported:

- wg-quick `.conf`
- nix `.nix`
- QR `.png`

For nix there is optional auto-encryption of keys via [pass](https://www.passwordstore.org) and [colmena](https://colmena.cli.rs/). See [Behaviour](./docs/behaviour.md)

## Getting started

- Create minimal [configuration](./docs/configuration.md)
- run wired, pass configuration with `-c`
- Config files will now be in a directory named after the network name
- Backup the generated statefile to recreated the exact same configs in the
  future.

## Documentation

- Run with `-h` to see CLI options
- [Configuration File](./docs/configuration.md)
- [Statefile](./docs/statefile.md)
- [Behaviour](./docs/behaviour.md)


## Development
A `flake.nix` with all dependencies as well as a `.envrc` for [direnv](https://direnv.net) is provided.
The program is tested with [cram tests](https://bitheap.org/cram/). Execute them with `just test`.

The code can be improved in places, so feel free to suggest refactorings in order to make extensibility easier going forward.
Just open an issue first if you would like to add new functionality and get it merged.
