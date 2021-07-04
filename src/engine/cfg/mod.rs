//=============================================
// Using People License
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=============================================
// 用来操作配置文件

/// 解析配置文件需要实现的接口
/// 所有函数以panic!("STR");报告错误
pub trait ConfigParser {
    /// 解析配置文件
    fn parse(&self, strs: &str);

    /// 获取字符串值
    fn get_string(&self, key: &str) -> &str;

    /// 获取数字值
    fn get_number(&self, key: &str) -> i64;

    /// 获取bool键值
    fn get_bool(&self, key: &str) -> bool;

    /// 获取object
    fn get_object(&self, key: &str) -> dyn ConfigParser;

    /// 是否包含某一键
    fn contain_key(&self, key: &str) -> bool;
}
