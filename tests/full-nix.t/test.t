Make sure all config files get created
  $ cd $TESTDIR
  $ wired --config-file full.toml

Check that expected files were created
  $ ls full
  client.key
  client.nix
  full.psk
  server.key
  server.nix

Confirm that configs have the correct output
  $ cat full/client.nix
  {
            config,
            pkgs,
            lib,
            ...
          }: {
            systemd.network = {
              enable = true;
              netdevs = {
                "10-full" = {
                  netdevConfig = {
                    Kind = "wireguard";
                    Name = "full";
                    MTUBytes = "1500";
                  };
                  wireguardConfig = {
                    #Must be readable by the systemd.network user
                    PrivateKeyFile = "UPDATE_THIS_VIA_YOUR_SECRET_MANAGER."
                  };
                  wireguardPeers = [
                    {
                     wireguardPeerConfig = {
                       PublicKey = "vvLcDOPrSPIflR8dJtM5Q3iqQCSCPvoyFaLrUlWoIHM=";
                       AllowedIPs = ["10.100.1.1"];
                       Endpoint = "1.1.1.1:20202"
                       PersistentKeepalive = 5;
                       PresharedKeyFile="UPDATE_THIS_VIA_YOUR_SECRET_MANAGER."
                     };
                  }
                  ];
                };
              };
              networks.full= {
                matchConfig.Name = "full";
                address = [
                  "10.100.1.1/32"
                ];
                DHCP = "no";
                dns = "dns = "10.10.10.1"";
                networkConfig = {
                  IPv6AcceptRA = false;
                };
                routes = [
                     {
                       routeConfig = {
                         Destination = 10.10.10.0/24;
                       };
                     }
                    ];
              };
            };
          } (no-eol)


  $ cat full/server.nix
  
  {
    config,
    pkgs,
    lib,
    ...
  }: {
    networking.firewall.allowedUDPPorts = [20202];
    networking.useNetworkd = true;
    systemd.network = {
      enable = true;
      netdevs = {
        "50-server" = {
          netdevConfig = {
            Kind = "wireguard";
            Name = "server";
            MTUBytes = "1500";
          };
          wireguardConfig = {
            PrivateKeyFile = "UPDATE_THIS_VIA_YOUR_SECRET_MANAGER.";
            ListenPort = 20202;
          };
          wireguardPeers = [
            {
            wireguardPeerConfig = {
              PublicKey = "92hH4QGMnvO0bnNMt8Bq3u17Sp0B5zPKWp7firxesGM=";
              AllowedIPs =["10.100.1.1"];
              PresharedKeyFile="UPDATE_THIS_VIA_YOUR_SECRET_MANAGER.";
            };
          }
          ];
        };
      };
      networks.server= {
        matchConfig.Name = "server";
        address = ["10.100.1.1"];
        routes = [
             {
               routeConfig = {
                 Destination = "10.100.1.0/24";
               };
             }
            ];
           };
    };
  }
  
  
               (no-eol)

Check that statefile is correct:
  $ cat full.statefile
  [network]
  presharedkey = "qPQ/T+4dHydnvk8cZXh+zBpZqOmLvaoxbC0W6c2gwtg="
  name = "full"
  type = "web"
  cidrv4 = "10.100.1.0/24"
  
  [servers]
  [Servers.server]
  ip = "10.100.1.1"
  output = "nix"
  privatekey = "MHYE0gQavBWsRvMNMOoYB/cL3YFoiiWpWAq5PjHMw0c="
  listenport = 20202
  endpoint = "1.1.1.1"
  persistentkeepalive = 5
  
  [clients]
  [Clients.client]
  ip = "10.100.1.1"
  output = "nix"
  dns = "10.10.10.1"
  privatekey = "8Fp1TVFMWY0qYufoGm6qFeJXrtzU3FodpoiCkdJfQ2k="

Cleanup
  $ rm -rf full
  $ rm *.statefile
