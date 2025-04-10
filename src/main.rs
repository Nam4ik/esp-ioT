#![no_std]

mod updater
mod wifi
mod core
mod server
mod init 
mod ioT

use ioT::{init, protocol};
use updater::{update, handle};
use init::{initGPIO, init_ioT};
use wifi::connect;
use core::{*}

use core::fmt;


fn main() -> !
{
}