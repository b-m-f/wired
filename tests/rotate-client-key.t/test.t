Check that always-rotate-key works for a single client
  $ cd $TESTDIR
  $ wired --config-file rotate.toml -f
  $ cat rotate_client.statefile | grep -A 10 "servers.server" | grep privatekey > server_first.key
  $ cat rotate_client.statefile | grep -A 10 "clients.client" | grep privatekey > client_first.key
  $ wired --config-file rotate.toml -f
  $ cat rotate_client.statefile | grep -A 10 "servers.server" | grep privatekey > server_second.key
  $ cat rotate_client.statefile | grep -A 10 "clients.client" | grep privatekey > client_second.key
  $ diff server_first.key server_second.key
  $ diff client_first.key client_second.key
  1c1
  .* (re)
  ---
  .* (re)
  [1]

Cleanup
  $ rm -rf wired
  $ rm *.statefile
  $ rm *.key
