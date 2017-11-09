#!/bin/python

import os
import glob
import subprocess
import sys

def build_cbindgen():
    try:
        subprocess.check_output(["cargo", "build"])
        return True
    except subprocess.CalledProcessError:
        return False

def cbindgen(rust_src, out, c, config):
    bin = ["target/debug/cbindgen"]
    compile = [rust_src, "-o", out]
    flags = []

    if c:
        flags += ["--lang", "c"]

    if config != None:
        flags += ["--config", config]

    command = bin + flags + compile
    print(command)
    subprocess.check_output(bin + flags + compile)

def gcc(src):
    gcc_bin = os.environ.get('CC')
    if gcc_bin == None:
        gcc_bin = 'gcc'

    subprocess.check_output([gcc_bin, "-D", "DEFINED", "-c", src, "-o", "tests/expectations/tmp.o"])
    os.remove("tests/expectations/tmp.o")

def gxx(src):
    gxx_bin = os.environ.get('CXX')
    if gxx_bin == None:
        gxx_bin = 'g++'

    subprocess.check_output([gxx_bin, "-D", "DEFINED", "-std=c++11", "-c", src, "-o", "tests/expectations/tmp.o"])
    os.remove("tests/expectations/tmp.o")

def run_compile_test(rust_src, c):
    if c:
        out = os.path.join('tests/expectations/', os.path.basename(rust_src).replace(".rs", ".c"))
    else:
        out = os.path.join('tests/expectations/', os.path.basename(rust_src).replace(".rs", ".cpp"))

    config = rust_src.replace(".rs", ".toml")
    if not os.path.exists(config):
        config = None

    try:
        cbindgen(rust_src, out, c, config)

        if c:
            gcc(out)
        else:
            gxx(out)

    except subprocess.CalledProcessError:
        return False

    return True

if not build_cbindgen():
    exit()

args = sys.argv[1:]
files = [x for x in args if not x.startswith("-")]

tests = []
if len(files) == 0:
    tests = glob.glob("tests/rust/*.rs")
else:
    tests = files

num_pass = 0
num_fail = 0

# C

for test in tests:
    if run_compile_test(test, True):
        num_pass += 1
        print("Pass - %s" % test)
    else:
        num_fail += 1
        print("Fail - %s" % test)

# C++

for test in tests:
    if run_compile_test(test, False):
        num_pass += 1
        print("Pass - %s" % test)
    else:
        num_fail += 1
        print("Fail - %s" % test)

print("Tests complete. %i passed, %i failed." % (num_pass, num_fail))
