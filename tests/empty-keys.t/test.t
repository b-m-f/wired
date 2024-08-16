Make sure all config files get created
  $ cd $TESTDIR
  $ wired --config-file empty-psk.toml
  $ wired --config-file empty-private-server.toml
  $ wired --config-file empty-private-client.toml

Check that expected files were created
  $ ls wired/empty-psk
  client.conf
  server.conf
  $ ls wired/empty-private-client
  client.conf
  server.conf
  $ ls wired/empty-private-server
  client.conf
  server.conf

Check that statefile changed with new output
  $ diff empty-psk.toml empty-psk.statefile | grep presharedkey | awk '{print $2;}'
  presharedkey
  presharedkey

  $ diff empty-private-client.toml empty-private-client.statefile | grep privatekey | awk '{print $2;}'
  privatekey
  privatekey
  privatekey

  $ diff empty-private-server.toml empty-private-server.statefile | grep privatekey | awk '{print $2;}'
  privatekey
  privatekey
  privatekey
 

Cleanup
  $ rm -rf wired
  $ rm *.statefile
