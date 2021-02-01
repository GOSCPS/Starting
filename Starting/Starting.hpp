/* * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 * 这个文件来自 GOSCPS(https://github.com/GOSCPS)
 * 使用 GOSCPS 许可证
 * File:    Starting.h
 * Content: Starting Main Head File
 * Copyright (c) 2020 GOSCPS 保留所有权利.
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

#pragma once

extern "C"{
#include <efi.h>
#include <efilib.h>
}

#define G_EFI_ERROR_RETURN(s,r) { if(EFI_ERROR(s)) {return r;} }

//定义各种数据
using uint8_t = UINT8;
using int8_t = INT8;
using uint16_t = UINT16;
using int16_t = INT16;
using uint32_t = UINT32;
using int32_t = INT32;
using uint64_t = UINT64;
using int64_t = INT64;
using uintn = UINTN;

namespace goscps{
//UEFI最后错误
static EFI_STATUS LastError = EFI_SUCCESS;

//UEFI Image句柄
static EFI_HANDLE *GolImageHandles = nullptr;

//内存信息
struct MemMap{
    uintn MemMapSize = 0;
	EFI_MEMORY_DESCRIPTOR *MemMap = nullptr;
	uintn MemKey = 0;
	uintn DESCRIPTOR_Size = 0;
	uint32_t des_version = 0;
};

using MemMap = struct MemMap;

//显示模式
struct VideoMode{
	uint64_t width = 0;
	uint64_t high = 0;
	//模式号
	uint64_t mode = 0;
};

using VideoMode = struct VideoMode;

//读取文件
char *ReadFromFile(const char16_t *Path);
//获取内存信息
MemMap GetMemMap();
//获取内存键值
uintn GetMemKey();

}