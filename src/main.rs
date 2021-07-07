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

// 启用alloc
extern crate alloc;

// 载入定义
use uefi::prelude::*;
use uuid::Uuid;

// 模块
mod engine;
mod panic;
mod tool;

// 全局系统表
pub static mut IMAGE_HANDLE: Option<Handle> = None;
pub static mut IMAGE_SYSTEM_TABLE: Option<SystemTable<Boot>> = None;

/// 入口函数
#[no_mangle]
pub extern "efiapi" fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
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

    // 读取配置文件
    tool::print_fmt(format_args!(
        "Read config from:{}\n",
        engine::fs::CONFIG_PATH
    ));

    let config: engine::cfg::Config =
        serde_json::from_str(&engine::fs::read_file(engine::fs::CONFIG_PATH)).unwrap();

    // 读取完毕
    // 设置gop
    if config.enable_gop {
        match engine::gop::set_video_resolution(config.gop_x, config.gop_y) {
            Ok(_ok) => (),

            Err(_err) => panic!("Set the Graphics Output Protocol resolution failed down!"),
        }

        // 清屏
        engine::gop::clear_framebuffer().unwrap();
    }

    // 准备内核
    let (begin, end) = engine::fs::disk::get_partition(
        Uuid::parse_str(&config.disk_guid).unwrap(),
        Uuid::parse_str(&config.partition_guid).unwrap(),
    );

    tool::print_fmt(format_args!(
        "Begin at lba:{0}; End at lba:{1}\n",
        begin, end
    ));

    loop {}
}
