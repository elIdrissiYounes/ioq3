#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import sys
import argparse
import plumbum as pb
import multiprocessing


IOQ3 = os.path.dirname(os.path.realpath(__file__))
QUAKE3_RS = os.path.join(IOQ3, 'quake3-rs')
TARGET_DIR = os.path.join(QUAKE3_RS, 'target/release')
BUILD_DIR = os.path.join(IOQ3, 'build/debug-linux-x86_64')
BASEQ3_DIR = os.path.join(BUILD_DIR, 'baseq3')
NUM_JOBS = multiprocessing.cpu_count()


"""
Helper function that checks if a binary exists, if it does return the path
"""
def get_binary(bin_path, bin_name):
    binary_path = os.path.join(bin_path, bin_name)
    err_msg = "`{}` does not exist in {}".format(bin_name, bin_path)
    assert os.path.isfile(binary_path), err_msg
    return binary_path


"""
Assert that the links were created
"""
def assert_links():
    assert os.path.islink(os.path.join(BUILD_DIR, 'ioquake3.x86_64'))
    assert os.path.islink(os.path.join(BASEQ3_DIR, 'cgamex86_64.so'))
    assert os.path.islink(os.path.join(BASEQ3_DIR, 'qagamex86_64.so'))
    assert os.path.islink(os.path.join(BASEQ3_DIR, 'uix86_64.so'))


"""
Assert that the project build directories exist
"""
def assert_build_directory():
    assert os.path.isdir(BUILD_DIR),\
        "{} does not exit, run build step!".format(BUILD_DIR)
    assert os.path.isdir(TARGET_DIR),\
        "{} does not exit, run build step!".format(TARGET_DIR)


"""
Run the binary with the appropriate LD_LIBRARY_PATH 
"""
def _run():
    print("Running...")
    # LD_LIBRARY_PATH=`rustc --print=sysroot`/lib
    # ./ioquake3.x86_64 +set sv_pure 0 +set vm_game 0 +set vm_cgame 0 +set vm_ui 0
    assert_build_directory()
    rustc = pb.local['rustc']
    rust_lib_path = rustc['--print=sysroot']()
    rust_lib_path = os.path.join(rust_lib_path.rstrip(), 'lib')

    ioq3_args = ['+set', 'sv_pure', '0', '+set', 'vm_game', '0', '+set',
                 'vm_cgame', '0', '+set', 'vm_ui', '0']

    ioq3 = pb.local[os.path.join(BUILD_DIR, 'ioquake3.x86_64')]
    with pb.local.env(LD_LIBRARY_PATH='{}'.format(rust_lib_path)):
        _, stdout, stderr = ioq3[ioq3_args].run()
        if stderr:
            print(stderr)
    print("Done running!")


"""
Links the rust binaries to ioq3 build directory
"""
def _copy():
    print("Copying...")
    make = pb.local['make']
    assert_build_directory()
    ln = pb.local['ln']
    ioquake3_rs = get_binary(TARGET_DIR, 'ioquake3')
    cgame_rs = get_binary(TARGET_DIR, 'libquake3_cgame.so')
    game_rs = get_binary(TARGET_DIR, 'libquake3_game.so')
    ui_rs = get_binary(TARGET_DIR, 'libquake3_ui.so')

    # Link main binary
    with pb.local.cwd(BUILD_DIR):
        ln['-sf', ioquake3_rs, 'ioquake3.x86_64']()

    with pb.local.cwd(BASEQ3_DIR):
        ln['-sf', cgame_rs, 'cgamex86_64.so']()
        ln['-sf', game_rs, 'qagamex86_64.so']()
        ln['-sf', ui_rs, 'uix86_64.so']()

    assert_links()
    print("Done copying!")


"""
Build the C project, then build the rust project.
(The reason the C project is needed is for the renderer)
"""
def _build():
    print("Building...")
    make = pb.local['make']
    cargo = pb.local['cargo']

    make['debug', '-j{}'.format(NUM_JOBS)]()

    with pb.local.cwd(QUAKE3_RS):
        cargo['build', '--release', '-j{}'.format(NUM_JOBS)]()

    assert_build_directory()
    print("Done building!")


"""
Run all of the steps
"""
def _all():
    print("Going to run all of the steps...")
    _build()
    _copy()
    _run()
    sys.exit(0)


def run_arg(args):
    if args.all:
        _all()
    if args.build:
        _build()
    if args.copy:
        _copy()
    if args.run:
        _run()


def parse_args():
    desc = 'Build, copy, and run quake3-rs'
    parser = argparse.ArgumentParser(description=desc)
    parser.add_argument('--all', default=False,
                        action='store_true', dest='all',
                        help='Build, copy, and run quake3-rs')
    parser.add_argument('--build', default=False,
                        action='store_true', dest='build',
                        help='Build quake3-rs')
    parser.add_argument('--copy', default=False,
                        action='store_true', dest='copy',
                        help='Copy all the binaries to the necessary locations')
    parser.add_argument('--run', default=False,
                        action='store_true', dest='run',
                        help='Run quake3-rs')
    return parser.parse_args()


def main():
    args = parse_args()
    if len(sys.argv) <= 1:
        print("Please give a command!")
        sys.exit(1)
    run_arg(args)


if __name__ == "__main__":
    main()
