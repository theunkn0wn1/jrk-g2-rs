# pololu Jrk G2 rust driver

[![doc](https://docs.rs/jrk-g2-rs/badge.svg)](https://docs.rs/jrk-g2-rs)

- Product page: https://www.pololu.com/product/3147
- Arduino Driver: https://github.com/pololu/jrk-g2-arduino

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
