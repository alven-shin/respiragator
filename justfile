default:
    just -l

console:
    #! /bin/bash
    screen /dev/tty.usbmodem* 115200

transfer:
    python3 scripts/transfer.py
