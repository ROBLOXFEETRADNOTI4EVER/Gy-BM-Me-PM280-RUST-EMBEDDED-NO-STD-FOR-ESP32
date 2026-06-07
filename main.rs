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
#[repr(u8)]
enum BMPADDRESSES{
    Bmp280RegisterDigT1 = 0x88,
    Bmp280RegisterDigT2 = 0x8A,
    Bmp280RegisterDigT3 = 0x8C,
    Bmp280RegisterDigP1 = 0x8E,
    Bmp280RegisterDigP2 = 0x90,
    Bmp280RegisterDigP3 = 0x92,
    Bmp280RegisterDigP4 = 0x94,
    Bmp280RegisterDigP5 = 0x96,
    Bmp280RegisterDigP6 = 0x98,
    Bmp280RegisterDigP7 = 0x9A,
    Bmp280RegisterDigP8 = 0x9C,
    Bmp280RegisterDigP9 = 0x9E,
    Bmp280RegisterChipid = 0xD0,
    Bmp280RegisterVersion = 0xD1,
    Bmp280RegisterSoftreset = 0xE0,
    Bmp280RegisterCal26 = 0xE1, /**< R calibration = 0xE1-0xF0 */
    Bmp280RegisterStatus = 0xF3,
    Bmp280RegisterControl = 0xF4,
    Bmp280RegisterConfig = 0xF5,
    Bmp280RegisterPressuredata = 0xF7,
    Bmp280RegisterTempdata = 0xFA,
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
    spawner.must_spawn(read_8_bytes(bmp280_i2c,BMPADDRESSES::Bmp280RegisterControl as u8));




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

// TODO IMPLEMENT READING 16 bytes and  24 bytes
//  READ THE CALIBRATION AND CALCULATE THE TEMPEATURE 
