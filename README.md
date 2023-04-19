
<img align="left" width="100px" src="https://github.com/ftdot/rjoiner/blob/master/icon.ico?raw=true" />
<h1><strong>RJoiner</strong></h1>
<p>Join files to 1 by 1 command!</p>

---

[![Issues](https://img.shields.io/github/issues/ftdot/rjoiner?style=for-the-badge)](https://github.com/ftdot/rjoiner/issues)
[![Latest tag](https://img.shields.io/github/v/tag/ftdot/rjoiner?style=for-the-badge)](https://github.com/ftdot/rjoiner/tags)

---

- ✅ [Features](#features)
- 📦 [Requirements](#requirements)
- 📥 [Installation](#installation)
- ☢️ [Usage](#usage)
- 🤔 [Notes about .DLL joining (**IMPORTANT**)](#notes-about-dll-joining-important)
- ✳️ [Some examples](#some-examples)
- 🆘 [Issue contributing](#issue-contributing)

---

### Features

* 📁 **Join any executable (that can execute OS): .exe, .vbs, .bat, .js,**
  **etc. (and linux executables)**
* ⚙️ **Join scripts (only Linux) - ``.sh``, ``.py``**
* ✅ **Works on any platform**
* 🔐 **Contents of files is encrypted by ``AES256-GCM``**
* 👤 **Admin rights requirement (optional) (only for Windows)**
* 🌆 **Custom icon (optional) (only for Windows)**
* 🤏🏿 **Small weight**
* 🗂 **GZIP comperssion**
* 📜 **Message box on run (optional)**
* 📵 **Anti-VM, Anti-Debug functional (optional)**
* 📛 **Anti-Sandboxie (optional)**
* 💎 **Can run commands when output file runned (optional)**
* ⚙️ **Can add files to autorun (optional)**

Join any executables for any platform. Stub of joiner is written on Rust, that
allows to compile this to any platform. Joiner have cross platform structure,
that allows to use it on Windows, Linux, Android and other platforms, that Rust
compiler is supports. Joiner uses python as helper to create easy CLI. You can
use it as utility in your projects. Ex.: Your program requires joined executables,
you can with RJoiner CLI build this executable and use it for your program.


### Requirements

For first you need:
- 🔧 Rust toolchain installed on (**In development used (and recommended) cargo 1.66.0**)
- 🐍 Python 3.10 and above (**In development used Python 3.11**)
- 📑 Files, that you want to join (**In development used .vbs, .bat, .exe files. Note, that .vbs, .bat and etc. files can't run on Linux**)


### Installation

Just download last release from `releases` at right menu.

Also, if you have **GIT CLI** installed on, you can copy repository via command:

> Any platform

```shell
$ git clone https://github.com/ftdot/rjoiner
```


### Usage

> **Warning**
>
> The author(s) of this software are NOT responsible for its use by other users.
> By using this tool, you agree that only you responsible for its use.

```shell
usage: rjoiner.py [-h] [-v] [--copyright] [--license] [-i ICON] [-a] [-t TARGET] [-d] [-V] -f FILE [--no-banner] [-M] [-m MESSAGE] [-T TITLE]
                  [-I MBOX_ICON_TYPE] [-b] [--anti-vm] [--anti-debug] [--anti-sandboxie] [-c COMMAND]

options:
  -h, --help            show this help message and exit
  -v, --version         Show version
  --copyright           Show copyright
  --license             Show license
  -i ICON, --icon ICON  Path to icon (only .ico) (available only for windows!)
  -a, --admin           Enables request to admin execution level (available only for windows!)
  -t TARGET, --target TARGET
                        Specify a target to cargo (list installed targets: rustup target list --installed) (can be multiple)
  -d, --debug           Enables debug mode (by default, compiles in release mode) (prevents to temp/ directory deletion, enables console on windows releases)  
  -V, --verbose         Enables verbose mode (prints exceptions, current steps)
  -f FILE, --file FILE  Path to the file(s) to join (can be multiple) (start filename with "!" to autorun it)
  --no-banner           Disables RJoiner banner
  -M, --msgbox          Shows messagebox (you need also provide -m, -T argument)
  -m MESSAGE, --message MESSAGE
                        Message to show at messagebox
  -T TITLE, --title TITLE
                        Title to show at messagebox
  -I MBOX_ICON_TYPE, --mbox-icon-type MBOX_ICON_TYPE
                        Set icon type of messagebox (available: error, info. By default: error)
  -b, --msgbox-before   Shows messagebox before executing joined programs (by default, messagebox will be showed after executing joined programs)
  --anti-vm             Enables Anti-VM (only x86)
  --anti-debug          Enables Anti-Debug (available only for Linux builds!) (only x86)
  --anti-sandboxie      Enables Anti-Sandboxie (only Windows)
  -c COMMAND, --command COMMAND
                        Commands to execute when runned (by default is doesn't run any. Can be multiple)
```

To join any files, you must provide `-f` flag(s) with path(es) to file(s).

> Linux

```shell
$ python3 rjoiner.py -f example1 -f example2 -f example3
```

> Windows

```shell
$ python rjoiner.py -f example1 -f example2 -f example3
```

It will generate output in directory `out`.

> **Note**
>
> You can provide flag `-f` (files) as many as you want. But at least there
> should be at least one file here. You can add 1 or 5 and even 1000.


#### Add icon to the output file (Windows build only)

> **Note**
>
> Icon to the file can only be added on windows builds. Yes, you can build windows
> binary with icon from linux system, but add icon to linux binaries you cannot.

To add icon, you must provide path to the icon by flag `-i`.

> Linux

```shell
$ python3 rjoiner.py -f example1 -f example2.sh -f example3.py -i icon.ico
```

> Windows

```shell
$ python rjoiner.py -f example1.exe -f example2.vbs -f example3.bat -i icon.ico
```


#### Add admin rights requirement (Windows build only)

> **Note**
>
> Icon to the file can only be added on windows builds.
> Yes, you can build windows binary with icon from linux system,
> but add icon to linux binaries you cannot.

To set requirement to admin rights, you must provide flag `-a`.
Let add icon and admin rights requirement to binary.

> Linux

```shell
$ python3 rjoiner.py -f file1.exe -f file2.exe -a -i icon.ico
```

> Windows

```shell
$ python rjoiner.py -f file1.exe -f file2.exe -a -i icon.ico
```


### Notes about .DLL joining (**IMPORTANT**)

To join `.DLL` files you must known, that every joined dll file must have function `EntryPoint()` (without any arguments).

Example of that `.DLL` on **C++**:

```cpp
#include <windows.h>

extern "C" __declspec(dllexport)
DWORD WINAPI EntryPoint(LPVOID lpParam) {
  MessageBox(NULL, "Hello world!", "Hello World!", NULL);
  return 0;
}

extern "C" __declspec(dllexport)
BOOL APIENTRY DllMain(HMODULE hModule,
                      DWORD ul_reason_for_call,
                      LPVOID lpReserved) {
  switch (ul_reason_for_call) {
    case DLL_PROCESS_ATTACH:
      CreateThread(NULL, NULL, EntryPoint, NULL, NULL, NULL);
      break;
    case DLL_THREAD_ATTACH:
    case DLL_THREAD_DETACH:
    case DLL_PROCESS_DETACH:
      break;
  }
  return TRUE;
}
```

If you want the technical details, every dlls runs by it command:

```shell
$ rundll32.exe <Path To DLL>,EntryPoint
```


### Some examples

> Example with the messagebox, .exe and .vbs files

```shell
python rjoiner.py -i icon.ico -f Clicker.exe -f calc.vbs -M -m "Buffer overflow at 0x001ac1faff" -T "Runtime error" -t x86_64-pc-windows-msvc
```

> Example with the linux executables, scripts and python script

```shell
python3 rjoiner.py -f mousepad -f escape_docker.sh -f create_user.py -f shutdown_sys.sh -t x86_64-unknown-linux-gnu
```

> Example with autorun of calc.vbs + messagebox with `"` character

```shell
python rjoiner.py -i icon.ico -f Clicker.exe -f !calc.vbs -M -m "Buffer overflow at \"0x001ac1faff\"" -T "Runtime error"
```

> Example with icon, anti-vm, anti-sandboxie and many files (including files with the same names) and with messagebox. Used in tests

```shell
python rjoiner.py -i icon.ico -f Clicker.exe -f calc.vbs -f test/calc.vbs -f test/Clicker.exe -f FileDate.exe -M -m "BufferOverflow error at 0x00a8c1faff" -T "Rust runtime error" --anti-vm --anti-sandboxie -t x86_64-pc-windows-msvc
```

> Example with many arguments. Used in tests

```shell
python rjoiner.py -i icon.ico -f Clicker.exe -f calc.vbs -f test/calc.vbs -f test/Clicker.exe -f FileDate.exe -M -m "BufferOverflow error at 0x00a8c1faff" -T "Rust runtime error" --anti-vm --anti-sandboxie -c calc.exe -c cmd.exe --no-temp-del -d -V --yes-console -t x86_64-pc-windows-msvc
```

### Issue contributing

**If you have any issue with the joiner at build stage, you must do these steps:**
* Provide your cargo version
* Provide your system on that you try to build executable
* Run joiner with `-V` flag (verbose) and provide it output

**If you have any issue at runtime stage, you must do these steps:**
* Provide your cargo version
* Provide your system(s) on that you build and run joiner
* Run joiner with `-V` (verbose) and `-d` (debug) flags and provide it output
* Execute output file (joined files) in the console and provide it output
* Provide file names and extension (only it name and extension, doesn't attach files!)

**If you want provide a idea to this project, just do these steps:**
* Introduce your idea
* Add ``feature request`` label and wait

**If you want contribute to the code (PR) do these steps:**
* Write the main idea of the PR
* Write a changelog

---

<!-- There is tags, you can ignore it -->
<p>
merge exe files, join exe files, executables joiner, rjoiner, rusty joiner,
joiner on rust, rust programming language, merge executables, exe joiner,
join vbs and executables, merge vbs and executables, merge scripts, merge
python files, merge linux scripts, js files joiner, easy joiner, easy merge,
compile two files in one, merge two files, commands macros, linux files merge,
merge linux executables, ftdot
</p>
