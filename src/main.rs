#![no_std]
#![feature(type_alias_impl_trait)]
#![no_main]

use defmt::{info, unwrap};
use  {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
// use embassy_nrf::interrupt;
use embassy_time;

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

/*
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    // let mut t = Timer::new_awaitable(p.TIMER0, interrupt::take!(TIMER0));

    // all timers are implemented as registers in uc
    // for 1 MHz default frequency, 1_000_000 count will strike after 1 second
    t.cc(0).write(1_000_000);
    // 
    t.cc(0).short_compare_clear();
    t.start();

    loop {
        t.cc(0).wait().await;
        info!("hardware timer tick");
    }
}
*/

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _p = embassy_nrf::init(Default::default());

    unwrap!(spawner.spawn(run1()));
    unwrap!(spawner.spawn(run2()));
}
