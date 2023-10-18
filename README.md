# gocontroll_platform
rust api for the GOcontroll Moduline (IV/Mini/Display) controllers

**This is a hobby project and not officially supported by GOcontroll!**

This api allows you to much more freely program your software without being limited by what can be done with Matlab/Simulink. 
For example network programming, integrating graphics with for example gtk4/slint or makepad for the moduline display. \
It also allows you to make a lot more use of the Linux environment of these controllers.

## Currently tested:
* MCP3004 ADC and leds on the Moduline Mini V1.11
* ADS1015 ADC and leds on the Moduline IV V3.03

## Example
```
//command used for compilation
//cargo build --target aarch64-unknown-linux-gnu
//don't forget to rustup add aarch64-unknown-linux-gnu
//also install the proper linker and put:
//[target.aarch64-unknown-linux-gnu]
//linker = "aarch64-linux-gnu-gcc"
//in /.cargo/config to make it use the proper linker

use gocontroll_platform::gocontroll::mainboard::{MainBoard,AdcChannel};
use std::io;

fn main() {
    futures::executor::block_on(async_main()).expect("err");
}

async fn async_main() -> io::Result<()> {
    let mut mainboard = MainBoard::new();
    // do other initialization
    mainboard.initialize_main_board(&mut []).await?;
    loop {
        // start reading adcs
        let bat_fut = mainboard.read_adc_channel(AdcChannel::K30);
        let k15a_fut = mainboard.read_adc_channel(AdcChannel::K15A);
        let k15b_fut = mainboard.read_adc_channel(AdcChannel::K15B);
        let k15c_fut = mainboard.read_adc_channel(AdcChannel::K15C);
        // do some other stuff
        std::thread::sleep(std::time::Duration::from_millis(400));
        // pick up the values from the adcs
        println!("Battery voltage: {}", bat_fut.await?);
        println!("K15A voltage: {}", k15a_fut.await?);
        println!("K15B voltage: {}", k15b_fut.await?);
        println!("K15C voltage: {}", k15c_fut.await?);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    Ok(())
}
```

## Yet to test
All modules \
GPIO based enclosure LEDs

## Yet to implement
XCP Stack \
CAN Busses
