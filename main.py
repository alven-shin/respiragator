import time

from adafruit_circuitplayground.express import cpx as cp

while True:
    cp.red_led = True
    time.sleep(0.5)
    cp.red_led = False
    time.sleep(0.5)
