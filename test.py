#!/usr/bin/env python

import os
import glob
import subprocess
import sys
import filecmp

def build_cbindgen():
    try:
        subprocess.check_output(["cargo", "build"])
        return True
    except subprocess.CalledProcessError:
        return False

def cbindgen(path, out, c, style):
    bin = ["cargo", "run", "--"]
    compile = [path, "-o", out]
    flags = []

    if c:
        flags += ["--lang", "c"]

    if style:
        flags += ["--style", style]

    config = path.replace(".rs", ".toml")
    if not os.path.isdir(path) and os.path.exists(config):
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

def run_compile_test(rust_src, should_verify, c, style=""):
    is_crate = os.path.isdir(rust_src)

    test_name = rust_src
    if is_crate:
        test_name = os.path.basename(rust_src[0:-1])
    else:
        test_name = os.path.splitext(os.path.basename(rust_src))[0]

    expectation = True
    if test_name.startswith("fail-"):
        expectation = False

    if c:
        subdir = style if style != "type" else ""
        out = os.path.join('tests/expectations/', subdir, test_name + ".c")
        verify = 'tests/expectations/__verify__.c'
    else:
        out = os.path.join('tests/expectations/', test_name + ".cpp")
        verify = 'tests/expectations/__verify__.cpp'

    try:
        if should_verify:
            cbindgen(rust_src, verify, c, style)

            if c:
                gcc(verify)
            else:
                gxx(verify)

            if not filecmp.cmp(out, verify):
                os.remove(verify)
                return False
            os.remove(verify)
        else:
            cbindgen(rust_src, out, c, style)

            if c:
                gcc(out)
            else:
                gxx(out)

    except subprocess.CalledProcessError:
        if os.path.exists(verify):
            os.remove(verify)
        return expectation == False

    return expectation == True

if not build_cbindgen():
    exit()

args = sys.argv[1:]
files = [x for x in args if not x.startswith("-")]
flags = [x for x in args if x.startswith("-")]

should_verify = False

for flag in flags:
    if flag == "-v":
        should_verify = True

tests = []
if len(files) == 0:
    tests = glob.glob("tests/rust/*.rs") + glob.glob("tests/rust/*/")
else:
    tests = files

num_pass = 0
num_fail = 0

# C

for test in tests:
    for style in ["type", "tag", "both"]:
        if run_compile_test(test, should_verify, True, style):
            num_pass += 1
            print("Pass - %s" % test)
        else:
            num_fail += 1
            print("Fail - %s" % test)

# C++

for test in tests:
    if run_compile_test(test, should_verify, False):
        num_pass += 1
        print("Pass - %s" % test)
    else:
        num_fail += 1
        print("Fail - %s" % test)

print("Tests complete. %i passed, %i failed." % (num_pass, num_fail))
