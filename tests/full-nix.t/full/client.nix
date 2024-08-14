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
        }