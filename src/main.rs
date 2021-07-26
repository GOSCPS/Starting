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
use alloc::vec;
use fat32::volume::Volume;
use object::{Object, ObjectSegment};

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
    }
    // 打印版本信息
    tool::print_fmt(format_args!(
        "Starting version {}\n",
        env!("CARGO_PKG_VERSION")
    ));

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
        engine::gop::set_video_resolution(config.gop_x, config.gop_y)
            .expect("Set the Graphics Output Protocol resolution failed down!");

        // 清屏
        engine::gop::clear_framebuffer().unwrap();
    }

    // 准备分区
    let (begin, end) = engine::fs::disk::get_partition(
        Uuid::parse_str(&config.disk_guid).unwrap(),
        Uuid::parse_str(&config.partition_guid).unwrap(),
    );

    tool::print_fmt(format_args!(
        "Begin at lba:{0}; End at lba:{1}\n",
        begin, end
    ));

    // 检查文件系统
    if config.file_system.trim().to_uppercase() != "FAT32"{
        panic!("File system not supported!");
    }

    let fat32;
    // 加载文件系统
    fat32 = engine::fs::fat32::Fat32{
        begin_lba : begin,
        end_lba   : end
    };

    // 加载根目录
    let partition = Volume::new(fat32);
    let root = partition.root_dir();

    // 读取内核
    let file = root.open_file(&config.kernel_path).unwrap();

    let mut kernel_buffer = vec![0u8;config.kernel_length];
    file.read(&mut kernel_buffer).unwrap();

    // 解析ELF
    let kernel = &*kernel_buffer.into_boxed_slice();
    let obj_file = object::File::parse(kernel).unwrap();

    // 内核
    // 在文件中的起始地址和结束地址
    let mut kernel_range : Option<(u64,u64)> = None;

    for segment in obj_file.segments() {
        if segment.name().unwrap_or(Some("ERROR SEGMENTS NAME")).unwrap_or("ERROR SEGMENTS NAME") == "kernel_start"{
            kernel_range = Some(segment.file_range());
        }
    }

    // 检查段
    if kernel_range.is_none(){
        panic!("Can't find kernel_start segment!");
    }

    // 退出UEFI boot服务
    unsafe{
        let memory_map = &mut *(&*engine::get_memory_map() as *const [u8] as *mut [u8]);    

        IMAGE_SYSTEM_TABLE.take().unwrap()
        .exit_boot_services(
            IMAGE_HANDLE.unwrap(),
            memory_map)
        .unwrap()
        .unwrap();
    }

    // 进入内核
    unsafe{
        let kernel_start : *mut fn() -> !;

        kernel_start = ((kernel.as_ptr() as usize) + kernel_range.unwrap().0 as usize)as *mut fn() -> !;

        (*kernel_start)();
    }
}
