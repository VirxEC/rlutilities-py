# RLUtilities pybinds

Produces 2 wheels for RLUtilties <-> Python interop: One for Windows, one for Linux, compatible with Python 3.7+ instead of one for each version of Python.

# Using these bindings

Everything you want is in the [latest release](https://github.com/VirxEC/rlutilities-rs/releases)!

Here's what the files are:
 - `rlutilities.zip` - Drop-in replacement for the default recommended RLUtilities Python bindings. Delete the old folder, unzip and drop in this replacement and have Windows & Linux support for Python 3.7+!
 - `rlutilities-X.X.X-cp37-abi3-win_amd64.whl` - The Windows-only wheel file for the bindings. You can `pip install file_name.whl` to try out RLUtilities in Python 3.7+ on Windows.
 - `rlutilities-X.X.X-cp37-abi3-manylinux_2_28_x86_64.whl` - The Linux-only wheel file for the bindings. You can `pip install file_name.whl` to try out RLUtilities in Python 3.7+ on Linux.
 - `rlutilities-X.X.X.tar.gz` - The minimal source code of this project. Unzip and it contains all the files you need to build the bindings yourself, for your plateform and architechure. Beware that this doesn't included any of the required tools/dependencies to build the bindings!
