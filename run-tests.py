#!/bin/python

import os
import glob
import subprocess
import sys

def cbindgen(rust_src, out, c):
    if c:
        subprocess.check_output(["cargo", "run", "--", "--lang", "c", rust_src, "-o", out])
    else:
        subprocess.check_output(["cargo", "run", "--", rust_src, "-o", out])

def gcc(src):
    gcc_bin = os.environ.get('CC')
    if gcc_bin == None:
        gcc_bin = 'gcc'

    subprocess.check_output([gcc_bin, "-c", src, "-o", "compile-tests/tmp.o"])
    os.remove("compile-tests/tmp.o")

def gxx(src):
    gxx_bin = os.environ.get('CXX')
    if gxx_bin == None:
        gxx_bin = 'g++'

    subprocess.check_output([gxx_bin, "--std=c++11", "-c", src, "-o", "compile-tests/tmp.o"])
    os.remove("compile-tests/tmp.o")

def run_compile_test(rust_src, leave_output, c):
    if c:
        out = rust_src.replace(".rs", ".c")
    else:
        out = rust_src.replace(".rs", ".cpp")

    try:
        cbindgen(rust_src, out, c)

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

tests = glob.glob("compile-tests/*.rs")
num_pass = 0
num_fail = 0

flags = sys.argv[1:]
leave_output = False
c = False
for flag in flags:
    if flag == "-l":
        leave_output = True
    elif flag == "-c":
        c = True

for test in tests:
    if run_compile_test(test, leave_output, c):
        num_pass += 1
        print("Pass - %s" % test)
    else:
        num_fail += 1
        print("Fail - %s" % test)

print("Tests complete. %i passed, %i failed." % (num_pass, num_fail))
