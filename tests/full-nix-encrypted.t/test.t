Make sure all config files get created
  $ cd $TESTDIR
  $ export PASSWORD_STORE_DIR=$TESTDIR/secrets
  $ pass init D607F9E45D7E32B311E9D9321E185A89832FCE97 >/dev/null
 
  $ wired --config-file full.toml
  Successfully encrypted all server secrets with pass
  Successfully encrypted all client secrets with pass

Check that expected files were created
  $ ls wired/full
  client.key
  client.nix
  full.psk
  server.key
  server.nix

Confirm that configs have the correct output
  $ cat wired/full/client.nix
  {
            config,
            pkgs,
            lib,
            ...
          }: {
            
            systemd.tmpfiles.rules = [ "d /etc/wired 0750 root systemd-network" ];
            deployment.keys."wg-full.key" = {
              keyCommand = [
                "pass"
                "wireguard/full/client.key"
              ];
  
              destDir = "/etc/wired";
              group = "systemd-network";
              permissions = "0440";
  
              uploadAt = "pre-activation";
            };
            deployment.keys."wg-full.psk" = {
              keyCommand = [
                "pass"
                "wireguard/full/psk"
              ];
  
              destDir = "/etc/wired";
              group = "systemd-network";
              permissions = "0440";
  
              uploadAt = "pre-activation";
            };
              
            systemd.network.enable = true;
            systemd.network.netdevs."10-full"= {
                "10-full" = {
                  netdevConfig = {
                    Kind = "wireguard";
                    Name = "full";
                    MTUBytes = "1500";
                  };
                  wireguardConfig = {
                    #Must be readable by the systemd.network user
                    PrivateKeyFile = "/etc/wired/wg-client.key"
                  };
                  wireguardPeers = [
                    {
                     wireguardPeerConfig = {
                       PublicKey = "vvLcDOPrSPIflR8dJtM5Q3iqQCSCPvoyFaLrUlWoIHM=";
                       AllowedIPs = ["10.100.1.1"];
                       Endpoint = "1.1.1.1:20202"
                       PersistentKeepalive = 5;
                       PresharedKeyFile="/etc/wired/wg-full.psk"
                     };
                  }
                  ];
                };
              };
          systemd.network.networks.full= {
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
          } (no-eol)


  $ cat wired/full/server.nix
  
  {
    config,
    pkgs,
    lib,
    ...
  }: {
    
            systemd.tmpfiles.rules = [ "d /etc/wired 0750 root systemd-network" ];
            deployment.keys."wg-full.key" = {
              keyCommand = [
                "pass"
                "wireguard/full/server.key"
              ];
  
              destDir = "/etc/wired";
              group = "systemd-network";
              permissions = "0440";
  
              uploadAt = "pre-activation";
            };
            deployment.keys."wg-full.psk" = {
              keyCommand = [
                "pass"
                "wireguard/full/psk"
              ];
  
              destDir = "/etc/wired";
              group = "systemd-network";
              permissions = "0440";
  
              uploadAt = "pre-activation";
            };
              
    networking.firewall.allowedUDPPorts = [20202];
    networking.useNetworkd = true;
    systemd.network.enable = true;
    systemd.network.netdevs."50-full" = {
          netdevConfig = {
            Kind = "wireguard";
            Name = "full";
            MTUBytes = "1500";
          };
          wireguardConfig = {
            PrivateKeyFile = "/etc/wired/wg-server.key";
            ListenPort = 20202;
          };
          wireguardPeers = [
            {
            wireguardPeerConfig = {
              PublicKey = "92hH4QGMnvO0bnNMt8Bq3u17Sp0B5zPKWp7firxesGM=";
              AllowedIPs =["10.100.1.1"];
              PresharedKeyFile="/etc/wired/wg-full.psk";
            };
          }
          ];
        };
    systemd.network.networks.full= {
      matchConfig.Name = "full";
      address = ["10.100.1.1/32"];
      routes = [
         {
            routeConfig = {
              Destination = "10.100.1.0/24";
            };
         }
      ];
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
  [servers.server]
  ip = "10.100.1.1"
  output = "nix"
  encryption = "colmena:pass"
  privatekey = "MHYE0gQavBWsRvMNMOoYB/cL3YFoiiWpWAq5PjHMw0c="
  listenport = 20202
  endpoint = "1.1.1.1"
  persistentkeepalive = 5
  
  [clients]
  [clients.client]
  ip = "10.100.1.1"
  output = "nix"
  encryption = "colmena:pass"
  dns = "10.10.10.1"
  privatekey = "8Fp1TVFMWY0qYufoGm6qFeJXrtzU3FodpoiCkdJfQ2k="

Check that statefile is the same as input
  $ cmp full.toml full.statefile
  $ diff full.toml full.statefile

Check that secrets where created
  $ ls secrets/wired
  full
  $ ls secrets/wired/full
  client.key.gpg
  full.psk.gpg
  server.key.gpg

Cleanup
  $ rm -rf wired
  $ rm *.statefile
  $ rm -rf secrets
