  $ cd $TESTDIR
Error when missing servers:

  $ wired --config-file missing-servers.toml
  Error: Missing server section
  [1]

Error when unknown server config is encountered:

  $ wired --config-file unknown-server.toml
  Error: Unknown toml configuration 'what' in server section
  [1]

Error when missing clients:

  $ wired --config-file missing-clients.toml
  Error: Missing Clients section
  [1]

Error when unknown client config is encountered:

  $ wired --config-file unknown-client.toml
  Error: Unknown toml configuration 'what' in clients section
  [1]
