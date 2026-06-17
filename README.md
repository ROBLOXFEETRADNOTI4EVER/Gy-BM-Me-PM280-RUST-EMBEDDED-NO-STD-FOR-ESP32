# BMP280 Driver — no_std Rust for ESP32

A minimal BMP280 temperature, pressure, and altitude driver written in no_std Rust for ESP32 using Embassy + esp-hal.

---

## Hardware

| BMP280 Pin | ESP32 Pin |
|------------|-----------|
| SDA        | GPIO21    |
| SCL        | GPIO22    |
| VCC        | 3.3V      |
| GND        | GND       |

Default I2C address: `0x76`

---

## Dependencies

```toml
[dependencies]
esp-hal = { version = "...", features = ["async"] }
esp-hal-embassy = "..."
embassy-executor = "..."
embassy-time = "..."
defmt = "..."
libm = "..."
```

---

## Usage

```rust
// Initialize I2C and create the driver
let bmp_i2c = I2c::new(peripherals.I2C0, Config::default()
    .with_frequency(Rate::from_khz(150)))
    .unwrap()
    .with_sda(peripherals.GPIO21)
    .with_scl(peripherals.GPIO22)
    .into_async();

let mut bmp280 = bmp_uart::new(bmp_i2c, 0x76).await;
bmp280.begin().await; // reads calibration coefficients and sets sampling

// Read temperature (°C)
if let Some(temp) = bmp280.read_temperature().await {
    info!("{}C", temp);
}

// Read pressure (Pa / hPa)
if let Some(pressure) = bmp280.read_pressure().await {
    info!("{}hPa", pressure / 100.);
}

// Read altitude (meters) — pass today's local QNH, not 1013.25
if let Some(altitude) = bmp280.read_altitude(1021.0).await {
    info!("{}m", altitude);
}
```

> **Note on altitude:** Pass your local sea-level pressure (QNH) for accurate results.
> `1013.25` is the ISA standard average — it will give wrong altitude unless that happens to be today's actual pressure.

---

## Default Sampling Settings

| Parameter         | Value         |
|-------------------|---------------|
| Mode              | Normal        |
| Temperature OS    | 2x            |
| Pressure OS       | 4x            |
| Filter            | 2x            |
| Standby           | 125ms         |

Call `change_settings()` before `begin()` to override.
