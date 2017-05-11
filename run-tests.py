#!/bin/python

import os
import glob
import subprocess
import sys

def cbindgen(rust_src, out):
    subprocess.check_output(["cargo", "run", "--", rust_src, "-o", out])

def gxx(src):
    subprocess.check_output(["g++", "-c", src, "-o", "compile-tests/tmp.o"])
    os.remove("compile-tests/tmp.o")

def run_compile_test(rust_src, leave_output):
    out = rust_src.replace(".rs", ".cpp")
    try:
        cbindgen(rust_src, out)
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

leave_output = False

if len(sys.argv) == 2 and sys.argv[1] == "-l":
    leave_output = True

for test in tests:
    if run_compile_test(test, leave_output):
        num_pass += 1
        print("Pass - %s" % test)
    else:
        num_fail += 1
        print("Fail - %s" % test)

print("Tests complete. %i passed, %i failed." % (num_pass, num_fail))
