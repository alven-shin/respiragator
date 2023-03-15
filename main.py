import time

import board
import busio
from adafruit_circuitplayground.express import cpx as cp
from analogio import AnalogIn


def main() -> None:
    # cp.pixels.brightness = 0.1
    uart = busio.UART(board.TX, board.RX, baudrate=9600)
    analog_in = AnalogIn(board.A1)

    # sum = 0
    # for _ in range(1000):
    while True:
        # accelerometer
        _x, _y, z = cp.acceleration
        # if z > 11:  # breathe in, red
        #     cp.pixels.fill((50, 0, 0))
        # elif z < 9:  # breathe out, green
        #     cp.pixels.fill((0, 50, 0))
        # else:
        #     cp.pixels.fill((0, 0, 0))

        # bluetooth rx/tx
        # data = uart.read(32)
        # uart.write(bytes(str(z) + "\n", "ascii"))
        try:
            print(int.to_bytes(int(analog_in.value / 5) - 100, 1, "big"))
            uart.write(int.to_bytes(int(analog_in.value / 10) - 100, 1, "big"))
            # uart.write(int.to_bytes(int(z), 1, "big"))
        except OverflowError:
            uart.write(b"\0")
        # print(int(z))
        # print(int.to_bytes(int(z), 1, "big"))
        # uart.write(b"poop\n")

        # if data is not None:
        #     data_string = "".join([chr(b) for b in data])
        #     print(data_string, end="")
        # sum += analog_in.value
        # time.sleep(0.01)

    # print(sum / 1000)
    # while True:
    #     ...


if __name__ == "__main__":
    main()
