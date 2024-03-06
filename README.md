# mmcai_rs

中文版请见[这里](https://github.com/CatMe0w/mmcai_rs/blob/master/README_zh.md)。

_Tee Hee._

Prism Launcher/MultiMC itself does not support authlib-injector (custom/homebrew/alternative/pirate/whatever you call it/... Yggdrasil servers) and officially says it never will. So I made this.

This project is inspired by [mmcai.sh](https://github.com/baobao1270/mmcai.sh), but it only supports Linux and macOS. I want to make it work on Windows.

At the very beginning, I was trying to make a simple PowerShell script to do the same thing. But after wasting many hours on Microsoft's pie in the sky, before my sanity completely drained away, I gave up and decided to join the RIIR army.

Windows, macOS, Linux, all supported.

## How to use

Download mmcai_rs from [here](https://github.com/CatMe0w/mmcai_rs/releases).

Then download authlib-injector from [here](https://github.com/yushijinhun/authlib-injector/releases).

Put `authlib-injector-X.Y.Z.jar` and mmcai_rs in the same directory.

Open Prism Launcher/MultiMC, edit an instance, select "Settings" - "Custom commands" and fill in the "Wrapper command" with the absolute path to mmcai_rs. Then followed it by filling in the username, password, and Yggdrasil API address, separated by spaces.

Example:
```
C:\path\to\mmcai_rs-windows-x86_64.exe PlayerName hunter2 https://www.example.com/api/yggdrasil
```

## License

[MIT License](https://opensource.org/licenses/MIT)

Exception: The file `easteregg.jpg` is all rights reserved. You may not use it without permission. Credits to [ZH9c418](https://github.com/zh9c418) & [瑞狩](https://twitter.com/Ruishou_Nyako).
