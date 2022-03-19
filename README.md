# eth-address-miner

finds interesting addresses very quickly

## example output

```
eth-address-miner
> searching for prefix: b00b5
> number of cpus: 8

[0] iterations / sec = 332682, total iterations = 332793
[1] iterations / sec = 335133, total iterations = 667945
[2] iterations / sec = 335613, total iterations = 1003562
[3] iterations / sec = 98574, total iterations = 1102140

match found!
private key: 0x0e74c7fafdcc57cf6033a7e0447fec33149544ca7ae89ca842b8e3e7ebc4c6e6
public key: 0x022631507eaecb8b8eec866c183a39f78c6269644c7bcce62f622ee83c02808ebb
address: 0xb00b5d944be2e1d87c0b750ec1898bdc35b7587d
```

## instructions

```
cargo build --release
cd target/release
./eth-address-miner b00b
```
