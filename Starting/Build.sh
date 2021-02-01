#!/bin/bash

# 此文件需要独家定制

clang++ \
-I/home/chhdao/WorkSpace/GnuEfi/gnu-efi/inc/ \
-I/home/chhdao/WorkSpace/GnuEfi/gnu-efi/inc/x86_64/ \
-fpic  \
-ffreestanding \
-fno-stack-protector \
-fno-stack-check \
-fshort-wchar \
-mno-red-zone \
-c StartingMain.cpp \
-o Starting.o

ld.lld \
-shared \
-Bsymbolic \
-L /home/chhdao/WorkSpace/GnuEfi/gnu-efi/gnuefi/ \
-T /home/chhdao/WorkSpace/GnuEfi/gnu-efi/gnuefi/elf_x86_64_efi.lds \
/home/chhdao/WorkSpace/GnuEfi/gnu-efi/x86_64/gnuefi/crt0-efi-x86_64.o \
Starting.o \
-o Starting.so \
-nostdlib \
-znocombreloc \
-l:libgnuefi.a \
-l:libefi.a \
-L /home/chhdao/WorkSpace/GnuEfi/gnu-efi/x86_64/lib \
-L /home/chhdao/WorkSpace/GnuEfi/gnu-efi/x86_64/gnuefi


objcopy \
-j .text \
-j .sdata \
-j .data \
-j .dynamic \
-j .dynsym  \
-j .rel \
-j .rela \
-j .rel.* \
-j .rela.* \
-j .reloc \
--target efi-app-x86_64 \
--subsystem=10 \
Starting.so Starting.efi