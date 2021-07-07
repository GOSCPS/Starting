//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================
// 用来磁盘分区

use crate::IMAGE_SYSTEM_TABLE;
use alloc::boxed::Box;
use alloc::vec;
use core::convert::TryInto;
use core::convert::TryFrom;
use core::{u128, u32, u64};
use uefi::proto::media::block::BlockIO;
use uuid::Uuid;
use alloc::string::String;
use alloc::vec::Vec;

/// 获取BlockIO Handle
unsafe fn get_disk() -> &'static mut uefi::proto::media::block::BlockIO {
    // 获取引导服务
    let boot_services = IMAGE_SYSTEM_TABLE.as_ref().unwrap().boot_services();

    // 获取BlockIO服务
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
struct PartitionItem {
    // 分区类型
    partition_type: u128,
    // 分区Guid
    partition_guid: u128,
    // lba范围
    begin_lba: u64,
    end_lba: u64,
    // 分区属性
    attributes: u64,
    // 分区名称
    utf16_name: String,
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

    // 解析GPT表头
    Box::new(GptHeaderInfo {
        signed: u64::from_le_bytes(block_buffer[0..8].try_into().unwrap()),
        version: u32::from_le_bytes(block_buffer[8..12].try_into().unwrap()),
        size: u32::from_le_bytes(block_buffer[12..16].try_into().unwrap()),
        crc32: u32::from_le_bytes(block_buffer[16..20].try_into().unwrap()),
        keep: u32::from_le_bytes(block_buffer[20..24].try_into().unwrap()),
        my_lba: u64::from_le_bytes(block_buffer[24..32].try_into().unwrap()),
        backup_lba: u64::from_le_bytes(block_buffer[32..40].try_into().unwrap()),
        partition_begin_lba: u64::from_le_bytes(block_buffer[40..48].try_into().unwrap()),
        partition_end_lba: u64::from_le_bytes(block_buffer[48..56].try_into().unwrap()),
        disk_guid: u128::from_le_bytes(block_buffer[56..72].try_into().unwrap()),
        partition_item_lba: u64::from_le_bytes(block_buffer[72..80].try_into().unwrap()),
        partition_item_count: u32::from_le_bytes(block_buffer[80..84].try_into().unwrap()),
        partition_item_size: u32::from_le_bytes(block_buffer[84..88].try_into().unwrap()),
        partition_item_crc32: u32::from_le_bytes(block_buffer[88..92].try_into().unwrap()),
    })
}

/// 转换字符串所用
fn convert_partition_utf16_name(raw: &[u8;72]) -> [u16; 36] {
    let mut result : [u16; 36] = [0u16;36];
    let mut p = 0;
    let mut p2 = 0;
    while p < raw.len() {
        let mut buf = [0u8; 2];
        buf[0] = raw[p];
        buf[1] = raw[p + 1];
        result[p2] = u16::from_le_bytes(buf);
        p += 2;
        p2 += 1;
    }
    result
}


/// 获取分区表项
unsafe fn get_partition_item(
    header: Box<GptHeaderInfo>,
    disk : &mut BlockIO,
) -> Vec<Box<PartitionItem>> {
    // 检查数据
    if header.partition_item_size != 128 {
        panic!("No support partition item size:{}",header.partition_item_size);
    }

    if header.version != 0x00010000{
        panic!("No support partition item version:{}",header.version);
    }

    // 读取数据
    let block_size = disk.media().block_size();

    // 缓存数据
    let mut current_lba = header.partition_item_lba;
    let mut current_offset : usize = 0;
    
    // 分区
    let mut partitions = Vec::<Box<PartitionItem>>::new();

    // 读取所有分区
    while partitions.len() != usize::try_from(header.partition_item_count).unwrap(){
        // 读取完一个扇区，切换到下一个
        if current_offset == usize::try_from(block_size).unwrap(){
            current_offset = 0;
            current_lba = current_lba+1;
        }

        // 缓冲区
        let mut block_buffer = vec![0u8; block_size.try_into().unwrap()].into_boxed_slice();
        
        // 读取数据
        disk.read_blocks(disk.media().media_id(), current_lba, &mut block_buffer)
        .unwrap()
        .unwrap();

        // 检索数据
        partitions.push(Box::new(PartitionItem {
            partition_type : u128::from_le_bytes(block_buffer[current_offset..current_offset+16].try_into().unwrap()),
            partition_guid : u128::from_le_bytes(block_buffer[current_offset+16..current_offset+32].try_into().unwrap()),
            begin_lba : u64::from_le_bytes(block_buffer[current_offset+32..current_offset+40].try_into().unwrap()),
            end_lba : u64::from_le_bytes(block_buffer[current_offset+40..current_offset+48].try_into().unwrap()),
            attributes : u64::from_le_bytes(block_buffer[current_offset+48..current_offset+56].try_into().unwrap()),
            utf16_name : String::from_utf16(&convert_partition_utf16_name(block_buffer[current_offset+56..current_offset+128].try_into().unwrap())).unwrap(),
        }));

        // 自增
        current_offset = current_offset + usize::try_from(header.partition_item_size).unwrap();
    }

    partitions
}

/// 根据磁盘Uuid和分区Uuid获取分区范围
pub fn get_partition(disk: Uuid, partition: Uuid) -> (u64, u64) {
    unsafe {
        // 获取磁盘
        let disk: &mut BlockIO = get_disk();

        // 读取表头信息
        let header_info = get_gpt_header(disk);
        crate::tool::print_fmt(format_args!("Get GPT header done.\n"));
        crate::tool::print_fmt(format_args!("GPT header:{0:?}\n", header_info));

        // 获取表项信息
        let item_infos = get_partition_item(header_info,disk);
        crate::tool::print_fmt(format_args!("Get GPT partition items done.\n"));

        for item in item_infos.iter(){
            // 只打印有效分区
            if item.partition_type != 0{
                crate::tool::print_fmt(format_args!("GPT partition:{0:?}\n", item));
            }
        }
    }

    (0, 0)
}
