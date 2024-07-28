#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]
#![feature(generic_const_exprs)]

// TODO: Edit the `definition.json` file in the `src` folder to match your keyboard.
// _generated.rs is generated at build time, and will contain a serialized version of your Vial definition file to be compiled into your firmware.
// This file won't be generated if you don't specify the "vial" feature flag for rumcake.
#[cfg(vial)]
include!(concat!(env!("OUT_DIR"), "/_generated.rs"));

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyberon;
use rumcake::keyboard;
use rumcake::keyboard::{build_layout, build_standard_matrix, remap_matrix};
use rumcake::keyboard::{KeyboardLayout, KeyboardMatrix};

// #[keyboard(usb)]
// pub struct MyKeyboard;

// use rumcake::keyboard::Keyboard;
// impl Keyboard for MyKeyboard {
//     const MANUFACTURER: &'static str = "Me";
//     const PRODUCT: &'static str = "MyKeyboard";
//     const SERIAL_NUMBER: &'static str = "1";
// }

// impl KeyboardMatrix for MyKeyboard {
//     type Layout = Self;

//     build_standard_matrix! {
//         rows: [ PB2 PB10 PB11 PA3 ],
//         cols: [ PB12 PB1 PB0 PA7 PA6 PA5 PA4 PA2 PB3 PB4 PA15 PB5 ]
//     }
// }

// impl KeyboardLayout for MyKeyboard {
//     build_layout! {
//         {
//             [ Tab    Q  W  E   R      T    Y      U     I   O  P  '['  ]
//             [ LCtrl  A  S  D   F      G    H      J     K   L  ;  '\'' ]
//             [ Escape Z  X  C   V      B    N      M     ,   .  /  ']'  ]
//             [ No     No No (1) LShift LAlt BSpace Space (2) No No No   ]
//         }
//         {
//             [ LGui F1 F2 F3 F4 F5 F6      F7     F8   F9    F10 F11 ]
//             [ t    t  t  t  t  t  Left    Down   Up   Right t   t   ]
//             [ t    t  t  t  t  t  Home    PgDown PgUp End   t   F12 ]
//             [ t    t  t  t  t  t  PScreen Enter  t    t     t   t   ]
//         }
//         {
//             [ t   1 2 3 4 5      6 7 8 9 0    '(' ]
//             [ t   t t t t t      - = t t t    t   ]
//             [ '`' t t t t t      t t t t '\\' ')' ]
//             [ t   t t t t Delete t t t t t    t   ]
//         }
//     }
// }
