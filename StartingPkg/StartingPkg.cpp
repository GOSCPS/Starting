/* * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 * 这个文件来自 GOSCPS(https://github.com/GOSCPS)
 * 使用 GOSCPS 许可证
 * File:    StartingPkg.cpp
 * Content: StartingPkg Main Source File
 * Copyright (c) 2020 GOSCPS 保留所有权利.
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

#include "StartingPkg.hpp"

//读取配置文件
char *goscps::ReadFromFile(const char16_t *Path){
	/*打开文件*/
	EFI_SIMPLE_FILE_SYSTEM_PROTOCOL *SimpleFileSystem = nullptr;
	EFI_FILE_PROTOCOL *Root = nullptr;

	LastError = gBS->LocateProtocol(&gEfiSimpleFileSystemProtocolGuid,nullptr,(void**)SimpleFileSystem);
	G_EFI_ERROR_RETURN(LastError,nullptr);

	LastError = SimpleFileSystem->OpenVolume(SimpleFileSystem,&Root);
	G_EFI_ERROR_RETURN(LastError,nullptr);

	//读取文件
	{
		EFI_FILE_PROTOCOL *Files = nullptr;

		LastError = Root->Open(Root,&Files,(CHAR16*)Path,EFI_FILE_MODE_READ,0);
		G_EFI_ERROR_RETURN(LastError,nullptr);

		char *Datas = nullptr;

		EFI_FILE_INFO *FileInfo;
		//sizeof(CHAR16) * 64 将储存文件名称
		uintn BufferSize = BufferSize = sizeof(EFI_FILE_INFO) + sizeof(CHAR16) * 64;

		LastError = gBS->AllocatePool(EfiLoaderData,BufferSize,(void**)&FileInfo);
		G_EFI_ERROR_RETURN(LastError,nullptr);

		LastError = Files->GetInfo(Files,&gEfiFileInfoGuid,&BufferSize,FileInfo);
		G_EFI_ERROR_RETURN(LastError,nullptr);

		//分配内存
		//Size+1用于储存0
		LastError = gBS->AllocatePool(EfiLoaderData,FileInfo->FileSize + 1,(void**)&Datas);
		G_EFI_ERROR_RETURN(LastError,nullptr);

		//读取
		{
			uintn bs = FileInfo->FileSize;
			LastError = Files->Read(Files,&bs,(void*)Datas);
			G_EFI_ERROR_RETURN(LastError,nullptr);

			//末尾设置0
			Datas[bs] = '\0';
		}

		LastError = Files->Close(Files);
		G_EFI_ERROR_RETURN(LastError,nullptr);

		LastError = gBS->CloseProtocol(SimpleFileSystem,&gEfiSimpleFileSystemProtocolGuid,GolImageHandles,nullptr);
		G_EFI_ERROR_RETURN(LastError,nullptr);

		return Datas;
	}

	return nullptr;
}

//获取内存数据
goscps::MemMap goscps::GetMemMap(){
	//准备数据
	uintn MemMapSize = 0;
	EFI_MEMORY_DESCRIPTOR *MemMap = nullptr;
	uintn MemKey = 0;
	uintn DESCRIPTOR_Size = 0;
	uint32_t des_version = 0;

	//获取数据
	gBS->GetMemoryMap(&MemMapSize,MemMap,&MemKey,&DESCRIPTOR_Size,&des_version);
	gBS->AllocatePool(EfiLoaderData,MemMapSize,(void**)MemMap);
	gBS->GetMemoryMap(&MemMapSize,MemMap,&MemKey,&DESCRIPTOR_Size,&des_version);

	goscps::MemMap m;

	m.des_version = des_version;
	m.DESCRIPTOR_Size = DESCRIPTOR_Size;
	m.MemKey = MemKey;
	m.MemMap = MemMap;
	m.MemMapSize = MemMapSize;

	return m;
}

//获取键值
uintn goscps::GetMemKey(){
	//准备数据
	uintn MemMapSize = 0;
	EFI_MEMORY_DESCRIPTOR *MemMap = nullptr;
	uintn MemKey = 0;
	uintn DESCRIPTOR_Size = 0;
	uint32_t des_version = 0;

	//获取数据
	gBS->GetMemoryMap(&MemMapSize,MemMap,&MemKey,&DESCRIPTOR_Size,&des_version);

	return MemKey;
}


using namespace goscps;

extern "C"
EFI_STATUS
EFIAPI
UefiMain(IN EFI_HANDLE ImageHandle,IN EFI_SYSTEM_TABLE *SystemTable) {
	GolImageHandles = &ImageHandle;

	char *Config = ReadFromFile(u"/Starting/Starting.config");

	if(Config == nullptr){
		Print((CHAR16*)u"Error:Can't Read The Config File:Error Code %ld\n",LastError);
		return LastError;
	}
	
	char *a = nullptr;
	uint64_t index = 0;
	while(true){
		a = Config + index;
		index++;

		if(a == '\0')
		break;

		Print((CHAR16*)u"%c",a);
	}
	Print((CHAR16*)u"\n");

	auto map = goscps::GetMemKey();
	gBS->ExitBootServices(ImageHandle,map);
	return LastError;
}