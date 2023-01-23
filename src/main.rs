#![no_std]
#![feature(type_alias_impl_trait)]
#![no_main]

use accelerometer::Accelerometer;
use defmt::{info, unwrap};
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_nrf::interrupt;
//use embassy_nrf::gpio::{Input, Pull};
use embassy_nrf::temp::Temp;
use embassy_time;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;

pub mod  accelerometer;

static SHARED: Channel<ThreadModeRawMutex, u16, 2> = Channel::new();

#[embassy_executor::task]
async fn run1() {
    loop {
        embassy_time::Timer::after(embassy_time::Duration::from_millis(500)).await;
        info!("Tick from fast timer");
    }
}

#[embassy_executor::task]
async fn run2() {
    loop {
        embassy_time::Timer::after(embassy_time::Duration::from_millis(2000)).await;
        info!("Tick from slow timer");
    }
}

#[embassy_executor::task]
async fn measure_vals(mut temp: Temp<'static>, mut acc: Accelerometer<'static>) {
    loop {
        let value = temp.read().await;
        SHARED.send(value.to_num::<u16>()).await;
        let acc_x = acc.accel_data();
        match acc_x {
            Ok(measurement) => {
                info!("Accelerometer: z: {}", measurement.z);
            },
            Err(_) => {
                info!("Error in reading acclerometer")
            }
        }
        embassy_time::Timer::after(embassy_time::Duration::from_secs(1)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let temp_irq = interrupt::take!(TEMP);

    let temp = Temp::new(p.TEMP, temp_irq);

    unwrap!(spawner.spawn(run1()));
    unwrap!(spawner.spawn(run2()));



    // temp.read().await;

    let receiver = SHARED.receiver();
    let acc_irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let acc = accelerometer::Accelerometer::new(p.TWISPI0, acc_irq, p.P0_16, p.P0_08).unwrap();

    unwrap!(spawner.spawn(measure_vals(temp, acc)));
    loop {
        let temp_val = receiver.recv().await;
        info!("Temperature is: {}", temp_val);
    }

}
