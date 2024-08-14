  $ cd $TESTDIR
Fail when network is missing:
  $ wired --config-file fail-missing.toml
  Error when parsing configuration: missing field `network` at line 10 column 1
  [1]

Fail when network is missing cidrv4:

  $ wired --config-file fail-cidrv4.toml
  Network is missing required field 'cidrv4'
  [1]

Fail when network is missing name:
  $ wired --config-file fail-name.toml
  Network is missing required field 'name'
  [1]
