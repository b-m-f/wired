  $ cd $TESTDIR
Fail when network is missing:
  $ wired --config-file fail-missing.toml
  Error when parsing configuration: TOML parse error at line 1, column 1
    |
  1 | [servers]
    | ^
  missing field `network`
  
  [1]

Fail when network is missing cidrv4:

  $ wired --config-file fail-cidrv4.toml
  Network is missing required field 'cidrv4'
  [1]

Fail when network is missing name:
  $ wired --config-file fail-name.toml
  Network is missing required field 'name'
  [1]
