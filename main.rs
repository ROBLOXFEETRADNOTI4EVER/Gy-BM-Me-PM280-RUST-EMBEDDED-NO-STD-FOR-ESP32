#![no_std]
#![no_main]

use core::ptr::read;
use core::fmt::UpperHex;
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


#[repr(u8)] // <--- We need this to make sure it detects it as a u8 and not a isize
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
    spawner.must_spawn(read_bytes(bmp280_i2c,BMPADDRESSES::Bmp280RegisterControl as u8));




    // bmp280_i2c.write_read(address, write_buffer, read_buffer)
    
    // loop {
    //     info!("Hello world!");
    //     Timer::after(Duration::from_secs(1)).await;
    // }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}

#[embassy_executor::task]
async fn read_bytes(mut bmp280_i2c: I2c<'static, esp_hal::Async>, read_from_register:u8 ){ // NOTE I TOOK OUT THE BYTES TO READ AMOUNT I WILL READ 16 BYTES BY DEFAULT
    info!("Before runing the loop");
    // Timer::after(Duration::from_millis(100)).await;
    let bmp280i2c = 0x76;

    // let mut register_buff = [0u8; 8];
    // bmp280_i2c.write_read_async(BMPADDRESSES::Bmp280RegisterControl as u8, &[0x03], &mut register_buff).await.unwrap(); // 0xF= Register PRESSURE DATA
  
    //       Timer::after(Duration::from_millis(300)).await;
    //       info!("buffer is {}",register_buff);


    let mut bmp280_register_dig_t1_buffer = [0u8; 16];
    bmp280_i2c.write_read_async(bmp280i2c, &[BMPADDRESSES::Bmp280RegisterDigT1 as u8], &mut bmp280_register_dig_t1_buffer).await.unwrap(); // 0xF= Register PRESSURE DATA
    let dig_T1 : u16 = (bmp280_register_dig_t1_buffer[0] << 8 | (bmp280_register_dig_t1_buffer[1])   ) as u16;





    let mut bmp280_register_dig_t2_buffer = [0u8; 16];
    bmp280_i2c.write_read_async(bmp280i2c, &[BMPADDRESSES::Bmp280RegisterDigT2 as u8], &mut bmp280_register_dig_t2_buffer).await.unwrap(); // 0xF= Register PRESSURE DATA
    let dig_T2 : i16 = (bmp280_register_dig_t2_buffer[0] << 8 | (bmp280_register_dig_t2_buffer[1])   ) as i16;




    let mut bmp280_register_dig_t3_buffer = [0u8; 16];
    bmp280_i2c.write_read_async(bmp280i2c, &[BMPADDRESSES::Bmp280RegisterDigT3 as u8], &mut bmp280_register_dig_t3_buffer).await.unwrap(); // 0xF= Register PRESSURE DATA
    let dig_T3 : i16 = (bmp280_register_dig_t3_buffer[0] << 8 | (bmp280_register_dig_t3_buffer[1])   ) as i16;

    let mut read_buff = [0u8; 8];
    bmp280_i2c.write_read_async(bmp280i2c, &[BMPADDRESSES::Bmp280RegisterChipid as u8], &mut read_buff).await.unwrap(); // 0xF= Register PRESSURE DATA


    info!("sensor id in  {}",read_buff);
  // if sensor_id != 0x56{
    
  let chip_id: u8= 0x58;
  // }
  if  chip_id != read_buff[0]{ // Another thing learned you can compare hexa values with decimal values if they are the same value just coded differently they will still match
      println!("false");
  } // If we don't see a false here then everything turned out to be okay
      


 
    // info

    loop{

        Timer::after(Duration::from_millis(200)).await;

    // let mut write_buff : [0u8;8];
    // let mut read_buff : [0u8; 22];
    // let mut write_buff: [u8] = [0u8];

    // let mut write_sht = [0u8; 8];

    bmp280_i2c.write_async(bmp280i2c, &[0xF4,0x25]).await.unwrap(); // RULE REMEMBER ADRESS THE CHIP FUCKING ADRESSS BUFFER INSIDE THE LOACTION WHERE YOU WANT TO WRITE AND WHAT THE FUCK YOU WANT TO WRITE 
    // bmp280_i2c.write_read_async(bmp280i2c, &[0x25], &mut read_buff).await.unwrap(); // 0xF= Register PRESSURE DATA

//   !("In loop but before i2c write read ");

    bmp280_i2c.write_read_async(bmp280i2c, &[read_from_register], &mut read_buff).await.unwrap(); // 0xF= Register PRESSURE DATA

    info!("buffer is {}",read_buff);
     
    // bmp280 temp reading
    bmp280_i2c.write_read_async(bmp280i2c, &[BMPADDRESSES::Bmp280RegisterTempdata as u8], &mut read_buff).await.unwrap(); // 0xF= Register PRESSURE DATA
    // bmp280_i2c.read_async(BMPADDRESSES::Bmp280RegisterTempdata as u8, &mut read_buff).await.unwrap();
    
    info!("Temperature buffer is {}",read_buff);
    /* return uint16_t(buffer[0]) << 8 | uint16_t(buffer[1]); mimicking this */ 
    let temp : u16 = (read_buff[0] << 8 | (read_buff[1])   ) as u16;

    // now mimicking this   return (temp >> 8) | (temp << 8);
    info!("Temperature bit shifting {}",(temp >> 8) | (temp << 8));
    let temp_read_le: u16 = (temp >> 8) | (temp << 8);

    let mut temp_buff = [0u8; 3];

    bmp280_i2c.write_read_async(bmp280i2c, &[BMPADDRESSES::Bmp280RegisterTempdata as u8], &mut temp_buff).await.unwrap(); // 0xF= Register PRESSURE DATA

    let mut adc_T: i32 = (temp_buff[0] as i32) << 16 | (temp_buff[1] as i32) << 8 | (temp_buff[2] as i32); // If this doesn't work the issue is with parsing the buffers to u32 and bit shifting with them


    info!("ADC_T {}",adc_T);
    info!("DIG_T1 {}",dig_T1);
    info!("DIG_T2 {}",dig_T2);
    info!("DIG_T3 {}",dig_T3);
    adc_T = adc_T >> 4;






    let var1: i32 =
    ((((adc_T as i32 >> 3) - ((dig_T1 as i32) << 1)))
        * (dig_T2 as i32))
        >> 11;
  info!("var 1 {}",var1);
let var2: i32 =
    ((((((adc_T as i32 >> 4) - (dig_T1 as i32))
        * ((adc_T as i32 >> 4) - (dig_T1 as i32)))
        >> 12)
        * (dig_T3 as i32))
        >> 14);
        info!("var 2 {}",var2);


        let t_fine = var1 + var2;

        let t: i32 = (t_fine * 5 + 128) >> 8;
        info!("Temperature :{}",t / 100);
        let mut test_buffer = [0u8; 8];

}



}

// TODO IMPLEMENT READING 16 bytes and  24 bytes
//  READ THE CALIBRATION AND CALCULATE THE TEMPEATURE 
