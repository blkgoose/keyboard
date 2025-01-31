#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyboard;
use rumcake::keyboard::build_standard_matrix;

#[keyboard(split_peripheral(driver_setup_fn = setup_nrf_ble, driver_type = "nrf-ble"))]
pub struct GooseRight;

use rumcake::keyboard::Keyboard;
impl Keyboard for GooseRight {
    // Needed for advertising data (Bluetooth GAP)
    const MANUFACTURER: &'static str = ""; // TODO: Change this
    const PRODUCT: &'static str = "{{ keyboard-name }}";
}

// Since this is a peripheral device, this only needs a matrix
use rumcake::keyboard::KeyboardMatrix;
impl KeyboardMatrix for GooseRight {
    type PeripheralDeviceType = Self; // send matrix events to the central device via the peripheral device's driver

    build_standard_matrix! {
        rows: [ P0_22 P1_00 P0_11 P1_04 ], // Rows
        cols: [ P0_09 P0_10 P1_11 P1_13 P1_15 P0_02 ] // Columns
    }

    fn remap_to_layout(row: u8, col: u8) -> (u8, u8) {
        // Since the layout is stored on the central device, we need to remap the matrix events
        // to the proper coordinates on the layout

        (row, 6 + col)
    }
}

// Bluetooth configuration
use rumcake::hw::platform::BluetoothDevice;
impl BluetoothDevice for GooseRight {
    const BLUETOOTH_ADDRESS: [u8; 6] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // TODO: Change this
}

// ADC setup required for battery level detection
use rumcake::hw::platform::setup_adc_sampler;
setup_adc_sampler! {
    (timer: TIMER1, ppi_ch0: PPI_CH0, ppi_ch1: PPI_CH1) => {}
}

// Split keyboard setup
use ::rumcake::split::peripheral::{PeripheralDevice, PeripheralDeviceDriver};
use rumcake::drivers::nrf_ble::peripheral::setup_nrf_ble_split_peripheral;
impl PeripheralDevice for GooseRight {}
async fn setup_nrf_ble() -> (impl PeripheralDeviceDriver, [u8; 6]) {
    setup_nrf_ble_split_peripheral! {
        central_address: [1, 1, 1, 1, 1, 1], // TODO: Change this, must match the left half's BLUETOOTH_ADDRESS
    }
}
