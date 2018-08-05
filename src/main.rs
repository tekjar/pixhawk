#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]
 
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate panic_semihosting;

use rt::ExceptionFrame;

entry!(main);
 
fn main() -> ! { 
    loop {}
}
 
exception!(HardFault, hard_fault);
 
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
 
exception!(*, default_handler);
 
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}