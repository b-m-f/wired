Make sure all config files get created
  $ cd $TESTDIR
  $ wired --config-file full.toml

Check that expected files were created
  $ ls wired/full
  client.conf
  server.conf

Check that statefile is almost the same as input 
  $ cmp full.toml full.statefile

  $ diff full.toml full.statefile

Check that statefile is different after rekey
  $ wired --config-file full.toml -f -r

Check that statefile is the same as input
  $ cmp full.toml full.statefile
  full.toml full.statefile differ: char 27, line 2
  [1]

Cleanup
  $ rm -rf wired
  $ rm *.statefile
