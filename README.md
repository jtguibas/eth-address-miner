# eth-address-miner

finds interesting addresses very quickly

## example output

```
eth-address-miner
> searching for prefix: deadb
> number of cpus: 8

[0] iterations / sec = 339643, total iterations = 340127
[1] iterations / sec = 338145, total iterations = 678338
[2] iterations / sec = 347536, total iterations = 1025881
[3] iterations / sec = 338528, total iterations = 1364414

match found!
private key: 0xa6ff9ae4b828679d148caf5985a1240ee09363bd32a921ab8b5f261f29e02907
public key: 0x032a3662a92954fde582f544e9e6281642e406722b6c8626e83edc99ef3b458060
address: 0xdeadbf7074caf261d039580e18bf7dfdddad43d8
```

## instructions

```
cargo build --release
cd target/release
./eth-address-miner deadb
```
