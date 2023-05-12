# import time

import board
import busio

# from adafruit_circuitplayground.express import cpx as cp
from analogio import AnalogIn

SAMPLE_SIZE = 100


def main() -> None:
    # cp.pixels.brightness = 0.1
    uart = busio.UART(board.TX, board.RX, baudrate=9600)
    analog_in = AnalogIn(board.A1)

    while True:
        # average the values after taking N samples
        sum = 0
        for _ in range(SAMPLE_SIZE):
            sum += analog_in.value
        avg = sum / SAMPLE_SIZE

        # send the value to the bluetooth module
        # ignore value if it overflows a byte
        try:
            uart.write(int.to_bytes(int(avg / 10), 1, "big"))
        except OverflowError:
            print(avg / 10)


"""
    start = time.monotonic()
    try:
        with open("data.csv", "w") as file:
            # file.write("time,data\n")
            while True:
                cp.pixels.fill((0, 50, 0))
                sum = 0
                for _ in range(SAMPLE_SIZE):
                    sum += analog_in.value

                # print("{},{},{},{}".format(time.monotonic() - start, x, y, z))
                file.write(
                    "{},{}\n".format(time.monotonic() - start, sum / SAMPLE_SIZE)
                )
                if time.monotonic() > 15:
                    break
        while True:
            cp.pixels.fill((50, 50, 50))
    except OSError:
        print("read-only fs")
        while True:
            cp.pixels.fill((50, 0, 00))
    # sum = 0
    # for _ in range(1000):
"""


if __name__ == "__main__":
    main()
