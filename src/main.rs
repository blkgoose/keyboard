#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyboard;
use rumcake::keyboard::build_direct_pin_matrix;
use rumcake::keyboard::build_layout;
use rumcake::keyboard::Keyboard as RumcakeKeyboard;
use rumcake::keyboard::KeyboardLayout;
use rumcake::keyboard::KeyboardMatrix;
use rumcake::usb::USBKeyboard;

#[keyboard(usb)]
pub struct Keyboard;

impl RumcakeKeyboard for Keyboard {
    const MANUFACTURER: &'static str = "blkgoose";
    const PRODUCT: &'static str = "keyboard";
}

impl USBKeyboard for Keyboard {
    const USB_VID: u16 = 0x2E8A;
    const USB_PID: u16 = 0x0003;
}

impl KeyboardMatrix for Keyboard {
    type Layout = Self;

    build_direct_pin_matrix! {
        [ PIN_2  PIN_3  PIN_4  PIN_5  PIN_6  ]
        [ PIN_7  PIN_8  PIN_9  PIN_12 PIN_13 ]
        [ PIN_14 PIN_15 PIN_16 PIN_21 PIN_23 ]
        [ No     No     No     PIN_20 PIN_22 ]
    }
}

impl KeyboardLayout for Keyboard {
    build_layout! {
        { [ Q  W  E  R     T    ]
          [ A  S  D  F     G    ]
          [ Z  X  C  V     B    ]
          [ No No No Space LGui ]
        }
    }
}
