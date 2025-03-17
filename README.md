# ISOName Parser
A simple tool to parse ISONames into their specific fields. 

## Installation:
To build and isntall run the following
```
cargo install path . --root ~/.local/
```

## Usage:
To get the full isoname from the individual fields run thhe following:
```
$ isoname-parser --pack-name
Address Arbitration Capable: 1
Industry Group: 2
Vehicle System (Device Class) Instance: 0
Vehicle System (Device Class): 0
Function Code: 132
Function Code Instance: 0
ECU Instance: 0
Manufacturer Code: 94 
Serial Number: 1234
Isoname: 0xA00084000BC004D2
```

To parse the fileds from a full isoname:
```
$ isoname-parser 0xA00084000BC004D2
Address Arbitration Capable:    1
Industry Group:                 2
Vehicle System Instance:        0
Vehicle System (Device Class):  0
Function Code:                  132
Function Instance:              0
ECU Instance:                   0
MC Code:                        94
Serial:                         1234
```


## References:
To translate these fields to values, see the [ISOBUS Database](https://www.isobus.net/isobus/)
