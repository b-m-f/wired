# Key management

You can omit all the key fields. In this case new keys will be generated.
Here are additional rules:

- Specified privatekeys/presharedkey will be kept
- Publickeys will always be regenerated from privatekeys

# Config overwrite
Existing configuration files will not be overwritten. Call with `--force` to do it anyway

# Auto encryption
If you choose `nix` as an output format you can optionally enable auto encryption of your secrets.
The following encryption processes are currently supported:

## Colmena and pass
Make sure that your password-store is initialized and set the [encryption option](./configuration.md) to `colmena:pass`.

After running `wired` you can simply import the generated `nix` files in your main configuration and deploy with `colmena`.

### Fallback
Privatekeys and Presharedkeys will still be written to the output directory in case something goes wrong and you need to do something manually. Make sure not to blindly check in that folder.
