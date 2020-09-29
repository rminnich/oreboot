#![feature(llvm_asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]

use arch::bzimage::BzImage;
use arch::ioport::IOPort;
use arch::bios::setup_bios_tables;
use core::fmt::Write;
use core::panic::PanicInfo;
use model::Driver;
use print;
use uart::i8250::I8250;
mod mainboard;
use mainboard::MainBoard;
mod msr;
use msr::msrs;
use x86_64::instructions::rdmsr;
extern crate heapless; // v0.4.x
use heapless::consts::*;
use heapless::Vec;

use core::ptr;
// Until we are done hacking on this, use our private copy.
// Plan to copy it back later.
global_asm!(include_str!("bootblock.S"));
fn poke32(a: u32, v: u32) -> () {
    let y = a as *mut u32;
    unsafe {
        ptr::write_volatile(y, v);
    }
}

fn peek32(a: u32) -> u32 {
    let y = a as *const u32;
    unsafe { ptr::read_volatile(y) }
}
// extern "C" {
//     fn run32(w: &mut print::WriteTo, start_address: usize, dtb: usize);
// }

fn peek(a: u64) -> u64 {
    let y = a as *const u64;
    unsafe { ptr::read_volatile(y) }
}

fn peekb(a: u64) -> u8 {
    let y = a as *const u8;
    unsafe { ptr::read_volatile(y) }
}

// Returns a slice of u32 for each sequence of hex chars in a.
fn hex(a: &[u8], vals: &mut Vec<u64, U8>) -> () {
    let mut started: bool = false;
    let mut val: u64 = 0u64;
    for c in a.iter() {
        let v = *c;
        if v >= b'0' && v <= b'9' {
            started = true;
            val = val << 4;
            val = val + (*c - b'0') as u64;
        } else if v >= b'a' && v <= b'f' {
            started = true;
            val = (val << 4) | (*c - b'a' + 10) as u64;
        } else if v >= b'A' && v <= b'F' {
            started = true;
            val = (val << 4) | (*c - b'A' + 10) as u64;
        } else if started {
            vals.push(val).unwrap();
            val = 0;
        }
    }
}

fn mem(w: &mut print::WriteTo, a: Vec<u8, U16>) -> () {
    let mut vals: Vec<u64, U8> = Vec::new();
    hex(&a, &mut vals);

    // I wish I knew rust. This code is shit.
    for a in vals.iter() {
        let m = peek(*a);
        write!(w, "{:x?}: {:x?}\r\n", *a, m).unwrap();
    }
}

fn memb(w: &mut print::WriteTo, a: Vec<u8, U16>) -> () {
    let mut vals: Vec<u64, U8> = Vec::new();
    hex(&a, &mut vals);
    write!(w, "dump bytes: {:x?}\r\n", vals).unwrap();
    // I wish I knew rust. This code is shit.
    for a in vals.iter() {
        for i in 0..16 {
            let addr = *a + i;
            let m = peekb(addr);
            write!(w, "{:x?}: {:x?}\r\n", addr, m).unwrap();
        }
    }
}

#[no_mangle]
pub extern "C" fn _asdebug(w: &mut print::WriteTo, a: u64) -> () {
    write!(w, "here we are in asdebug\r\n").unwrap();
    write!(w, "stack is {:x?}\r\n", a).unwrap();
    debug(w);
    write!(w, "back to hell\r\n").unwrap();
}

fn debug(w: &mut print::WriteTo) -> () {
    let mut done: bool = false;
    let newline: [u8; 2] = [10, 13];
    while done == false {
        let io = &mut IOPort;
        let uart0 = &mut I8250::new(0x3f8, 0, io);
        let mut line: Vec<u8, U16> = Vec::new();
        loop {
            let mut c: [u8; 1] = [12; 1];
            uart0.pread(&mut c, 1).unwrap();
            uart0.pwrite(&c, 1).unwrap();
            line.push(c[0]).unwrap();
            if c[0] == 13 || c[0] == 10 || c[0] == 4 {
                uart0.pwrite(&newline, 2).unwrap();
                break;
            }
            if line.len() > 15 {
                break;
            }
        }
        match line[0] {
            0 | 4 => {
                done = true;
            }
            b'm' => {
                mem(w, line);
            }
            b'h' => {
                memb(w, line);
            }
            _ => {}
        }
    }
}
//global_asm!(include_str!("init.S"));

