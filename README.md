# lser

A cli tool to display avaliable serial ports of your device.

* Support Linux, Windows
* Support arm and x86 devices of linux

## install
```bash
$ cargo install lser
```

## example
```bash
$ lser
+--------------+------------+---------------------+-----------+
| Name         | Vendor     | Product             | USB       |
+--------------+------------+---------------------+-----------+
| /dev/ttyS0   |    pnp     |       PNP0501       |    --     |
+--------------+------------+---------------------+-----------+
| /dev/ttyUSB0 |  Digilent  | Digilent USB Device | 0403:6014 |
+--------------+------------+---------------------+-----------+
| /dev/ttyUSB1 | ch341-uart |    USB2.0-Serial    | 1a86:7523 |
+--------------+------------+---------------------+-----------+
```