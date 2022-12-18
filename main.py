import time

from adafruit_circuitplayground.express import cpx as cp


def main() -> None:
    cp.pixels.brightness = 0.1

    while True:
        for i in range(len(cp.pixels)):
            cp.pixels[i] = (15 + i * 10, 15, 15)
            time.sleep(0.15)

        for i in range(len(cp.pixels)):
            cp.pixels[i] = (0, 0, 0)
            time.sleep(0.15)


if __name__ == "__main__":
    main()
