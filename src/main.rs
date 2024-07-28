#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyboard;
use rumcake::keyboard::build_direct_pin_matrix;
use rumcake::keyboard::build_layout;
use rumcake::keyboard::Keyboard;
use rumcake::keyboard::KeyboardLayout;
use rumcake::keyboard::KeyboardMatrix;
use rumcake::usb::USBKeyboard;

#[keyboard(usb)]
pub struct MyKeyboard;

impl Keyboard for MyKeyboard {
    const MANUFACTURER: &'static str = "Stronzo";

    const PRODUCT: &'static str = "Stronzo";
}

impl USBKeyboard for MyKeyboard {
    const USB_VID: u16 = 0x2E8A;
    const USB_PID: u16 = 0x0003;
}

impl KeyboardMatrix for MyKeyboard {
    type Layout = Self;

    build_direct_pin_matrix! { [ PIN_9 ] }
}

impl KeyboardLayout for MyKeyboard {
    build_layout! {
        { [ 1 ] }
    }
}
