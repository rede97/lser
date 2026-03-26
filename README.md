# lser

[![Crates.io](https://img.shields.io/crates/v/lser)](https://crates.io/crates/lser)
[![Crates.io Downloads](https://img.shields.io/crates/d/lser)](https://crates.io/crates/lser)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/rede97/lser/actions/workflows/rust.yml/badge.svg)](https://github.com/rede97/lser/actions/workflows/rust.yml)

`lser` is a fast, cross-platform CLI tool written in Rust that lists all available serial ports on your machine — including USB-to-serial adapters, JTAG probes, and built-in UART controllers.

It reads port metadata directly from the OS (vendor name, product string, USB VID:PID) and presents it in a polished terminal table. Three output formats are supported so `lser` fits equally well in interactive terminal sessions and automation scripts.

## Features

- Beautiful terminal table with Unicode borders (powered by [rich_rust](https://crates.io/crates/rich_rust))
- `--plain`: CSV output for use with `awk`, `grep`, `cut`, `csvkit`, and other text tools
- `--json`: JSON array output for shell scripts, Python, or any tooling that speaks JSON
- Results sorted alphabetically by port name
- Detects USB vendor/product info and VID:PID automatically
- Falls back to kernel driver name when USB metadata is unavailable
- Supports Linux (x86, x86_64, arm, aarch64) and Windows
- Single static binary, zero configuration

## Supported Platforms

| Platform | Target triple | Release binary |
|----------|--------------|----------------|
| Linux x86_64 | `x86_64-unknown-linux-gnu` | `lser-linux-x86_64` |
| Linux arm (soft-float) | `arm-unknown-linux-gnueabi` | `lser-linux-arm-eabi` |
| Linux arm (hard-float) | `arm-unknown-linux-gnueabihf` | `lser-linux-arm-eabihf` |
| Linux aarch64 | `aarch64-unknown-linux-gnu` | `lser-linux-aarch64` |
| Windows x86_64 | `x86_64-pc-windows-msvc` | `lser-windows-x86_64.exe` |
| macOS | — | Untested |

## Install

```bash
cargo install lser
```

Or download a pre-built binary from the [Releases](https://github.com/rede97/lser/releases) page.

## Usage

```bash
lser              # default: rich table
lser --plain      # CSV output, pipe-friendly
lser --json       # JSON array for tooling
lser --help       # show help
```

### Default output (rich table)

```
┏━━━━━━━━━━━━━━┳━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┳━━━━━━━━━━━┓
┃ Name         ┃  Vendor   ┃            Product            ┃    USB    ┃
┡━━━━━━━━━━━━━━╇━━━━━━━━━━━╇━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╇━━━━━━━━━━━┩
│ /dev/ttyACM0 │ Espressif │ USB JTAG/serial debug unit:00 │ 303a:1001 │
│ /dev/ttyACM1 │  wch.cn   │          WCH-Link:01          │ 1a86:8010 │
└──────────────┴───────────┴───────────────────────────────┴───────────┘
```

### `--plain` — CSV output

One port per line with a header row. Works with `awk`, `cut`, `csvkit`, etc.

```
$ lser --plain
name,vendor,product,usb
/dev/ttyACM0,Espressif,USB JTAG/serial debug unit:00,303a:1001
/dev/ttyACM1,wch.cn,WCH-Link:01,1a86:8010
```

```bash
# Example: print only device names
lser --plain | tail -n +2 | cut -d, -f1
```

### `--json` — JSON output

Outputs a JSON array, ideal for shell scripts or third-party tools.

```
$ lser --json
[
  {
    "name": "/dev/ttyACM0",
    "vendor": "Espressif",
    "product": "USB JTAG/serial debug unit:00",
    "usb": "303a:1001"
  },
  {
    "name": "/dev/ttyACM1",
    "vendor": "wch.cn",
    "product": "WCH-Link:01",
    "usb": "1a86:8010"
  }
]
```

```bash
# Example: extract all device names with jq
lser --json | jq '.[].name'
```

### Output columns

| Column  | Description |
|---------|-------------|
| Name    | Device path (e.g. `/dev/ttyACM0`, `COM3`) |
| Vendor  | USB vendor name or kernel driver name |
| Product | USB product string |
| USB     | USB Vendor ID and Product ID in `VID:PID` format |

Fields without information are shown as `--`.

## Build from source

```bash
git clone https://github.com/rede97/lser.git
cd lser
cargo build --release
```

The binary will be at `target/release/lser`.

## Dependencies

- [serial_enumerator](https://crates.io/crates/serial_enumerator) — cross-platform serial port enumeration
- [rich_rust](https://crates.io/crates/rich_rust) — beautiful terminal output (Rust port of Python's Rich)
- [clap](https://crates.io/crates/clap) — command-line argument parsing
- [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) — JSON serialization

## License

MIT — see [LICENSE](LICENSE)
