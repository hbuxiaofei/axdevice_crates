#![no_std]

extern crate alloc;

#[macro_use]
extern crate log;

use axerrno::AxResult;
use memory_addr::AddrRange;

// TODO: support vgicv2
// pub(crate) mod emu_vgicdv2;
mod emu_config_notuse;
mod emu_type;
mod emu_vgicdv2;
pub use emu_vgicdv2::EmuVgicdV2;
// pub use emu_config_notuse::EmulatedDeviceConfig;
pub use emu_type::EmuDeviceType;

pub trait BaseDeviceOps {
    fn emu_type(&self) -> EmuDeviceType;
    fn address_range(&self) -> AddrRange<usize>;
    fn handle_read(&self, addr: usize, width: usize) -> AxResult<usize>;
    fn handle_write(&self, addr: usize, width: usize, val: usize);
}
