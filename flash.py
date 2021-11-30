#!/bin/python3
import argparse
from genericpath import exists
import subprocess
import os

def build(release):
    cmd = ['cargo', 'build']
    if release:
        cmd.append('--release')
    subprocess.run(cmd)

def flash(path):
    if not os.path.exists(path):
        raise f"binary {path} does not exist"

    subprocess.run(['sudo', 'lm4flash', path])

def make_bin(release):
    release_path = 'release' if release else 'debug'
    exe_name = 'tiva-serial'
    exe_path = f"target/thumbv7em-none-eabihf/{release_path}/{exe_name}"
    bin_path = exe_path + ".bin"

    if not os.path.exists(exe_path):
        raise f"executable {exe_path} does not exist"

    subprocess.run(['arm-none-eabi-objcopy', '-O', 'binary', exe_path, bin_path])
    return bin_path


def main():
    parser = argparse.ArgumentParser(description='Create binary and flash it')
    parser.add_argument('--release', default = False, action='store_true', help = 'specifies to flash the release binary')
    args = parser.parse_args()

    build(args.release)
    bin_path = make_bin(args.release)
    print(bin_path)
    flash(bin_path)

if __name__ == "__main__":
    main()
