  $ cd $TESTDIR
Fail when network is missing:
  $ wired --config-file fail-missing.toml
  Error: Missing network specification
  [1]

Fail when network is missing cidrv4:

  $ wired --config-file fail-cidrv4.toml
  Error: Missing network CIDR v4
  [1]

Fail when network is missing name:
  $ wired --config-file fail-name.toml
  Error: Missing network name
  [1]
