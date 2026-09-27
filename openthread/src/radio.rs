//! IEEE 802.15.4 PHY Radio trait and associated types for OpenThread.
//!
//! `openthread` operates the radio in terms of this trait, which is implemented by the actual radio driver.

#![allow(clippy::unnecessary_cast)]

pub use openthread_radio::*;

pub use mac::*;
pub use proxy::*;

// Concrete [`Radio`] implementations for the supported radio hardware /
// deployments. Each is gated on the feature that enables it.
//
// - `esp` / `nrf`: the 802.15.4 radio is local to this MCU (SoC deployment).
// - `spinel`: the radio lives on a *separate* chip (an OpenThread RCP) reached
//   over a UART/SPI spinel link — an RCP-host deployment. (The feature is named
//   `rcp` after that deployment role; the module is named `spinel` after the
//   wire protocol it speaks.)
#[cfg(feature = "esp-radio")]
pub mod esp;
mod mac;
#[cfg(feature = "embassy-nrf")]
pub mod nrf;
mod proxy;
#[cfg(feature = "rcp")]
pub mod spinel;

// The trait crate spells OpenThread's radio constants without the C
// bindings; they must be the bindings' values.
const _: () = {
    use crate::sys;
    core::assert!(caps::OT_RADIO_CAPS_ACK_TIMEOUT as u32 == sys::OT_RADIO_CAPS_ACK_TIMEOUT as u32);
    core::assert!(caps::OT_RADIO_CAPS_ENERGY_SCAN as u32 == sys::OT_RADIO_CAPS_ENERGY_SCAN as u32);
    core::assert!(
        caps::OT_RADIO_CAPS_TRANSMIT_RETRIES as u32 == sys::OT_RADIO_CAPS_TRANSMIT_RETRIES as u32
    );
    core::assert!(
        caps::OT_RADIO_CAPS_CSMA_BACKOFF as u32 == sys::OT_RADIO_CAPS_CSMA_BACKOFF as u32
    );
    core::assert!(caps::OT_RADIO_CAPS_SLEEP_TO_TX as u32 == sys::OT_RADIO_CAPS_SLEEP_TO_TX as u32);
    core::assert!(
        caps::OT_RADIO_CAPS_TRANSMIT_SEC as u32 == sys::OT_RADIO_CAPS_TRANSMIT_SEC as u32
    );
    core::assert!(
        caps::OT_RADIO_CAPS_TRANSMIT_TIMING as u32 == sys::OT_RADIO_CAPS_TRANSMIT_TIMING as u32
    );
    core::assert!(
        caps::OT_RADIO_CAPS_RECEIVE_TIMING as u32 == sys::OT_RADIO_CAPS_RECEIVE_TIMING as u32
    );
    core::assert!(
        caps::OT_RADIO_CAPS_RX_ON_WHEN_IDLE as u32 == sys::OT_RADIO_CAPS_RX_ON_WHEN_IDLE as u32
    );
    core::assert!(
        caps::OT_RADIO_CAPS_TRANSMIT_FRAME_POWER as u32
            == sys::OT_RADIO_CAPS_TRANSMIT_FRAME_POWER as u32
    );
    core::assert!(
        caps::OT_RADIO_CAPS_ALT_SHORT_ADDR as u32 == sys::OT_RADIO_CAPS_ALT_SHORT_ADDR as u32
    );
    core::assert!(RSSI_INVALID as i32 == sys::OT_RADIO_RSSI_INVALID as i32);
};
