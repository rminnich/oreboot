#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![deny(warnings)]

//use clock::ClockNode;
use core::intrinsics::transmute;
use core::panic::PanicInfo;
use core::sync::atomic::{spin_loop_hint, AtomicUsize, Ordering};
//use core::{fmt::Write, ptr};
//use core::{ptr};
//use device_tree::print_fdt;
//use model::Driver;
use payloads::payload;
//use soc::clock::Clock;
//use soc::ddr::DDR;
//use soc::is_qemu;
//use spi::SiFiveSpi;
//use uart::sifive::SiFive;
//use wrappers::{Memory, SectionReader, SliceReader};

//global_asm!(include_str!("../../../../../src/soc/starfive/vic7100/src/bootblock.S"));
//global_asm!(include_str!("../../../../../src/soc/starfive/vic7100/src/init.S"));

// TODO: For some reason, on hardware, a1 is not the address of the dtb, so we hard-code the device
// tree here. TODO: The kernel ebreaks when given this device tree.
//const DTB: &'static [u8] = include_bytes!("hifive.dtb");

// All the non-boot harts spin on this lock.
static SPIN_LOCK: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn _start_nonboot_hart(hart_id: usize, _fdt_address: usize) -> ! {
    spin_loop_hint();
    loop {
        // NOPs prevent thrashing the bus.
        for _ in 0..128 {
            arch::nop();
        }
        match SPIN_LOCK.load(Ordering::Relaxed) {
            0 => {}
            entrypoint => unsafe {
                let entrypoint = transmute::<usize, payload::EntryPoint>(entrypoint);
                // TODO: fdt_address might different from boot hart
                entrypoint(hart_id, _fdt_address);
                // TODO: panic if returned from entrypoint
            },
        };
    }
}

#[no_mangle]
pub extern "C" fn _start_boot_hart(_hart_id: usize, _fdt_address: usize) -> ! {
    arch::halt()
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    arch::halt()
}
