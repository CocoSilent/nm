# nm

## 简介

使用rust开发的一个node.js版本管理器

## 安装
1，下载nm，放到指定目录如：D:\nm

2，设置环境变量，在path中加入以下两个路径(D:\nm替换为nm所在路径)

    2.1 D:\nm 为了让nm命令找到可执行文件

    2.2 D:\nm\nodejs 为了让node、npm命令找到可执行文件
   

## 使用

- **`nm current`**: 显示当前使用的版本
- **`nm install <version>`**:  安装指定的nodejs版本,版本号支持1位、2位、3位

    nm install 12  将安装nodejs v12最新版本 12.22.12

    nm install 12.19 将安装nodejs v12.19最新版本 12.19.1

    nm install 12.20.2 则直接安装12.20.2
- **`nm list`**: 显示当前安装的所有版本，也可以使用 nm ls
- **`nm uninstall <version>`**: 卸载指定版本，version必须为3位，如nm install 12.22.12,也可使用nm remove
- **`nm use <version>`**: 使用指定版本，version必须为3位，如nm install 12.22.12
- **`nm version`**: 显示当前nm版本号

## 为什么使用rust开发

内存安全，高性能，代码质量可靠，在编译期就能消除各种错误。
