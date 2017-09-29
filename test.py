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

    subprocess.check_output([gcc_bin, "-D", "DEFINED", "-c", src, "-o", "compile-tests/tmp.o"])
    os.remove("compile-tests/tmp.o")

def gxx(src):
    gxx_bin = os.environ.get('CXX')
    if gxx_bin == None:
        gxx_bin = 'g++'

    subprocess.check_output([gxx_bin, "-D", "DEFINED", "-std=c++11", "-c", src, "-o", "compile-tests/tmp.o"])
    os.remove("compile-tests/tmp.o")

def run_compile_test(rust_src, leave_output, c):
    if c:
        out = rust_src.replace(".rs", ".c")
    else:
        out = rust_src.replace(".rs", ".cpp")

    config = rust_src.replace(".rs", ".toml")
    if not os.path.exists(config):
        config = None

    try:
        cbindgen(rust_src, out, c, config)

        if c:
            gcc(out)
        else:
            gxx(out)

        if not leave_output:
            os.remove(out)
    except subprocess.CalledProcessError:
        if not leave_output and os.path.exists(out):
            os.remove(out)
        return False

    return True

if not build_cbindgen():
    exit()

args = sys.argv[1:]
files = [x for x in args if not x.startswith("-")]
flags = [x for x in args if x.startswith("-")]

leave_output = False
c = False

for flag in flags:
    if flag == "-l":
        leave_output = True
    elif flag == "-c":
        c = True

tests = []
if len(files) == 0:
    tests = glob.glob("compile-tests/*.rs")
else:
    tests = files

num_pass = 0
num_fail = 0

for test in tests:
    if run_compile_test(test, leave_output, c):
        num_pass += 1
        print("Pass - %s" % test)
    else:
        num_fail += 1
        print("Fail - %s" % test)

print("Tests complete. %i passed, %i failed." % (num_pass, num_fail))
