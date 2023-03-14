use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;

use generic_array::GenericArray;
// use heapless::consts::*;

use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::wifi_prelude::rpcs as rpc;
use wio::wifi_prelude::*;
use wio::wifi_types::IPInfo;
use wio::wifi_types::Security;
use wio::{wifi_singleton, WifiPins};

wifi_singleton!(WIFI);

// struct WifiDevice {
//     wifi: Wifi,
// }

// impl WifiDevice {
//     fn new(wifi: Wifi) -> Self {
//         Self { wifi }
//     }
// }

/// Initializes WIFI singleton.
///
/// # Errors
/// If wifi_init or interruption enabling does something not nice.
pub fn init(
    sets_wifi: WifiPins,
    sercom0: SERCOM0,
    clocks: &mut GenericClockController,
    mclk: &mut MCLK,
    delay: &mut Delay,
    nvic: &mut NVIC,
) {
    disable_interrupts(|cs| unsafe {
        wifi_init(cs, sets_wifi, sercom0, clocks, mclk, delay);

        if let Some(wifi) = WIFI.as_mut() {
            wifi.enable(cs, nvic);
        }
    });
}

use seeed_erpc;
use seeed_erpc::{rpcs::ScanResult, L3Interface, WifiMode};

pub fn connect_to_ap(
    delay: &mut Delay,
    ssid: &str,
    pw: &str,
    security: Security,
) -> Result<IPInfo, seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.connect_to_ap(delay, ssid, pw, security))
            .unwrap()
    }
}

pub fn adapter_init() -> Result<(), seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::AdapterInit {}))
            .unwrap()
    }
}

pub fn dhcp_client_start(l3_interface: L3Interface) -> Result<i32, seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| {
                wifi.blocking_rpc(rpc::DHCPClientStart {
                    interface: l3_interface,
                })
            })
            .unwrap()
    }
}

pub fn dhcp_client_stop(l3_interface: L3Interface) -> Result<i32, seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| {
                wifi.blocking_rpc(rpc::DHCPClientStop {
                    interface: l3_interface,
                })
            })
            .unwrap()
    }
}

pub fn get_ip_info(l3_interface: L3Interface) -> Result<IPInfo, seeed_erpc::Err<i32>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| {
                wifi.blocking_rpc(rpc::GetIPInfo {
                    interface: l3_interface,
                })
            })
            .unwrap()
    }
}

use heapless::consts::*;
pub fn get_mac_address() -> Result<heapless::String<U18>, seeed_erpc::Err<i32>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::GetMacAddress {}))
            .unwrap()
    }
}

pub fn get_version() -> Result<heapless::String<U16>, seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::GetVersion {}))
            .unwrap()
    }
}

pub fn is_scanning() -> Result<bool, seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::IsScanning {}))
            .unwrap()
    }
}

pub fn scan_get_num_aps() -> Result<u16, seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::ScanGetNumAPs {}))
            .unwrap()
    }
}

pub fn scan_start() -> Result<i32, seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::ScanStart {}))
            .unwrap()
    }
}

pub fn wifi_connect(ssid: &str, pw: &str, security: Security) -> Result<i32, seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| {
                wifi.blocking_rpc(rpc::WifiConnect {
                    ssid: ssid.into(),
                    password: pw.into(),
                    security,
                    semaphore: 0,
                })
            })
            .unwrap()
    }
}

pub fn wifi_off() -> Result<i32, seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::WifiOff {}))
            .unwrap()
    }
}

pub fn wifi_on(wifi_mode: WifiMode) -> Result<i32, seeed_erpc::Err<()>> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| wifi.blocking_rpc(rpc::WifiOn { mode: wifi_mode }))
            .unwrap()
    }
}

pub fn scan_get_ap() -> Result<
    (
        GenericArray<ScanResult, generic_array::typenum::consts::U16>,
        i32,
    ),
    seeed_erpc::Err<usize>,
> {
    unsafe {
        WIFI.as_mut()
            .map(|wifi| {
                wifi.blocking_rpc(rpc::ScanGetAP::<generic_array::typenum::consts::U16>::new())
            })
            .unwrap()
    }
}
