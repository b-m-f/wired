  $ cd $TESTDIR
Fail if server is missing endpoint:

  $ wired --config-file fail-missing.toml
  Error: No endpoint specified for server server
  [1]
Fail if server has wrong endpoint:

  $ wired --config-file fail-wrong.toml
  Error: Endpoint for server server has to be a string
  [1]
