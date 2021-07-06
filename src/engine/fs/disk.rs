//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================
// 用来磁盘分区

use crate::IMAGE_SYSTEM_TABLE;
use alloc::boxed::Box;
use alloc::vec;
use core::convert::TryInto;
use core::{u128, u32, u64};
use uefi::proto::media::block::BlockIO;
use uuid::Uuid;

/// 获取BlockIO Handle
unsafe fn get_disk() -> &'static mut uefi::proto::media::block::BlockIO {
    // 获取引导服务
    let boot_services = IMAGE_SYSTEM_TABLE.as_ref().unwrap().boot_services();

    // 获取gop服务
    let block = match boot_services.locate_protocol::<BlockIO>() {
        Ok(some) => some.unwrap().get(),

        Err(_err) => panic!("Load BlockIO failed down!"),
    };

    crate::tool::print_fmt(format_args!("Get BlockIO done.\n"));

    block.as_mut().unwrap()
}

/// GPT 头信息
///
/// 分区表头
struct GptHeaderInfo {
    // 签名
    signed: u64,
    // 修订/版本号
    version: u32,
    // 大小
    size: u32,
    // 校验
    crc32: u32,
    // 保留 为0
    keep: u32,
    // 当前LBA
    my_lba: u64,
    // 备份Lba
    backup_lba: u64,
    // 第一个可用于分区的lba
    partition_begin_lba: u64,
    // 最后一个可用于分区的lba
    partition_end_lba: u64,
    // 硬盘guid
    disk_guid: u128,
    // 分布表项起始lba
    partition_item_lba: u64,
    // 分区表项的数量
    partition_item_count: u32,
    // 一个分区表项的大小
    partition_item_size: u32,
    // 分区序列的CRC32校验
    partition_item_crc32: u32,
    // 空
}

/// 分区表项
struct PartitionItem {
    // 分区类型
    partition_type: u128,
    // 分区Guid
    partition_guid: u128,
    // lba范围
    begin_lba: u64,
    end_lba: u64,
    // 分区名称
    utf16_name: [u16; 36],
}

/// 获取GPT表头
unsafe fn get_gpt_header(disk: &mut BlockIO) -> Box<GptHeaderInfo> {
    let media = disk.media();

    // 每个磁盘块的大小
    let block_size = media.block_size();

    crate::tool::print_fmt(format_args!("Block size:{0}\n", block_size));

    // 每个磁盘块的缓冲区
    let mut block_buffer = vec![0u8; block_size.try_into().unwrap()].into_boxed_slice();

    // 读取内容
    // GPT表头位于第一LBA
    disk.read_blocks(media.media_id(), 1, &mut block_buffer)
        .unwrap()
        .unwrap();

    // 解析
    Box::new(GptHeaderInfo {
        signed: u64::from_le_bytes(block_buffer[0..7].try_into().unwrap()),
        version: u32::from_le_bytes(block_buffer[8..11].try_into().unwrap()),
        size: u32::from_le_bytes(block_buffer[12..15].try_into().unwrap()),
        crc32: u32::from_le_bytes(block_buffer[16..19].try_into().unwrap()),
        keep: u32::from_le_bytes(block_buffer[20..23].try_into().unwrap()),
        my_lba: u64::from_le_bytes(block_buffer[24..31].try_into().unwrap()),
        backup_lba: u64::from_le_bytes(block_buffer[32..39].try_into().unwrap()),
        partition_begin_lba: u64::from_le_bytes(block_buffer[40..47].try_into().unwrap()),
        partition_end_lba: u64::from_le_bytes(block_buffer[48..55].try_into().unwrap()),
        disk_guid: u128::from_le_bytes(block_buffer[56..71].try_into().unwrap()),
        partition_item_lba: u64::from_le_bytes(block_buffer[72..79].try_into().unwrap()),
        partition_item_count: u32::from_le_bytes(block_buffer[80..83].try_into().unwrap()),
        partition_item_size: u32::from_le_bytes(block_buffer[84..87].try_into().unwrap()),
        partition_item_crc32: u32::from_le_bytes(block_buffer[88..91].try_into().unwrap()),
    })
}

/// 根据磁盘Uuid和分区Uuid获取分区范围
pub fn get_partition(disk: Uuid, partition: Uuid) -> (u64, u64) {
    unsafe {
        // 获取磁盘
        let disk: &mut BlockIO = get_disk();

        // 读取表头信息
        let header_info = get_gpt_header(disk);
        crate::tool::print_fmt(format_args!("Get GPT header done!\n"));

        // DEBUG
        crate::tool::print_fmt(format_args!("Header:{0}\n", header_info.signed));
        crate::tool::print_fmt(format_args!("Version:{0}\n", header_info.version));
        crate::tool::print_fmt(format_args!("Size:{0}\n", header_info.size));
        crate::tool::print_fmt(format_args!("Crc32:{0}\n", header_info.crc32));
        crate::tool::print_fmt(format_args!("keep:{0}\n", header_info.keep));
        crate::tool::print_fmt(format_args!("my lba:{0}\n", header_info.my_lba));
        crate::tool::print_fmt(format_args!("backup lba:{0}\n", header_info.backup_lba));
        crate::tool::print_fmt(format_args!(
            "partition begin lba:{0}\n",
            header_info.partition_begin_lba
        ));
        crate::tool::print_fmt(format_args!(
            "partition end lba:{0}\n",
            header_info.partition_end_lba
        ));
        crate::tool::print_fmt(format_args!("GUID:{0}\n", header_info.disk_guid));
        crate::tool::print_fmt(format_args!(
            "partition item begin lba:{0}\n",
            header_info.partition_item_lba
        ));
        crate::tool::print_fmt(format_args!(
            "partition item count:{0}\n",
            header_info.partition_item_count
        ));
        crate::tool::print_fmt(format_args!(
            "partition item size:{0}\n",
            header_info.partition_item_size
        ));
        crate::tool::print_fmt(format_args!(
            "partition item crc32:{0}\n",
            header_info.partition_item_crc32
        ));
    }

    (0, 0)
}
