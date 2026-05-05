# **WinActivationCheck**

<p align="center">
  <img src="https://img.shields.io/github/v/release/NeetheCheeBao/WinActivationCheck?style=flat-square" alt="release">
  <img src="https://img.shields.io/github/license/NeetheCheeBao/WinActivationCheck?style=flat-square" alt="license">
  <img src="https://img.shields.io/github/stars/NeetheCheeBao/WinActivationCheck?style=flat-square" alt="stars">
  <img src="https://img.shields.io/badge/python-3.6+-blue.svg?style=flat-square" alt="python">
  <img src="https://img.shields.io/badge/platform-windows-lightgrey.svg?style=flat-square" alt="platform">
</p>

### **WinActivationCheck** 是一个轻量级 Windows 激活信息图形化查询工具，基于 Python 和 Tkinter 开发。

通过调用系统内置的 `slmgr.vbs` 脚本，将复杂的命令行输出转化为直观、清晰的窗口界面。

## ✨ 特性介绍
* **多维度查询**：同时展示详细激活信息 (`-dlv`)、基本许可证信息 (`-dli`) 以及过期日期 (`-xpr`)。
* **高 DPI 适配**：适配 Windows 系统缩放，在高分辨率屏幕下保持字体清晰。
* **自动提权**：程序启动时会自动检测并请求管理员权限以执行系统级指令。

![WinActivationCheck Screenshot](IMG/img.png)

## ⬇️ 下载使用

前往 [Releases](https://github.com/NeetheCheeBao/WinActivationCheck/releases)页面下载

## ⚖️ 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。
