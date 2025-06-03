#![feature(error_generic_member_access, downcast_unchecked, never_type)]

extern crate smol;
extern crate thiserror;

pub mod command;
pub mod error;
pub mod event;
pub mod listener;
