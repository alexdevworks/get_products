# auto-generated file
__all__ = ['lib', 'ffi']

import os
from example._native__ffi import ffi

lib = ffi.dlopen(os.path.join(os.path.dirname(__file__), '_native__lib.pyd'), 0)
del os
