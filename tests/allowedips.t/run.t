  $ cd $TESTDIR
Error when allowedips is a string:

  $ wired --config-file invalid-type.toml
  allowedips must be an array
  [1]

Error when allowedips format is invalid:

  $ wired --config-file invalid-format.toml
  Invalid allowed IP 'not_an_ip' for server
  [1]

Successful parsing of valid allowedips:

  $ wired --config-file valid.toml

No duplicate IPs should appear in the generated nix files:

  $ wired --config-file duplicate-ip.toml
  $ grep "AllowedIPs =" wired/dupnet/server.nix
              AllowedIPs =["10.0.0.2"];
  $ grep "AllowedIPs =" wired/dupnet/client.nix
                       AllowedIPs = ["10.0.0.1"];

  $ rm -rf wired
  $ rm *.statefile
