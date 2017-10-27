# Hashchain

Create a sha256 chain starting from 32 random bytes

## Building and running

```
$ git clone https://github.com/RCasatta/hashchain
$ cd hashchain
$ cargo build --release
$ ./target/release/hashchain
Printing every 1000000
0 ca8884437d8e80a4408ae87ec0a83519ee12deb105ea349b0ae72e0086ce907c    NaN Mhash/s
1000000 76a0baf462f5606373f9f8b5dfbad2e4b4305b8c534256b28c629233af14d72d    inf Mhash/s
2000000 3595ab4266ac879d7295dd0acb6564c287f8fefe73ff8d0b75a88fc7d1f0d3a5    inf Mhash/s
3000000 3f3897a1af75aa33e2405d5e3fb0c70e0bb1958e2c6d0e2cb07a92df45412a43  3.000 Mhash/s
4000000 229353d77959b733ec6d2fa974fc1330b65e902990dc0e85d40061f3ef1c3d68  4.000 Mhash/s

```
Remember to add the `--release` flag otherwise performance are order of magnitude lower.


### Running with less output
```
$ ./target/release/hashchain 20
Printing every 20000000
0 d336fceb88eb8073495802ebfff62f812dfe269cf81312a2d604f58d6c4bbdd1    NaN Mhash/s
20000000 99907160a1e11bc073e5ca6057fba268ba5360152207d1cc5bea0c1b1bf48d39  2.222 Mhash/s
40000000 5351e27b637f406a589788ccfee0a3a77b8d8221767fba3ed092efb46ce1cc39  2.222 Mhash/s
```

### Running with nice and logging 
To start the process on a server with too much free time without impacting on performance
```
$ nice -10 ./target/release/hashchain 1000 >chainXXX &
```

## Now what?

Use [timelock](https://github.com/petertodd/timelock) from Todd/Taaki to create a timelock by manually change the config file:

* Use the second element (eg. `ca8884437d8e80a4408ae87ec0a83519ee12deb105ea349b0ae72e0086ce907c`) in the first line produced by the program to initialize the `iv`.
* Use the first element of the last line (eg. `4000000`) to set `i` and `n`
* Use the second element of the last line (eg. `229353d77959b733ec6d2fa974fc1330b65e902990dc0e85d40061f3ef1c3d68`) to set the `midstate`
