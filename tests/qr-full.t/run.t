Create from full config:
  $ cd $TESTDIR
  $ wired --config-file full.toml

Check that all files were created
  $ ls full
  client.png
  server.conf

Cleanup
  $ rm -rf full
  $ rm full.statefile
