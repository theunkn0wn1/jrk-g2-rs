# pololu Jrk G2 rust driver

## Example with a STM 32:

```
cargo embed --release --features stm32 --chip STM32F103C8 --example stm32-serial
cargo embed --release --features stm32 --chip STM32F103C8 --example stm32-i2c
```

## Example with a Raspberry Pi:

```
cargo run --release --features rpi --example rpi-serial
cargo run --release --features rpi --example rpi-i2c
```

## Example with an Arduino UNO:

```
cd arduino
rustup override set nightly
cargo run --release --example arduino-uno-i2c
cargo run --release --example arduino-uno-serial
```