#[no_mangle]
pub extern "C" fn _start(fdt_address: usize) -> ! {
    let m = &mut MainBoard::new();
    m.init().unwrap();
    let io = &mut IOPort;
    let post = &mut IOPort;
    let uart0 = &mut I8250::new(0x3f8, 0, io);
    uart0.init().unwrap();

    for _i in 1..32 {
        uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();
    }
    let mut p: [u8; 1] = [0xf0; 1];
    post.pwrite(&p, 0x80).unwrap();
    let w = &mut print::WriteTo::new(uart0);

    msrs(w);
    // It is hard to say if we need to do this.
    if true {
        let v = rdmsr(0xc001_1004);
        write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
        // it's set already
        //unsafe {wrmsr(0xc001_1004, v | (1 << 9));}
        //let v = rdmsr(0xc001_1004);
        //write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
    }
    if true {
        let v = rdmsr(0xc001_1005);
        write!(w, "c001_1005 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
        // it's set already
        //unsafe {wrmsr(0xc001_1004, v | (1 << 9));}
        //let v = rdmsr(0xc001_1004);
        //write!(w, "c001_1004 is {:x} and APIC is bit {:x}\r\n", v, 1 << 9).unwrap();
    }
        write!(w, "0x1b is {:x} \r\n", rdmsr(0x1b)).unwrap();
    p[0] = p[0] + 1;
    let payload = &mut BzImage {
        low_mem_size: 0x80000000,
        high_mem_start: 0x100000000,
        high_mem_size: 0,
        // TODO: get this from the FDT.
        rom_base: 0xffc00000,
        rom_size: 0x300000,
        load: 0x01000000,
        entry: 0x1000200,
    };
    p[0] = p[0] + 1;
    write!(w, "Write bios tables\r\n").unwrap();
    setup_bios_tables(w, 0xf0000, 1);
    write!(w, "Wrote bios tables, entering debug\r\n").unwrap();
    debug(w);
    write!(w, "LDN is {:x}\r\n", peek32(0xfee000d0)).unwrap();
    poke32(0xfee000d0, 0x1000000);
    write!(w, "LDN is {:x}\r\n", peek32(0xfee000d0)).unwrap();
    write!(w, "loading payload with fdt_address {}\r\n", fdt_address).unwrap();
    post.pwrite(&p, 0x80).unwrap();
    p[0] = p[0] + 1;
    payload.load(w).unwrap();
    post.pwrite(&p, 0x80).unwrap();
    p[0] = p[0] + 1;
    write!(w, "Back from loading payload, call debug\r\n").unwrap();
    debug(w);
    write!(w, "Running payload entry is {:x}\r\n", payload.entry).unwrap();
    post.pwrite(&p, 0x80).unwrap();
    p[0] = p[0] + 1;
    payload.run(w);
    post.pwrite(&p, 0x80).unwrap();
    p[0] = p[0] + 1;

    write!(w, "Unexpected return from payload\r\n").unwrap();
    post.pwrite(&p, 0x80).unwrap();
    p[0] = p[0] + 1;
    arch::halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Assume that uart0.init() has already been called before the panic.
    let io = &mut IOPort;
    let uart0 = &mut I8250::new(0x3f8, 0, io);
    let w = &mut print::WriteTo::new(uart0);
    // Printing in the panic handler is best-effort because we really don't want to invoke the panic
    // handler from inside itself.
    let _ = write!(w, "PANIC: {}\r\n", info);
    arch::halt()
}
