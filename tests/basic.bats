#!/usr/bin/env bats

setup() {
    load './test_helper/bats-support/load'
    load './test_helper/bats-assert/load'
}
  
client_configs=( "desktop.conf" "laptop.conf" "phone.conf" "server2.conf" "server.conf" )

@test "Consecutive runs on the same config do not change the configs per default" {
  for file in "${client_configs[@]}"
  do
      wired --config-file configs/test-basic.toml
      result_first_run="$(cat configs/test-basic/$file | md5sum | awk '{print $1;}')"

      wired --config-file configs/test-basic.toml
      result_second_run="$(cat configs/test-basic/$file | md5sum | awk '{print $1;}')"

      # Expect the configs not to change between runs
      [ "$result_first_run" = "$result_second_run" ]
  done
  run cat configs/test-basic/gateway.conf

  assert_line --regexp 'Address = 10.0.0.1'
  assert_line --regexp 'PrivateKey =.*'
  assert_line --regexp 'DNS = 1.1.1.1'
  assert_line --regexp 'ListenPort = 10101'
  
  run cat configs/test-basic/laptop.conf

  assert_line --regexp 'DNS = 10.0.0.1'

  for file in "${client_configs[@]}"
  do
      run cat configs/test-basic/$file

      assert_line --regexp 'Address = .*'
      assert_line --regexp 'PrivateKey =.*'
      assert_line --regexp 'PersistentKeepalive = 25'
      assert_line --regexp 'PublicKey = .*'
      assert_line --regexp 'AllowedIPs = 10.0.0.1'
      assert_line --regexp 'Endpoint = test.test:10101'

  done
}

@test "IPs are changed correctly" {
  for file in "${client_configs[@]}"
  do
    wired --config-file configs/test-rotate-ips.toml
    result_first_run="$(cat configs/test-rotate-ips/$file | md5sum | awk '{print $1;}')"
    mv configs/test-rotate-ips.toml configs/test-rotate-ips.toml.bkp
    mv configs/test-rotate-ips-reorder.toml configs/test-rotate-ips.toml

    # execute the reordered file. Nothing should change
    wired --config-file configs/test-rotate-ips.toml
    result_second_run="$(cat configs/test-rotate-ips/$file| md5sum | awk '{print $1;}')"

    # Expect the configs not to change between runs
    [ "$result_first_run" = "$result_second_run" ]
    mv configs/test-rotate-ips.toml configs/test-rotate-ips-reorder.toml
    mv configs/test-rotate-ips.toml.bkp configs/test-rotate-ips.toml
  done

  # Now enable IP switching and expect the IPs to differ in the diff
  # Keys should all be the same
  cp -r configs/test-rotate-ips configs/test-rotate-ips.bkp

  wired --config-file configs/test-rotate-ips.toml -i

  for file in "${client_configs[@]}"
  do
      run diff configs/test-rotate-ips/$file configs/test-rotate-ips.bkp/$file

      if assert_failure; then
          refute_line --regexp 'PrivateKey =.*'
          assert_line --regexp 'Address = .*'
      else
        assert_success
      fi

  done
}

@test "Keys are changed correctly" {
  for file in "${client_configs[@]}"
  do
      wired --config-file configs/test-rotate-keys.toml
      result_first_run="$(cat configs/test-rotate-keys/$file  | md5sum | awk '{print $1;}')"


      wired --config-file configs/test-rotate-keys.toml
      result_second_run="$(cat configs/test-rotate-keys/$file  | md5sum | awk '{print $1;}')"


      # Expect the configs to change between runs
      [ "$result_first_run" = "$result_second_run" ]
  done

  # Save output configs to compare them after executing a key rotation
  cp -r configs/test-rotate-keys configs/test-rotate-keys.bkp


  wired -r --config-file configs/test-rotate-keys.toml

  for file in "${client_configs[@]}"
  do
      run diff configs/test-rotate-keys/$file configs/test-rotate-keys.bkp/$file
      
      refute_line --regexp 'Address = .*'
      assert_line --regexp 'PrivateKey =.*'
      assert_line --regexp 'PresharedKey = .*'
  done
}

@test "Keys and IPs are changed correctly" {
  for file in "${client_configs[@]}"
  do
      wired --config-file configs/test-rotate-keys-and-ips.toml
      result_first_run="$(cat configs/test-rotate-keys-and-ips/$file  | md5sum | awk '{print $1;}')"

      mv configs/test-rotate-keys-and-ips.toml configs/test-rotate-keys-and-ips.toml.bkp
      mv configs/test-rotate-keys-and-ips-reorder.toml configs/test-rotate-keys-and-ips.toml

      wired --config-file configs/test-rotate-keys-and-ips.toml
      result_second_run="$(cat configs/test-rotate-keys-and-ips/$file  | md5sum | awk '{print $1;}')"

      mv configs/test-rotate-keys-and-ips.toml configs/test-rotate-keys-and-ips-reorder.toml
      mv configs/test-rotate-keys-and-ips.toml.bkp configs/test-rotate-keys-and-ips.toml


      # Expect the configs to change between runs
      [ "$result_first_run" = "$result_second_run" ]
  done

  # Save output configs to compare them after executing a key rotation
  cp -r configs/test-rotate-keys-and-ips configs/test-rotate-keys-and-ips.bkp


  wired -r -i --config-file configs/test-rotate-keys-and-ips.toml

  for file in "${client_configs[@]}"
  do
      run diff configs/test-rotate-keys-and-ips/$file configs/test-rotate-keys-and-ips.bkp/$file
      
      if assert_line --regexp 'Address =.*'; then
        assert_line --regexp 'Address =.*'
      fi
      assert_line --regexp 'PrivateKey =.*'
      assert_line --regexp 'PresharedKey = .*'
  done
}
