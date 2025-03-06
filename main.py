import ctypes
import platform


pair = "BTCUSDT".encode('utf-8')
interval = "1s".encode('utf-8')
date = "2024-03-06T00:00:00Z".encode('utf-8')
os = platform.system().encode('utf-8')


rust_lib = ctypes.CDLL("./librslogic.dll")


rust_lib.get_price_series.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p]
rust_lib.get_price_series.restype = ctypes.POINTER(ctypes.c_char)

rust_lib.free_rust_heap.argtypes = [ctypes.POINTER(ctypes.c_char)]
rust_lib.free_rust_heap.restype = None


result_ptr = rust_lib.get_price_series(pair, interval, date, os)
result = ctypes.string_at(result_ptr).decode("utf-8")
print(result)
rust_lib.free_rust_heap(result_ptr)
