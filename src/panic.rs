//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================

use crate::tool;
use core::panic::PanicInfo;
use crate::IMAGE_SYSTEM_TABLE;
use uefi::CStr16;
use core::fmt::Write;

/// panic擦屁股函数
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    // 获取系统表
    unsafe {
        let sys = IMAGE_SYSTEM_TABLE.as_ref().unwrap();

        // 报告panic
        sys.stdout().write_str("Starting panic!\n").ok();

        // 打印panic信息
        if let Some(s) = panic_info.payload().downcast_ref::<&'static str>() {
            sys.stdout().write_str(s).ok();
            sys.stdout().write_str("\n").ok();
        } else if let Some(s) = panic_info.payload().downcast_ref::<&CStr16>() {
            sys.stdout().output_string(s).ok();
            sys.stdout().write_str("\n").ok();
        }

        // 打印message
        if let Some(msg) = panic_info.message() {
            tool::print_fmt(*msg);
            tool::print_fmt(format_args!("\n"));
        }

        // 打印panic位置
        if let Some(pos) = panic_info.location() {
            tool::print_fmt(format_args!(
                "At file `{}` lines {}\n",
                pos.file(),
                pos.line()
            ));
        }

        // 警告用户
        tool::print_fmt(format_args!("Starting stop working. Try reboot?\n"));

        loop {}
    }
}
