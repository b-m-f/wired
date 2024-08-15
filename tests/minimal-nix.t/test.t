Make sure all config files get created
  $ cd $TESTDIR
  $ wired --config-file minimal.toml

Check that expected files were created
  $ ls wired/minimal
  client.key
  client.nix
  minimal.psk
  server.key
  server.nix

Check that statefile changed with new keys
  $ cmp minimal.toml minimal.statefile
  minimal.toml minimal.statefile differ: char 11, line 2
  [1]

Cleanup
  $ rm -rf wired
  $ rm *.statefile
