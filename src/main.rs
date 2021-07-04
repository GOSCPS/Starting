//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================

// 禁用自带的东西
#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use core::panic::PanicInfo;

extern crate alloc;

// 载入uefi定义
use core::fmt::Write;
use uefi::prelude::*;
use uefi::CStr16;

// 模块
mod engine;
mod tool;

// 引入memcpy等函数

// 全局系统表
pub static mut IMAGE_HANDLE: Option<Handle> = None;
pub static mut IMAGE_SYSTEM_TABLE: Option<SystemTable<Boot>> = None;

/// 入口函数
#[no_mangle]
pub extern "C" fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    // 初始化全局变量
    unsafe {
        IMAGE_HANDLE = Some(handle);
        IMAGE_SYSTEM_TABLE = Some(system_table);

        // 打印版本信息
        tool::print_fmt(format_args!(
            "Starting version {}\n",
            env!("CARGO_PKG_VERSION")
        ));
    }

    // 初始化
    engine::init_system();
    tool::print_fmt(format_args!("Init boot system done.\n"));

    // 设置分辨率
    // 1024 * 768
    match engine::gop::set_video_resolution(1024, 768) {
        Ok(ok) => ok,

        Err(_err) => panic!("Couldn't set the video resolution!"),
    }

    // 测试gop
    match engine::gop::clear_framebuffer() {
        Ok(ok) => ok,

        Err(_err) => panic!("Couldn't test the gop!"),
    }

    loop {}
}

/// panic擦屁股函数
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    // 获取系统表
    unsafe {
        let sys = IMAGE_SYSTEM_TABLE.as_ref().unwrap();

        // 报告panic
        sys.stdout().write_str("Starting panic!\n").unwrap();

        // 打印panic信息
        if let Some(s) = panic_info.payload().downcast_ref::<&'static str>() {
            sys.stdout().write_str(s).unwrap();
            sys.stdout().write_str("\n").unwrap();
        } else if let Some(s) = panic_info.payload().downcast_ref::<&CStr16>() {
            sys.stdout().output_string(s).unwrap().unwrap();
            sys.stdout().write_str("\n").unwrap();
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
