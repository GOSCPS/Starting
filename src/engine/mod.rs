//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================

pub mod cfg;
pub mod fs;
pub mod gop;
use crate::IMAGE_SYSTEM_TABLE;
use core::alloc::{GlobalAlloc, Layout};
use uefi::table::boot::MemoryType;
use alloc::vec::Vec;

/// UEfi内存分配器
pub struct UefiAllocator;

/// 实现内存分配器
unsafe impl GlobalAlloc for UefiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let boot = IMAGE_SYSTEM_TABLE.as_ref().unwrap().boot_services();

        // 内存类型为LOADER_DATA(即数据)
        boot.allocate_pool(MemoryType::LOADER_DATA, layout.size())
            .unwrap()
            .unwrap()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let boot = IMAGE_SYSTEM_TABLE.as_ref().unwrap().boot_services();

        boot.free_pool(ptr).unwrap().unwrap();
    }
}

/// 设置内存分配器
#[global_allocator]
static GLOBAL: UefiAllocator = UefiAllocator;

/// 设置分配error处理函数
#[alloc_error_handler]
fn alloc_error_catch(layout: core::alloc::Layout) -> ! {
    panic!(
        "Couldn't alloc memory! Size:{0} Align:{1}",
        layout.size(),
        layout.align()
    );
}

/// 初始化系统
pub fn init_system() {
    set_watchdog_timer();
}

/// 重置看门狗
fn set_watchdog_timer() {
    unsafe {
        // 获取引导服务
        let boot_services = IMAGE_SYSTEM_TABLE.as_ref().unwrap().boot_services();

        // 禁用看门狗
        boot_services
            // 0-0xffff 的watchdog为UEFI所保留的code
            .set_watchdog_timer(0, 0xffff + 1, None)
            .unwrap()
            .unwrap();
    }
}

/// 获取UEFI的memory map
pub unsafe fn get_memory_map() -> Vec<u8>{
    let map_size = IMAGE_SYSTEM_TABLE.as_mut().unwrap().boot_services().memory_map_size();

    // Build a buffer bigger enough to handle the memory map
    let mut buffer = Vec::with_capacity(map_size);
        
    buffer.set_len(map_size);

    IMAGE_SYSTEM_TABLE.as_ref().unwrap().boot_services()
    .memory_map(&mut buffer)
    .unwrap()
    .unwrap();

    buffer
}