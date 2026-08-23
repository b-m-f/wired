  $ cd $TESTDIR
Error when allowedips is a string:

  $ wired --config-file invalid-type.toml
  allowedips must be an array
  [1]

Error when allowedips format is invalid:

  $ wired --config-file invalid-format.toml
  Invalid allowed IP 'not_an_ip' for server
  [1]

Successful parsing of valid allowedips and correct route generation:

  $ wired --config-file valid.toml
  $ grep "Destination" wired/testnet/server.nix | awk '{$1=$1;print}'
  Destination = "10.0.0.0/24";
  Destination = "2.2.2.2/32";
  $ grep "Destination" wired/testnet/client.nix | awk '{$1=$1;print}'
  Destination = "10.0.0.0/24";
  Destination = "1.1.1.1/32";
  Destination = "0.0.0.0/0";
  $ grep "AllowedIPs" wired/testnet/server.nix | awk '{$1=$1;print}'
  AllowedIPs =["10.0.0.2" "2.2.2.2/32"];
  $ grep "AllowedIPs" wired/testnet/client.nix | awk '{$1=$1;print}'
  AllowedIPs = ["10.0.0.1" "1.1.1.1/32" "0.0.0.0/0"];
No duplicate IPs should appear in the generated nix files:

  $ wired --config-file duplicate-ip.toml
  $ grep "AllowedIPs =" wired/dupnet/server.nix
              AllowedIPs =["10.0.0.2"];
  $ grep "AllowedIPs =" wired/dupnet/client.nix
                       AllowedIPs = ["10.0.0.1"];

  $ rm -rf wired
  $ rm *.statefile
