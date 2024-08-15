Create from full config:
  $ cd $TESTDIR
  $ wired --config-file full.toml

Check that all files were created
  $ ls wired/full
  client.png
  server.conf

Cleanup
  $ rm -rf wired
  $ rm full.statefile
