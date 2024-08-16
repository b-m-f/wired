  $ cd $TESTDIR
Fail when server is missing ip:
  $ wired --config-file fail-server.toml
  Server server is missing required field 'ip'
  [1]

Fail when client is missing ip:
  $ wired --config-file fail-client.toml
  Client client is missing required field 'ip'
  [1]

Fail when server is not in cidr:
  $ wired --config-file fail-server-not-in-cidr.toml
  IP 10.200.1.1 of server server is not in network CIDR 10.100.1.0/24
  [1]

Fail when client is not in cidr:
  $ wired --config-file fail-client-not-in-cidr.toml
  IP 10.200.1.1 of client client is not in network CIDR 10.100.1.0/24
  [1]
