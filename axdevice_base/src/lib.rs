#![no_std]

extern crate alloc;

#[macro_use]
extern crate log;

use axerrno::AxResult;
use memory_addr::{AddrRange,GuestPhysAddr};

// TODO: support vgicv2
// pub(crate) mod emu_vgicdv2;
mod emu_type;
// pub use emu_config_notuse::EmulatedDeviceConfig;
pub use emu_type::EmuDeviceType;

pub trait BaseDeviceOps {
    fn emu_type(&self) -> EmuDeviceType;
    fn address_range(&self) -> AddrRange<GuestPhysAddr>;
    fn handle_read(&self, addr: GuestPhysAddr, width: usize) -> AxResult<usize>;
    fn handle_write(&self, addr: GuestPhysAddr, width: usize, val: usize);
}
