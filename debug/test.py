import ctypes
import platform
import time


"""
    Global parameters
"""
pair = "BTCUSDT".encode('utf-8')
interval = "1h".encode('utf-8')
date = "2018-01-01T00:00:00Z".encode('utf-8')
min_window = "10".encode('utf-8')
n_iter = "1000".encode('utf-8')

"""
    Detecting of the operation system
"""
rust_lib = ctypes.CDLL("./rslogic.dll") #It is necessary to define the OS 

"""
    Adding the rules of communication with the rust library for each function separately.
"""


rust_lib.get_price.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p] #Time series download function
rust_lib.get_price.restype = ctypes.POINTER(ctypes.c_char)

rust_lib.get_rs.argtypes = [ctypes.c_char_p,ctypes.c_char_p] # RS-analysis function
rust_lib.get_rs.restype = ctypes.POINTER(ctypes.c_char)

rust_lib.free_heap.argtypes = [ctypes.POINTER(ctypes.c_char)] # Clean heap function, deleting the allocated memory for error or success messages
rust_lib.free_heap.restype = None
start_time = time.time()
"""
    Calling of time series download function, and deleting of message after processing
"""
result_price_ptr = rust_lib.get_price(pair, interval, date)
result_price = ctypes.string_at(result_price_ptr).decode("utf-8")
print(result_price)
rust_lib.free_heap(result_price_ptr)

"""
    RS-analysis proccesing and deleting of messege after the output to the screen
"""

result_rs_ptr = rust_lib.get_rs( min_window, n_iter)
result_rs = ctypes.string_at(result_rs_ptr).decode("utf-8")
print(result_rs)
rust_lib.free_heap(result_rs_ptr)
print("Working time: ", time.time() - start_time)
