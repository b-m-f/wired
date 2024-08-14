  $ cd $TESTDIR
Fail when server is missing ip:
  $ wired --config-file fail-server.toml
  Error: No IP specified for server server
  [1]

Fail when client is missing ip:
  $ wired --config-file fail-client.toml
  Error: No IP specified for client client
  [1]
