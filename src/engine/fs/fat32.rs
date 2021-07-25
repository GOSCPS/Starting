//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================
// Fat32 支持

use block_device::BlockDevice;
use alloc::string::String;

/// Fat32支持
#[derive(Debug,Clone,Copy)]
pub struct Fat32{
    /// 文件系统启示lba
    pub begin_lba : u64,
    /// 文件系统结束lba
    pub end_lba : u64
}

impl BlockDevice for Fat32 {
    type Error = String;

    fn read(&self, buf: &mut [u8], address: usize, _number_of_blocks: usize) -> Result<(), Self::Error> {
        unsafe{
        let disk = super::disk::get_disk();
        let media_id = disk.media().media_id();
        

        disk.read_blocks(
            media_id, 
            address as u64,
            buf).unwrap().unwrap();
        }

        Ok(())
    }

    fn write(&self, buf: &[u8], address: usize, number_of_blocks: usize) -> Result<(), Self::Error> {
        unsafe{

        let disk = super::disk::get_disk();
        let media_id = disk.media().media_id();

        disk.write_blocks(
            media_id,
            address as u64, 
            buf).unwrap().unwrap();

        }

        Ok(())
    }
}