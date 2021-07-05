//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================
// 配置文件

use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "EnableGop")]
    pub enable_gop: bool,

    #[serde(rename = "GopX")]
    pub gop_x: usize,

    #[serde(rename = "GopY")]
    pub gop_y: usize,

    #[serde(rename = "DiskGUUID")]
    pub disk_guid: String,

    #[serde(rename = "PartitionGUUID")]
    pub partition_guid: String,
}
