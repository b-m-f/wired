Make sure all config files get created
  $ cd $TESTDIR
  $ wired --config-file minimal.toml

Check that expected files were created
  $ ls wired/minimal
  client.conf
  server.conf

Check that statefile changed with new output
  $ diff minimal.toml minimal.statefile | grep output | cut -c 3-
  output = "conf"
  output = "conf"
  $ diff minimal.toml minimal.statefile | grep privatekey | awk '{print $2;}'
  privatekey
  privatekey
  $ diff minimal.toml minimal.statefile | grep presharedkey | awk '{print $2;}'
  presharedkey
 

Cleanup
  $ rm -rf wired
  $ rm *.statefile
