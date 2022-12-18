import os
import shutil
from pathlib import Path


def main() -> None:
    user = os.getlogin()

    if Path("/Volumes/CIRCUITPY").exists():
        print("macos")
        shutil.copy("main.py", "/Volumes/CIRCUITPY")
    elif Path("/run/media/{}/CIRCUITPY".format(user)).exists():
        print("linux")
        shutil.copy("main.py", "/run/media/{}/CIRCUITPY".format(user))
    else:
        print("oops")


if __name__ == "__main__":
    main()
