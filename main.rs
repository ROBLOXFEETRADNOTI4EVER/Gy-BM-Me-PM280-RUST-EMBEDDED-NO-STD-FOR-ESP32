#![no_std]
#![no_main]

use defmt::{info, println};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{clock::CpuClock, time::Rate};
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use esp_hal::i2c::master::{Config, I2c};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    
    loop {}
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    // The sensor may be available at 0x77 or 0x76.

    // it should be 0x76

    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    let _ = spawner;

    // let mut gy_bm_pm280_i2c
    let bmp280_i2c: I2c<'static, esp_hal::Async> = I2c::new(peripherals.I2C0, Config::default().with_frequency(Rate::from_khz(150))).unwrap()
    .with_sda(peripherals.GPIO21)
    .with_scl(peripherals.GPIO22)
    .into_async();


    // Need to first write to the i2c to read it 
    // read write 8 bits bits and then read 0x80 at this register

    // sensor_id is writing and reading from and checking if it is = 0xD0 or  0x58 // It is actually 0x58
    // must continue implementing this https://github.com/adafruit/Adafruit_BMP280_Library/blob/master/Adafruit_BMP280.cpp#L186

    // need a read function that takes a register as a parameter and reads from it 8 bytes worth of data to a buffer  and returns the buffer
    // spawner.must_spawn(read_8_bytes(bmp280_i2c,0xD0)); to read the chip id which is 0x58
    spawner.must_spawn(read_8_bytes(bmp280_i2c,0xD0));




    // bmp280_i2c.write_read(address, write_buffer, read_buffer)
    
    // loop {
    //     info!("Hello world!");
    //     Timer::after(Duration::from_secs(1)).await;
    // }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}

#[embassy_executor::task]
async fn read_8_bytes(mut bmp280_i2c: I2c<'static, esp_hal::Async>, read_from_register:u8 ){
    info!("Before runing the loop");
    loop{
        Timer::after(Duration::from_millis(200)).await;
    let bmp280i2c = 0x76;

    // let mut write_buff : [0u8;8];
    // let mut read_buff : [0u8; 22];
    // let mut write_buff: [u8] = [0u8];

    let mut read_buff = [0u8; 8];

    info!("In loop but before i2c write read ");

    bmp280_i2c.write_read_async(bmp280i2c, &[read_from_register], &mut read_buff).await.unwrap(); // 0xF= Register PRESSURE DATA

    info!("buffer is {}",read_buff);
    
}

}
