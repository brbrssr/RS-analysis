import ctypes
import platform


pair = "BTCUSDT".encode('utf-8')
interval = "1h".encode('utf-8')
date = "2025-03-01T00:00:00Z".encode('utf-8')
os = platform.system().encode('utf-8')
min_window = "10".encode('utf-8')

if os == "Windows":
    rust_lib = ctypes.CDLL("./librslogic.dll")
elif os == "Linux":
    rust_lib = ctypes.CDLL("./librslogic.so")
else:
    rust_lib = ctypes.CDLL("./librslogic.so")


rust_lib.get_price.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p]
rust_lib.get_price.restype = ctypes.POINTER(ctypes.c_char)

rust_lib.get_rs.argtypes = [ctypes.c_char_p,ctypes.c_char_p,ctypes.c_char_p]
rust_lib.get_rs.restype = ctypes.POINTER(ctypes.c_char)

rust_lib.free_heap.argtypes = [ctypes.POINTER(ctypes.c_char)]
rust_lib.free_heap.restype = None


result_price_ptr = rust_lib.get_price(pair, interval, date, os)
result_price = ctypes.string_at(result_price_ptr).decode("utf-8")
print(result_price)
rust_lib.free_heap(result_price_ptr)

result_rs_ptr = rust_lib.get_rs(os, min_window, None)
result_rs = ctypes.string_at(result_rs_ptr).decode("utf-8")
print(result_rs)
rust_lib.free_heap(result_rs_ptr)
