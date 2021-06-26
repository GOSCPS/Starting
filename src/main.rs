//=============================================
// Using People License 
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================

// 禁用自带的东西
#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;

// 载入uefi定义
use uefi::prelude::*;
use core::fmt::Write;

/// 入口函数
#[no_mangle]
pub extern "C" fn efi_main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {


        system_table.stdout().write_str("Hello UEFI!\n");

        return Status::SUCCESS;
}

/// panic擦屁股函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}