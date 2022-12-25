import time

from adafruit_circuitplayground.express import cpx as cp


def main() -> None:
    # cp.pixels.brightness = 0.1
    print("running...")
    start = time.monotonic()

    try:
        with open("data.csv", "w") as file:
            file.write("time,x,y,z\n")
            while True:
                # for i in range(len(cp.pixels)):
                #     cp.pixels[i] = (15 + i * 10, 15, 15)
                #     time.sleep(0.15)

                # for i in range(len(cp.pixels)):
                #     cp.pixels[i] = (0, 0, 0)
                #     time.sleep(0.15)
                x, y, z = cp.acceleration
                # print("{},{},{},{}".format(time.monotonic() - start, x, y, z))
                file.write("{},{},{},{}\n".format(time.monotonic() - start, x, y, z))
                time.sleep(0.1)
    except OSError:
        print("read-only fs")


if __name__ == "__main__":
    main()
