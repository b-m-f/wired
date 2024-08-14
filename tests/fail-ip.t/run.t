  $ cd $TESTDIR
Fail when server is missing ip:
  $ wired --config-file fail-server.toml
  Server server is missing required field 'ip'
  [1]

Fail when client is missing ip:
  $ wired --config-file fail-client.toml
  Client client is missing required field 'ip'
  [1]
