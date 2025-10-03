#!/bin/bash

# 循环从0到13，创建对应的文件夹
for i in {0..37}; do
    # 格式化数字为4位，前面补0
    prefix=$(printf "%04d" $((i * 100)))
    suffix=$(printf "%04d" $((i * 100 + 99)))
    # 创建文件夹，名称格式为"xxxx-xxxx"
    folder_name="${prefix}-${suffix}"
    mkdir -p "$folder_name"
    echo "创建文件夹: $folder_name"
done

echo "所有文件夹创建完成！"