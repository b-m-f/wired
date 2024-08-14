
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


            