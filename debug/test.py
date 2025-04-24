import ctypes
import json
import numpy as np
from math import log
from pathlib import Path
import matplotlib.pyplot as plt
import time

"""
    Global parameters
"""
pair = "BTCUSDT".encode('utf-8')
interval = "1h".encode('utf-8')
date = "2020-01-01T00:00:00Z".encode('utf-8')
min_window = "10".encode('utf-8')
n_iter = "1000".encode('utf-8')
freq = "7".encode('utf-8')
alpha = "0.95".encode('utf-8')
ub = "0.499".encode('utf-8')
hybrid = "true".encode('utf-8')
max_iters = "100".encode('utf-8')

"""
    Detecting of the operation system
"""
rust_lib = ctypes.CDLL(".\\rslogic.dll") #It is necessary to define the OS

"""
    Adding the rules of communication with the rust library for each function separately.
"""


rust_lib.get_price.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p] #Time series download function
rust_lib.get_price.restype = ctypes.POINTER(ctypes.c_char)

rust_lib.get_rs.argtypes = [ctypes.c_char_p,ctypes.c_char_p] # RS-analysis function
rust_lib.get_rs.restype = ctypes.POINTER(ctypes.c_char)

rust_lib.test.argtypes = [ctypes.c_char_p,ctypes.c_char_p,ctypes.c_char_p,ctypes.c_char_p,ctypes.c_char_p]
rust_lib.test.restype = ctypes.POINTER(ctypes.c_char)

rust_lib.free_heap.argtypes = [ctypes.POINTER(ctypes.c_char)] # Clean heap function, deleting the allocated memory for error or success messages
rust_lib.free_heap.restype = None

"""
    Table of params
"""
def print_parameter_table(params):
    """Печатает параметры в виде красивой таблицы."""
    max_key_len = max(len(k) for k in params)
    print("+" + "-" * (max_key_len + 2) + "+" + "-" * 22 + "+")
    print(f"| {'Parameter'.ljust(max_key_len)} | Value".ljust(22) + " |")
    print("+" + "-" * (max_key_len + 2) + "+" + "-" * 22 + "+")
    for key, value in params.items():
        decoded_value = value.decode('utf-8') if isinstance(value, bytes) else str(value)
        print(f"| {key.ljust(max_key_len)} | {decoded_value.ljust(20)} |")
    print("+" + "-" * (max_key_len + 2) + "+" + "-" * 22 + "+")

# Параметры
params = {
    "pair": pair,
    "interval": interval,
    "date": date,
    "min_window": min_window,
    "n_iter": n_iter,
    "freq": freq,
    "alpha": alpha,
    "ub": ub,
    "hybrid": hybrid,
    "max_iters": max_iters
}

# Вывод
print_parameter_table(params)
"""
    Calling of time series download function, and deleting of message after processing
"""
start_time = time.time()
result_price_ptr = rust_lib.get_price(pair, interval, date)
result_price = ctypes.string_at(result_price_ptr).decode("utf-8")
print(result_price)
rust_lib.free_heap(result_price_ptr)
print("Time of process[Load series]: ", time.time() - start_time, "sec\n")
"""
    RS-analysis proccesing and deleting of messege after the output to the screen
"""
start_time = time.time()
result_rs_ptr = rust_lib.get_rs( min_window, n_iter)
result_rs = ctypes.string_at(result_rs_ptr).decode("utf-8")
print(result_rs)
rust_lib.free_heap(result_rs_ptr)
print("Time of process[RS-analysis]: ", time.time() - start_time, "sec\n")
"""
    Test: Box_Cox
"""
start_time = time.time()
result_test_ptr = rust_lib.test(freq,alpha,ub,hybrid,max_iters)
result_test = ctypes.string_at(result_test_ptr).decode("utf-8")
print(result_test)
rust_lib.free_heap(result_test_ptr)
print("Time of process[Box-Cox + Z score]: ", time.time() - start_time, "sec\n")
"""
    Visualize 
"""
def load_price_data(filepath):
    """Загружает JSON с ценами из файла."""
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)
    return [entry["price"] for entry in data]


import json


def variance(data):
    if len(data) == 0:
        raise ValueError("Массив пуст")

    mean = sum(data) / len(data)
    return sum((x - mean) ** 2 for x in data) / len(data)


def load_float_series(filepath):
    """Загружает список чисел из JSON-файла."""
    with open(filepath, 'r', encoding='utf-8') as f:
        data = json.load(f)

    if not isinstance(data, list) or not all(isinstance(x, (int, float)) for x in data):
        raise ValueError("file could contain (float).")

    return data


def plot_price_series(original_path, transformed_path):
    """Строит графики оригинального и трансформированного рядов."""
    original = load_price_data(original_path)
    transformed = load_float_series(transformed_path)

    print("Variance: ",variance(transformed))

    plt.figure(figsize=(12, 6))

    plt.plot(original, label='Original series', color='blue', linewidth=2)
    plt.plot(transformed, label='transformed series', color='green', linestyle='--', linewidth=2)

    plt.title("PLot")
    plt.xlabel("Index")
    plt.ylabel("Cost")
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.show()

plot_price_series(".\\data\\price_series.json", ".\\data\\test.json")