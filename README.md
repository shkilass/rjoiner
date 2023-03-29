
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
- 🆘 [Issue contributing](#issue-contributing)
- ✳️ [Some examples](#some-examples)

---

> **Warning**
>
> Current version is unstable and have many issues at runtime time.
> Also, if targeted linux doesn't have python installed on, this will not
> work.

### Features

* 📁 **Join any executable (that can execute OS): .exe, .vbs, .bat, .js, etc. (and linux executable)**
* ⚙️ **Join scripts (only Linux) - ``.sh``, ``.py``**
* ✅ **Works on any platform**
* 🔐 **Contents of file is encrypted by ``AES256-GCM``**
* 👤 **Admin rights requirement (optional) (only for Windows)**
* 🌆 **Custom icon (optional) (only for Windows)**
* 🤏🏿 **Small weight**
* 🗂 **GZIP comperssion**
* 📜 **Message box on run (optional)**
* 📵 **Anti-VM, Anti-Debug functional (optional)**
* 📛 **Anti-sandboxie (optional)**
* 🧠 **Support files with the same names**
* 💎 **Can run commands (optional)**
* ⚙️ **Can add to autorun (optional)**

Joiner of any executables for any platform. Stub of joiner is written on Rust,
that allows to copmile this to any platform. Joiner have crossplatform structure,
that allows to use it on Windows, Linux, Android and other platforms, that Rust
compiler is supports.

**💡 To be implemented:**
- [*] Add files in autorun (Medium)
- [*] Execute commands (Easy)
- [ ] Drop executable in memory (Linux only) (Medium)
- [ ] .dll execution/injection (Windows only) (Easy/Hard)
- [+] Add support for: .txt (Windows only), .ps1 (Windows only) (Easy) (Added support for all
  executable files on Windows with simple cmd.exe usage to start files)
- [ ] Add full support to build joiner to Android (.APK) (Not, it wouldn't join APKs) (Hard)
- [*] Multiple targets (Easy)
- [ ] Anti-VM for Windows on Registry, FS, etc. checks (Medium)
- [ ] GUI (Medium)
- [ ] Builds with Cargo & Python integrated in (Medium)

> **Note**
>
> Tasks from this list may be never completed


### Helpers list

There aren't helpers :(


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
> The author(s) of this software are NOT responsible for the usage.
> With this tool usage, you agree to take RESPONSIBILITY of your
> actions only FOR YOU.

```shell
usage: rjoiner.py [-h] [-v] [--copyright] [--license] [-i ICON] [-a] [-t TARGET] [-d] [--yes-console] [--no-temp-del] [-V] -f FILE [--no-banner] [-M]
                  [-m MESSAGE] [-T TITLE] [-I MBOX_ICON_TYPE] [--anti-vm] [--anti-debug] [--anti-sandboxie] [-c COMMAND]

options:
  -h, --help            show this help message and exit
  -v, --version         Show version
  --copyright           Show copyright
  --license             Show license
  -i ICON, --icon ICON  Path to icon (only .ico) (available only for windows!)
  -a, --admin           Enables request to admin execution level (available only for windows!)
  -t TARGET, --target TARGET
                        Specify a target to cargo (list installed targets: rustup target list --installed) (use "all" to specify all targets)
  -d, --debug           Enables debug mode (by default, compiles in release mode)
  --yes-console         Enables console for windows (useful for debugging)
  --no-temp-del         Prevents from temporary directory deletion (useful for debugging)
  -V, --verbose         Enables verbose mode (prints exceptions, current steps)
  -f FILE, --file FILE  Path to the file(s) to join (can be multiple)
  --no-banner           Disables RJoiner banner
  -M, --msgbox          Shows messagebox (you need also provide -m, -T argument)
  -m MESSAGE, --message MESSAGE
                        Message to show at messagebox
  -T TITLE, --title TITLE
                        Title to show at messagebox
  -I MBOX_ICON_TYPE, --mbox-icon-type MBOX_ICON_TYPE
                        Set icon type of messagebox (available: error, info. By default: error)
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
$ python3 rjoiner.py -f 
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

If you will provide any help to this project, I will add you to **Helpers list**

You read this to the end... Then I tell you about the name of joiner.
Rusty was chosen because this tool written on Rust (and python, but... Okay).
And tagline of this tool that: **Old doesn't mean bad**. But, this tool
is new and Rust programming language is ~new, but okay.
