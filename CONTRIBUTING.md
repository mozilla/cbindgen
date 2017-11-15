# Contributing

Thanks for wanting to contribute!

If you want help or mentorship, please file a Github issue or comment on an existing one and I'll be sure to provide guidance to the best of my ability.

Otherwise be sure to check out `ARCHITECTURE.md` for an overview on the internals.

## Filing a pull request

Check out [Servo's Github workflow](https://github.com/servo/servo/wiki/Github-workflow) for an overview on creating a pull request. Don't worry about requesting code review, as there is nothing formally setup for this repository. I review each pull request as soon as I can.

There is continuous integration setup for `cbindgen` using `travis`. It will automatically run `./test.py` which tests `cbindgen` on a series of rust files in `tests/rust/` and verifes that the output compiles using `gcc` or `g++`.

Please run `./test.py` before filing a pull request to be sure that all tests pass. Bonus points if you write a new test for your pull request!

The outputed headers for each test are committed and verified as being up to date in each pull request, so be sure to run `./test.py`.
