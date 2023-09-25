import board
import busio

from analogio import AnalogIn

SAMPLE_SIZE = 100


def main() -> None:
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


if __name__ == "__main__":
    main()
