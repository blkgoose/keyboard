#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use rumcake::{
    bluetooth::BluetoothKeyboard,
    drivers::nrf_ble::central::setup_nrf_ble_split_central,
    hw::platform::{setup_adc_sampler, BluetoothDevice},
    keyboard,
    keyboard::{build_direct_pin_matrix, build_layout, Keyboard, KeyboardLayout, KeyboardMatrix},
    split::central::{CentralDevice, CentralDeviceDriver},
    usb::USBKeyboard,
};

async fn setup_nrf_ble() -> (impl CentralDeviceDriver, &'static [[u8; 6]]) {
    setup_nrf_ble_split_central! {
        peripheral_addresses: [[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]]
    }
}

#[keyboard(
    usb,
    bluetooth,
    split_central(
        driver_setup_fn = setup_nrf_ble,
        driver_type = "nrf-ble"
    )
)]
pub struct GooseLeft;

impl CentralDevice for GooseLeft {
    type Layout = Self;
}

impl Keyboard for GooseLeft {
    const MANUFACTURER: &'static str = "blkgoose";
    const PRODUCT: &'static str = "goose-1";
}

impl USBKeyboard for GooseLeft {
    const USB_VID: u16 = 0x239a;
    const USB_PID: u16 = 0x8029;
}
impl BluetoothKeyboard for GooseLeft {
    const BLE_VID: u16 = 0x666a;
    const BLE_PID: u16 = 0x000a;
}

impl BluetoothDevice for GooseLeft {
    const BLUETOOTH_ADDRESS: [u8; 6] = [0xd3, 0x68, 0x14, 0xc9, 0x65, 0x3b];
}

setup_adc_sampler! {
    (timer: TIMER1, ppi_ch0: PPI_CH0, ppi_ch1: PPI_CH1) => {}
}

impl KeyboardMatrix for GooseLeft {
    type Layout = Self;

    build_direct_pin_matrix! {
        [ P0_02 ]
    }
}

impl KeyboardLayout for GooseLeft {
    build_layout! {
        { [ 1 ] }
    }
}
