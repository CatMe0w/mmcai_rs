# mmcai_rs

_窃笑一声_

Prism Launcher/MultiMC 本身不支持 authlib-injector (自建/自制/替代/盗版/随你怎么叫/... Yggdrasil 服务器)，并且官方表示永远不会实现。因此自己实现了一个。

本项目受 [mmcai.sh](https://github.com/baobao1270/mmcai.sh) 启发，但它只支持 Linux 和 macOS，而我希望在 Windows 使用。

一开始，我试图用一个简单的 PowerShell 脚本来实现同样的功能。但在微软的空中楼阁上浪费了几个小时后，在我的神智彻底归零之前，我决定打起退堂鼓并加入 RIIR 大军。

Windows、macOS、Linux 全支持。

## 如何使用
从 [这里](https://github.com/CatMe0w/mmcai_rs/releases) 下载 mmcai_rs。

然后从 [这里](https://github.com/yushijinhun/authlib-injector/releases) 下载 authlib-injector。

将 `authlib-injector-X.Y.Z.jar` 和 mmcai_rs 放在同一个目录下。

打开 Prism Launcher/MultiMC，编辑实例，进入 “设置” - “自定义命令”，在 "包装器命令" 中填入 mmcai_rs 的绝对路径。然后在后面填入用户名、密码和 Yggdrasil API 地址，使用空格分隔。

例子：
```
C:\path\to\mmcai_rs-windows-x86_64.exe PlayerName hunter2 https://www.example.com/api/yggdrasil
```

## 开源许可

[MIT License](https://opensource.org/licenses/MIT)

例外：文件 `easteregg.jpg` 版权所有，未经许可不得使用。鸣谢： [ZH9c418](https://github.com/zh9c418) & [瑞狩](https://twitter.com/Ruishou_Nyako)。
