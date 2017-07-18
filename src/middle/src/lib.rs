#![feature(concat_idents)]
#![feature(conservative_impl_trait)]

extern crate vk_middle_sys as sys;

#[macro_use] extern crate log;
#[macro_use] extern crate bitflags;

pub mod icd;
