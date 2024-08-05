#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use panic_probe as _;

use rumcake::{
    hw::platform::setup_adc_sampler,
    keyberon::{
        action::{
            k,
            Action::{self, *},
            HoldTapAction, HoldTapConfig,
        },
        key_code::KeyCode::*,
    },
    keyboard,
    keyboard::{
        build_direct_pin_matrix, build_layout, Keyboard, KeyboardLayout, KeyboardMatrix, Keycode,
    },
    usb::USBKeyboard,
};

impl KeyboardLayout for GooseLeft {
    build_layout! {
        {
            [ Q  W  E  R  T     ]
            [ {G_A}  {G_S}  {G_D}  {G_F}  G     ]
            [ Z  X  C  V  B     ]
            [ No No No {LAYER_1} Space ]
        }
        {
            [ Y t t t t ]
            [ t t t t t ]
            [ t t t t t ]
            [ t t t {LAYER_0} t ]
        }
    }
}

macro_rules! hold_tap {
    ($hold:expr, $tap:expr) => {
        HoldTap(&HoldTapAction {
            timeout: HOLD_TIME,
            tap_hold_interval: 0,
            config: HoldTapConfig::Default,
            hold: $hold,
            tap: k($tap),
        })
    };
}

const HOLD_TIME: u16 = 200;

const G_A: Action<Keycode> = hold_tap!(k(LCtrl), A);
const G_S: Action<Keycode> = hold_tap!(k(LShift), S);
const G_D: Action<Keycode> = hold_tap!(k(LGui), D);
const G_F: Action<Keycode> = hold_tap!(k(LAlt), F);

const LAYER_0: Action<Keycode> = Action::ToggleLayer(0);
const LAYER_1: Action<Keycode> = Action::ToggleLayer(1);

/*
 *
 * Keyboard configuration
 *
 */

#[keyboard(usb)]
pub struct GooseLeft;

impl Keyboard for GooseLeft {
    const MANUFACTURER: &'static str = "blkgoose";
    const PRODUCT: &'static str = "goose-1";
}

impl USBKeyboard for GooseLeft {
    const USB_VID: u16 = 0x0000;
    const USB_PID: u16 = 0x00b3;
}

setup_adc_sampler! {
    (timer: TIMER1, ppi_ch0: PPI_CH0, ppi_ch1: PPI_CH1) => {}
}

impl KeyboardMatrix for GooseLeft {
    type Layout = Self;

    build_direct_pin_matrix! {
        [ P0_22 P0_20 P0_17 P0_08 P0_06 ]
        [ P1_04 P0_11 P1_00 P0_24 P0_31 ]
        [ P1_15 P1_13 P1_11 P0_10 P0_09 ]
        [ No    No    No    P0_29 P0_02 ]
    }
}
