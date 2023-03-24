
# RJoiner (Rusty Joiner)

* Join these executable: .exe, .vbs, .bat (and linux executable)
* Join scripts (only Linux) - .sh, .py
* Works on any platform
* Contents of file is encrypted by AES256-GCM
* Admin rights requirement (optional) (only for Windows)
* Custom icon (optional) (only for Windows)
* Small weight
* GZIP comperssion

Joiner of any executables for any platform. Stub of joiner is written on Rust,
that allows to copmile this to any platform. Joiner have crossplatform structure,
that allows to use it on Windows, Linux, Android and other platforms, that Rust
compiler is supports.

### Requirements

For first you need:
- Rust toolchain installed on (**Used in development and recommended is cargo 1.66.0**)
- Python 3.10 and above (**In development used Python 3.11**)
- Files, that you want to join (**In development used .vbs, .bat, .exe files. Note, that .vbs, .bat and etc. files can't run on Linux**)

### Installation

Just download last release from `releases` at right menu.

Also, if you have **GIT CLI** installed on, you can copy repository via command:

> Any platform

```shell
$ git clone https://github.com/ftdot/rjoiner
```

### Usage

To join any files, you must provide `-f` flag(s) with path(es) to file(s).

> Linux

```shell
$ python3 joiner.py -f example1 -f example2 -f example3
```

> Windows

```shell
$ python joiner.py -f example1 -f example2 -f example3
```

It will generate output in directory `out`.

---
**NOTE**

You can provide flag `-f` (files) as many as you want. But at least there
should be at least one file here. You can add 1 or 5 and even 1000.

---

#### Add icon to the output file (Windows build only)

---
**NOTE**

Icon to the file can only be added on windows builds. Yes, you can build windows
binary with icon from linux system, but add icon to linux binaries you cannot.

---

To add icon, you must provide path to the icon by flag `-i`.

> Linux

```shell
$ python3 joiner.py -f example1 -f example2 -i icon.ico
```

> Windows

```shell
$ python3 joiner.py -f example1 -f example2 -i icon.ico
```

#### Add admin rights requirement (Windows build only)

---
**NOTE**

Icon to the file can only be added on windows builds.
Yes, you can build windows binary with icon from linux system,
but add icon to linux binaries you cannot.

---

To set requirement to admin rights, you must provide flag `-a`.
Let add icon and admin rights requirement to binary.

> Linux

```shell
$ python3 joiner.py -f 
```

### Issue contributing

If you have any issue with the joiner, you must do this steps:
* Provide your cargo version
* Run joiner with `-V` flag (verbose) and provide it output
* 
