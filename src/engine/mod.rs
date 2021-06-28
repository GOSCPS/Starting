//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================

pub mod gop;

use crate::IMAGE_SYSTEM_TABLE;

/// 重置看门狗
pub fn set_watchdog_timer() {
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
