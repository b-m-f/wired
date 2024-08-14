  $ cd $TESTDIR
Error when missing servers:

  $ wired --config-file missing-servers.toml
  Error when parsing configuration: TOML parse error at line 1, column 1
    |
  1 | [network]
    | ^
  missing field `servers`
  
  [1]

Error when unknown server config is encountered:

  $ wired --config-file unknown-server.toml
  Server section error: 'what' is not a valid TOML table
  [1]

Error when missing clients:

  $ wired --config-file missing-clients.toml
  Error when parsing configuration: TOML parse error at line 1, column 1
    |
  1 | [network]
    | ^
  missing field `clients`
  
  [1]

Error when unknown client config is encountered:

  $ wired --config-file unknown-client.toml
  Clients section error: 'what' is not a valid TOML table
  [1]
