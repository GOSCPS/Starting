//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================
// 用来设置GOP

use crate::tool;
use crate::IMAGE_SYSTEM_TABLE;
use core::convert::TryInto;
use uefi::proto::console::gop::BltOp;
use uefi::proto::console::gop::BltPixel;
use uefi::proto::console::gop::GraphicsOutput;

/// 获取gop的服务
unsafe fn get_gop() -> &'static mut GraphicsOutput<'static> {
    // 获取引导服务
    let boot_services = IMAGE_SYSTEM_TABLE.as_ref().unwrap().boot_services();

    // 获取gop服务
    let gop = match boot_services.locate_protocol::<uefi::proto::console::gop::GraphicsOutput>() {
        Ok(some) => some.unwrap().get(),

        Err(_err) => panic!("Load GraphicsOutputProtocol failed down!"),
    };

    crate::tool::print_fmt(format_args!("Get gop done.\n"));

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
                tool::print_fmt(format_args!("Find resolution successfully.\n"));

                // 设置模式
                gop.as_mut().unwrap().set_mode(&mode).unwrap().unwrap();

                tool::print_fmt(format_args!("Set resolution successfully.\n"));

                return Ok(());
            }
        }
    }

    // 失败
    Err(())
}

/// 清空屏幕
///
/// 将会把GOP渲染成黑色。
pub fn clear_framebuffer() -> Result<(), ()> {
    unsafe {
        // 获取gop
        let gop: *mut GraphicsOutput<'static> = get_gop();

        let mode_info = gop.as_mut().unwrap().current_mode_info();

        gop.as_mut().unwrap().blt(
            BltOp::VideoFill{
                color : BltPixel::new(0,0,0),
                dest : (0,0),
                dims : mode_info.resolution()
            }
        ).unwrap().unwrap();
    }

    Ok(())
}
