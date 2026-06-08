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
pub enum BMPADDRESSES{
    Bmp280RegisterDigT1 = 0x88 as u8,
    Bmp280RegisterDigT2 = 0x8A as u8,
    Bmp280RegisterDigT3 = 0x8C as u8,
    Bmp280RegisterDigP1 = 0x8E as u8,
    Bmp280RegisterDigP2 = 0x90 as u8,
    Bmp280RegisterDigP3 = 0x92 as u8,
    Bmp280RegisterDigP4 = 0x94 as u8,
    Bmp280RegisterDigP5 = 0x96 as u8,
    Bmp280RegisterDigP6 = 0x98 as u8,
    Bmp280RegisterDigP7 = 0x9A as u8,
    Bmp280RegisterDigP8 = 0x9C as u8,
    Bmp280RegisterDigP9 = 0x9E as u8,
    Bmp280RegisterChipid = 0xD0 as u8,
    Bmp280RegisterVersion = 0xD1 as u8,
    Bmp280RegisterSoftreset = 0xE0 as u8,
    Bmp280RegisterCal26 = 0xE1 as u8, /**< R calibration = 0xE1-0xF0 */
    Bmp280RegisterStatus = 0xF3 as u8,
    Bmp280RegisterControl = 0xF4 as u8,
    Bmp280RegisterConfig = 0xF5 as u8,
    Bmp280RegisterPressuredata = 0xF7 as u8,
    Bmp280RegisterTempdata = 0xFA as u8,
  }

  struct bmp_uart{
    i2c:  I2c<'static, esp_hal::Async>,
    chip_address : u8
}


impl  bmp_uart{
    
    async fn new(i2c:  I2c<'static, esp_hal::Async>,chip_address : u8 ) -> Self{
        Self {i2c,chip_address}

    }
    async fn print_self_data(&mut self){
    //   info!("sel.i2c:{} self.chip_address:{} ",self.i2c,self.chip_address);
    info!("Chip address: {=u8:#x} as u8",self.chip_address); // https://defmt.ferrous-systems.com/hints read about how to print a u8 as hex value 

    }

    async fn check_chip(&mut self) -> bool{
        Timer::after(Duration::from_millis(30)).await;

        let mut read_buff = [0u8; 1];
        let chip_reading = self.i2c.write_read_async(self.chip_address, &[BMPADDRESSES::Bmp280RegisterChipid as u8], &mut read_buff).await; 
        // info!("the read buffer is {}",read_buff);
        match chip_reading{
            Ok(_) =>{
                // todo!()
            }
            Err(e) =>{
                info!("Error {}",e)
            }
        }
        if read_buff[0] != 0x58{ // checking if the chips address that is being read it same as the given one 
            info!("Chip id doesn't match check your sensor and hardware");
            return false;
        }else{
            info!("true");
            return true;
        }
        // true
    }

    async fn begin(&mut self) -> bool{

     if !self.check_chip().await{
        // todo!() 
        false
        // Here we just won't do anything since the check_chip() will say if there is an error
     }else{



        self.read_coefficents().await;
        self.set_sampling().await;
        Timer::after(Duration::from_millis(30)).await;
        true
     }
        
        

    }

    async fn read_8(&mut self){

        let mut read_8_byte_buffer = [0u8; 1];
        let read_8_bytes = self.i2c.write_read_async(self.chip_address, &[BMPADDRESSES::Bmp280RegisterChipid as u8], &mut read_8_byte_buffer).await; 
        match read_8_bytes{
            Ok(_) =>{
                


            }
            Err(e) =>{
                info!("Error {}",e)
            }
        }
    }

    async fn read_16(&mut self){
        todo!();
    }

    async fn read_16_le(&mut self){
        todo!();
    }

    async fn read_24(&mut self){
        todo!();
    }

    async fn read_coefficents(&mut self) {
        todo!();
    }

    async fn set_sampling(&mut self){
        todo!();
    }

 
    
}






#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);
    let bmp_i2c: I2c<'static, esp_hal::Async> = I2c::new(peripherals.I2C0, Config::default().with_frequency(Rate::from_khz(150)))
    .unwrap()
    .with_sda(peripherals.GPIO21)
    .with_scl(peripherals.GPIO22)
    .into_async();
    info!("Embassy initialized!");



    // let my_uart = bmp_uart{
    //     i2c : bmp_i2c,
    //     chip_address :0x76,
    // };

    let mut bmp280: bmp_uart = bmp_uart::new(bmp_i2c, 0x76).await;

    bmp280.print_self_data().await;
    // TODO: Spawn some tasks
    // let _ = spawner;
    info!("checking if the chips id is the same as what is being read:{:#?}",bmp280.check_chip().await);


    loop {
        info!("Hello world!");
        Timer::after(Duration::from_millis(1000)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
