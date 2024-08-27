# ESP32C6 Temperature, Pressure, and Humidity Measurement Project

This project uses an ESP32C6 microcontroller to measure temperature, pressure, and humidity using a BME sensor. The data is then displayed on a computer terminal. The project is implemented in Rust using the `esp-hal` hardware abstraction layer provided by Espressif Systems.

## Features

- **Temperature Measurement**: Reads temperature data from the BME sensor.
- **Pressure Measurement**: Captures atmospheric pressure using the BME sensor.
- **Humidity Measurement**: Measures the ambient humidity with the BME sensor.
- **Terminal Output**: Displays the collected data on a computer terminal via serial communication.
- **ESP-HAL**: Leverages the `esp-hal` crate from Espressif Systems to interface with the ESP32C6 microcontroller's peripherals.

## Hardware

### Components Used

- **ESP32C6**: The main microcontroller used in the project.
- **BME Sensor**: A sensor for measuring temperature, pressure, and humidity.

### Pin Connections

#### BME Sensor (I2C)

- **VCC** -> 5V
- **GND** -> GND
- **SCK** -> GPIO5
- **SDA** -> GPIO4

## Software

### Dependencies

- **Rust**: The project is written in Rust. Make sure you have Rust installed and set up for embedded development.
- **esp-hal**: The hardware abstraction layer for ESP32 microcontrollers provided by Espressif Systems.

### Project Structure

- **src/main.rs**: The main program logic. Initializes the I2C peripheral, reads data from the BME sensor, and sends it to the terminal.
- **Cargo.toml**: Manages dependencies and project configuration.

### Building and Flashing

1. **Clone the repository**:
    ```bash
    git clone https://github.com/tobiaszfilar/temperature-calc.git
    cd calculate-temperature
    ```

2. **Build the project**:
    ```bash
    cargo build --release
    ```

3. **Flash the firmware to the ESP32C6**:
    ```bash
    cargo run --release
    ```

### Usage

After flashing the firmware, the ESP32C6 will begin reading data from the BME sensor. The temperature, pressure, and humidity values will be sent to the computer terminal via serial communication.

## License

This project is licensed under the MIT License. See the LICENSE-MIT file for more details.

## Acknowledgements

- Thanks to Espressif Systems for the `esp-hal` crate, simplifying hardware interactions.
- Special thanks to the Rust Embedded community for their valuable resources and support.

