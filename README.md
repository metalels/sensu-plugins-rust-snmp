# Sensu-Plugins-Rust-snmp

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
  -N, --name NAME                     set target agent name
  -H, --host ADDRESS                  set target host ip address
  -P, --port PORT                     set target host port
  -C, --community COMMUNITY           set target community name
  -O, --oids OID[:OID_NAME],OID...    set target oid(s: use [,] to joins multi oids)
  -d, --debug                         print debug logs
  -h, --help                          print help menu
```

## Authors ##

[metalels](https://github.com/metalels)

