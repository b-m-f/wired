  $ cd $TESTDIR
Create from full config:

  $ wired --config-file full.toml

Check that all files were created
  $ ls full
  client.conf
  server.conf

Make sure that server config is correct
  $ cat full/server.conf
  
  [Interface]
  Address = 10.100.1.1
  PrivateKey = MHYE0gQavBWsRvMNMOoYB/cL3YFoiiWpWAq5PjHMw0c=
  ListenPort = 20202 
  
  [Peer]
  AllowedIPs = 10.100.1.1
  PublicKey = 92hH4QGMnvO0bnNMt8Bq3u17Sp0B5zPKWp7firxesGM=
  PresharedKey = qPQ/T+4dHydnvk8cZXh+zBpZqOmLvaoxbC0W6c2gwtg=
       
       

Make sure that client config is correct
  $ cat full/client.conf
  [Interface]
  Address = 10.100.1.1
  PrivateKey = 8Fp1TVFMWY0qYufoGm6qFeJXrtzU3FodpoiCkdJfQ2k=
  
  DNS = 10.10.10.1;
  [Peer]
  PublicKey = vvLcDOPrSPIflR8dJtM5Q3iqQCSCPvoyFaLrUlWoIHM=
  AllowedIPs = 10.100.1.1
  Endpoint = 1.1.1.1:20202
  PresharedKey = qPQ/T+4dHydnvk8cZXh+zBpZqOmLvaoxbC0W6c2gwtg=
  
