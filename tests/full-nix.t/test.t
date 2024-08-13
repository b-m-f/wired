Make sure all config files get created
  $ cd $TESTDIR
  $ wired --config-file full.toml

Check that expected files were created
  $ ls full
  client.conf
  server.nix

Confirm that configs have the correct output
  $ cat full/client.conf
  [Interface]
  Address = 10.100.1.1
  PrivateKey = 8Fp1TVFMWY0qYufoGm6qFeJXrtzU3FodpoiCkdJfQ2k=
  DNS = 10.10.10.1
  
  [Peer]
  PublicKey = vvLcDOPrSPIflR8dJtM5Q3iqQCSCPvoyFaLrUlWoIHM=
  Endpoint = 1.1.1.1:20202
  AllowedIPs = 10.100.1.1
  PresharedKey = qPQ/T+4dHydnvk8cZXh+zBpZqOmLvaoxbC0W6c2gwtg=
  
  $ cat full/server.nix
