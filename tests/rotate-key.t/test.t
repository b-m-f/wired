Check that always-rotate-key works
  $ cd $TESTDIR
  $ wired --config-file rotate.toml -f
  $ cat rotate.statefile | grep privatekey > first.keys
  $ wired --config-file rotate.toml -f
  $ cat rotate.statefile | grep privatekey > second.keys
  $ diff first.keys second.keys
  1,2c1,2
  .* (re)
  .* (re)
  ---
  .* (re)
  .* (re)
  [1]

Cleanup
  $ rm -rf wired
  $ rm *.statefile
  $ rm first.keys second.keys
