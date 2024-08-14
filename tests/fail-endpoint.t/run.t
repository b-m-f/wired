  $ cd $TESTDIR
Fail if server is missing endpoint:

  $ wired --config-file fail-missing.toml
  Server server is missing required field 'endpoint'
  [1]
Fail if server has wrong endpoint:

  $ wired --config-file fail-wrong.toml
  Server server has wrong endpoint defined: 'true'
  [1]
