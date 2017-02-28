# Sensu-Plugins-Rust-snmp

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.txt)

Sensu plugins uses Rust.

## Installation ##

  1. git clone https://github.com/metalels/sensu-plugins-rust-snmp.git
  2. execute metrics/metrics-snmp

## Dependencies of compile ##

* Rust
* Cargo
* and see Cargo.toml

## Usage ##

```
Usage: metrics-snmp METRIC [options]

Requires:
    METRIC: <desc|ss|la|mem|dsk|if> or custom name with -O(oids)

Options:
    -n, --name NAME     set target agent name
    -h, --host ADDRESS  set target host ip address
    -p, --port PORT     set target host port
    -c, --community COMMUNITY
                        set target community name
    -o, --oids OID[:OID_NAME],OID[:OID_NAME]...
                        set target oid(s: use [,] to joins multi oids)
    -D, --debug         print debug logs
    -H, --help          print this help menu
```

## Authors ##

[metalels](https://github.com/metalels)

