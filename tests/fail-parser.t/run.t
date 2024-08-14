  $ cd $TESTDIR
Error when missing servers:

  $ wired --config-file missing-servers.toml
  Error when parsing configuration: missing field `servers` at line 9 column 1
  [1]

Error when unknown server config is encountered:

  $ wired --config-file unknown-server.toml
  Server section error: 'what' is not a valid TOML table
  [1]

Error when missing clients:

  $ wired --config-file missing-clients.toml
  Error when parsing configuration: missing field `clients` at line 8 column 1
  [1]

Error when unknown client config is encountered:

  $ wired --config-file unknown-client.toml
  Clients section error: 'what' is not a valid TOML table
  [1]
