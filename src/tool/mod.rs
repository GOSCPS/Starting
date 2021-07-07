//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================
use crate::IMAGE_SYSTEM_TABLE;
use core::fmt::Arguments;
use core::fmt::Write;

pub fn print_fmt(args: Arguments<'_>) {
    unsafe {
        IMAGE_SYSTEM_TABLE
            .as_mut()
            .unwrap()
            .stdout()
            .write_fmt(args)
            .ok();
    }
}
