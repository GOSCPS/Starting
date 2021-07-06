//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================
// 用来操作文件系统

pub mod disk;

use crate::IMAGE_SYSTEM_TABLE;
use alloc::string::String;
use alloc::vec;
use core::convert::TryInto;
use uefi::proto::media::file::{Directory, File, FileAttribute, FileInfo, FileMode, FileType};
use uefi::proto::media::fs::SimpleFileSystem;

/// 读取文件
pub fn read_file(path: &str) -> String {
    unsafe {
        // 获取引导服务
        let boot_services = IMAGE_SYSTEM_TABLE.as_mut().unwrap().boot_services();

        // 获取fs服务
        let fs = match boot_services.locate_protocol::<SimpleFileSystem>() {
            Ok(some) => some.unwrap().get(),

            Err(_err) => panic!("Load SimpleFileSystem failed down!"),
        };

        crate::tool::print_fmt(format_args!("Get fs done.\n"));

        // 打开文件
        let mut dic: Directory = fs.as_mut().unwrap().open_volume().unwrap().unwrap();

        let file = dic
            .open(path, FileMode::Read, FileAttribute::empty())
            .unwrap()
            .unwrap();

        // 读取文件
        if let FileType::Regular(mut reader) = file.into_type().unwrap().unwrap() {
            // 获取文件大小
            // 设置缓冲区...
            // 应该不会溢出
            let mut info_buffer: [u8; 512] = [0u8; 512];
            let file_size = reader
                .get_info::<FileInfo>(&mut info_buffer)
                .unwrap()
                .unwrap()
                .file_size();

            // 读进缓冲区
            let mut text_buffer = vec![0u8; file_size.try_into().unwrap()].into_boxed_slice();

            reader.read(&mut text_buffer).unwrap().unwrap();

            // 转换为字符串
            String::from_utf8((&text_buffer).to_vec()).unwrap()
        } else {
            panic!("Filed to open file. Look like a Directory");
        }
    }
}

/// 配置文件路径
pub const CONFIG_PATH: &str = "\\starting\\config.json";
