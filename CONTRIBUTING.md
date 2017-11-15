# Contributing

Thanks for wanting to contribute!

If you want help or mentorship, please file a Github issue and I'll be sure to provide guidance to the best of my ability.

Otherwise be sure to check out `ARCHITECTURE.md` for an overview on the internals.

## Filing a pull request

Check out [Servo's Github workflow](https://github.com/servo/servo/wiki/Github-workflow) for an overview on creating a pull request.

Don't worry about requesting code review, as there is nothing formally setup for this repository. I try and review each pull request as soon as I can.

There is continuous integration setup for `cbindgen` using [travis](https://travis-ci.org/). It automatically runs `./test.py` which runs `cbindgen` against a series of rust files from `tests/rust/` and checks that the output compiles using `gcc` or `g++`.

Please run `./test.py` before filing a pull request to be sure that all tests pass. This will also update the test expectations.

Bonus points if you write a new test for your pull request!
