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

  #[repr(8)] // <--- We need this to make sure it detects it as a u8 and not a isize
pub enum SensorSampling {
    /** No over-sampling. */
    SamplingNone = 0x00,
    /** 1x over-sampling. */
    SamplingX1 = 0x01,
    /** 2x over-sampling. */
    SamplingX2 = 0x02,
    /** 4x over-sampling. */
    SamplingX4 = 0x03,
    /** 8x over-sampling. */
    SamplingX8 = 0x04,
    /** 16x over-sampling. */
    SamplingX16 = 0x05
  }
  struct Bmp280CalibrationData{
    dig_t1 : u16,
    dig_t2 : i16,
    dig_t3 : i16,
    
    dig_p1 : u16,
    dig_p2 : i16,
    dig_p3 : i16,
    dig_p4 : i16,
    dig_p5 : i16,
    dig_p6 : i16,
    dig_p7 : i16,
    dig_p8 : i16,
    dig_p9 : i16,

  }

   /** Operating mode for the sensor. */
pub enum SensorMode {
    /** Sleep mode. */
    ModeSleep = 0x00,
    /** Forced mode. */
    ModeForced = 0x01,
    /** Normal mode. */
    ModeNormal = 0x03,
    /** Software reset. */
    ModeSoftResetCode = 0xB6
  }

   /** Filtering level for sensor data. */
pub enum SensorFilter {
    /** No filtering. */
    FilterOff = 0x00,
    /** 2x filtering. */
    FilterX2 = 0x01,
    /** 4x filtering. */
    FilterX4 = 0x02,
    /** 8x filtering. */
    FilterX8 = 0x03,
    /** 16x filtering. */
    FilterX16 = 0x04
  }

  /** Standby duration in ms */
pub enum StandbyDuration {
    /** 1 ms standby. */
    StandbyMs1 = 0x00,
    /** 62.5 ms standby. */
    StandbyMs63 = 0x01,
    /** 125 ms standby. */
    StandbyMs125 = 0x02,
    /** 250 ms standby. */
    StandbyMs250 = 0x03,
    /** 500 ms standby. */
    StandbyMs500 = 0x04,
    /** 1000 ms standby. */
    StandbyMs1000 = 0x05,
    /** 2000 ms standby. */
    StandbyMs2000 = 0x06,
    /** 4000 ms standby. */
    StandbyMs4000 = 0x07
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
        // todo!() later make it so it has default parameters I can just call and save it to the struct itself
        self.set_sampling(SensorMode::ModeNormal,SensorSampling::SamplingX4,SensorSampling::SamplingX4,SensorFilter::FilterX2,StandbyDuration::StandbyMs125).await;
        Timer::after(Duration::from_millis(30)).await;
        true
     }
        
        

    }
    async fn write_8(&mut self , register :u8,value:u8){
        let mut buffer = [0u8; 2];

        buffer[0] = register; // putting the register in the 1st part of the buffer
        buffer[1] = value; // putting the value in the second part of the buffer

        self.i2c.write_async(self.chip_address as u8,&mut buffer ).await; // NEED TO ADD A PROPER ERROR HANDLING SYSTEM

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

    async fn set_sampling(&mut self, sensor_mode:SensorMode,temperature_sampling:SensorSampling,pressure_sampling:SensorSampling,filter:SensorFilter,duration:StandbyDuration){
        // todo!();

        // everything is hard coded so no
        /*
         *   _measReg.mode = mode;
                _measReg.osrs_t = tempSampling;
                _measReg.osrs_p = pressSampling;

                _configReg.filter = filter;
                _configReg.t_sb = duration;
         * 
         * */

         /*
            C++ CODE BELOW ME IS TRANSLATED TO ctrl_get 
             unsigned int get() { return (osrs_t << 5) | (osrs_p << 2) | mode; }
--------------------------------------------------------------------------------------------------
            C++ CODE BELOW ME IS TRANSLATED TO config_get 
             unsigned int get() { return (t_sb << 5) | (filter << 2) | spi3w_en; }
          */
        let config_get: u8 = ((duration as u8 )<< 5) | ((filter as u8) << 2) ;
        let ctrl_get: u8 = ((temperature_sampling as u8 )<< 5) | ((pressure_sampling as u8) << 2) ;




        self.write_8(BMPADDRESSES::Bmp280RegisterConfig as u8, config_get).await;
        self.write_8(BMPADDRESSES::Bmp280RegisterConfig as u8, ctrl_get).await;
        
        
         //NEED TO IMPLEMENT A .get to bitshift numbs since it I'm having troubles  i need to shift all the config data into a u8 and pass it as an arugment
        // CONTINUE FROM LINE 139 
        // CONTINUE WITH SET SAMPLING YES I MADE AN ERROR BY CHOICE 
        // https://github.com/adafruit/Adafruit_BMP280_Library/blob/master/Adafruit_BMP280.cpp#L125

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
