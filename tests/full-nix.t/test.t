Make sure all config files get created
  $ cd $TESTDIR
  $ wired --config-file full.toml

Check that expected files were created
  $ ls full
  client.conf
  server.nix
