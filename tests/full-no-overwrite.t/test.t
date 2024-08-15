Make sure all config files get created
  $ cd $TESTDIR
  $ wired --config-file full.toml

Check that expected files were created
  $ ls full
  client.key
  client.nix
  full.psk
  server.key
  server.nix

Make sure configs wont be overwritten
  $ wired --config-file full.toml
  Error when trying to create config dir Config directory full already exists. Use --force to overwrite
  [1]

Works with --force
  $ wired --force --config-file  full.toml

Works with -f
  $ wired -f --config-file  full.toml

Cleanup
  $ rm -rf full
  $ rm *.statefile
