//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================

use crate::tool;
use crate::IMAGE_SYSTEM_TABLE;
use core::convert::TryInto;
use uefi::proto::console::gop::GraphicsOutput;

unsafe fn get_gop() -> &'static mut GraphicsOutput<'static> {
    // 获取引导服务
    let boot_services = IMAGE_SYSTEM_TABLE.as_ref().unwrap().boot_services();

    // 获取gop服务
    let gop = match boot_services.locate_protocol::<uefi::proto::console::gop::GraphicsOutput>() {
        Ok(some) => some.unwrap().get(),

        Err(_err) => panic!("Load GraphicsOutputProtocol failed down!"),
    };

    crate::tool::print_fmt(format_args!("Get gop done."));

    gop.as_mut().unwrap()
}

/// 设置显示分辨率
///
/// 如果成功返回true，否则false
pub fn set_video_resolution(
    // 水平分辨率
    horizontal: u64,
    // 垂直分辨率
    vertical: u64,
) -> Result<(), ()> {
    unsafe {
        // 获取引导服务
        let gop: *mut GraphicsOutput<'static> = get_gop();

        // 遍历显示模式
        for mode_completion in gop.as_ref().unwrap().modes() {
            let mode = mode_completion.unwrap();
            let info = mode.info();
            let resolution = info.resolution();

            // 输出分辨率
            tool::print_fmt(format_args!(
                "resolution: {}x{} pixel format:{:?}\n",
                resolution.0,
                 resolution.1,
                 info.pixel_format()
            ));

            // 检查分辨率
            // 必须为RGB格式
            if resolution.0 == horizontal.try_into().unwrap()
                && resolution.1 == vertical.try_into().unwrap()
            {
                tool::print_fmt(format_args!("Find resolution successfully."));

                // 设置模式
                gop.as_mut().unwrap().set_mode(&mode).unwrap().unwrap();

                tool::print_fmt(format_args!("Set resolution successfully."));

                return Ok(());
            }
        }
    }

    // 失败
    Err(())
}

/// 测试GOP的函数
///
/// 将会把GOP渲染成白色。
pub fn test_gop() -> Result<(), ()> {
    unsafe {
        // 获取gop
        let gop: *mut GraphicsOutput<'static> = get_gop();

        let mut frame = gop.as_mut().unwrap().frame_buffer();

        // 红色
        let color = 0x00ff0000u32.to_ne_bytes();

        // 写入像素
        for y in 0..768 {
            for x in 0..1024 {
                frame.write_value(x * 4 + (y * 4 * 1024), color);
            }
        }
    }

    Ok(())
}
