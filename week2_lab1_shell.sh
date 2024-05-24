#!/bin/bash

set -e

# 编译 Rust 项目并生成二进制文件

# 获取应用程序大小
app1_size=$(stat -c%s "./hello_app/hello_app.bin")
app2_size=0  # 假设没有第二个应用程序，初始为0

# 计算偏移量
header_size=32
app1_offset=$header_size
app2_offset=$(($app1_offset + $app1_size))

# 创建一个空的 apps.bin 文件
dd if=/dev/zero of=./apps.bin bs=1M count=32

# 生成文件头部数据并写入到 apps.bin
{
  printf "%016x" $app1_size | sed 's/../& /g' | tac -rs ' ' | tr -d ' ' | xxd -r -p
  printf "%016x" $app2_size | sed 's/../& /g' | tac -rs ' ' | tr -d ' ' | xxd -r -p
  printf "%016x" $app1_offset | sed 's/../& /g' | tac -rs ' ' | tr -d ' ' | xxd -r -p
  printf "%016x" $app2_offset | sed 's/../& /g' | tac -rs ' ' | tr -d ' ' | xxd -r -p
} | dd of=./apps.bin conv=notrunc

# 将第一个应用程序二进制数据写入到 apps.bin
dd if=./hello_app/hello_app.bin of=./apps.bin bs=1 seek=$app1_offset conv=notrunc
dd if=./ebreak/ebreak.bin of=./apps.bin bs=1 seek=$app2_offset conv=notrunc

# 创建目标目录并移动生成的文件
mkdir -p arceos/payload
mv ./apps.bin arceos/payload/apps.bin

echo "Process completed successfully!"
