import ctypes
import platform
import time
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from scipy.stats import boxcox
from arch import arch_model
import json

"""
    Global parameters
"""
pair = "BTCUSDT".encode('utf-8')
interval = "1h".encode('utf-8')
date = "2024-01-01T00:00:00Z".encode('utf-8')
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

# ================================
#   Variance normalize
# ================================

CONFIG = {
    "filename": ".\\data\\price_series.json",
    "apply_trend_diff": True,
    "apply_seasonal_diff": True,
    "seasonal_period": 7,
    "use_boxcox": True,
    "boxcox_shift": None,
    "use_garch": True,
    "garch_p": 1,
    "garch_q": 1,
}

def load_data(filename):
    with open(filename, "r") as f:
        data = json.load(f)
    return [entry["price"] for entry in data]

def preprocess_data(prices, config):
    prices_series = pd.Series(prices)
    if config["apply_trend_diff"]:
        prices_diff = prices_series.diff().dropna()
    else:
        prices_diff = prices_series.copy()
    if config["apply_seasonal_diff"]:
        seasonal_diff = prices_diff.diff(config["seasonal_period"]).dropna()
    else:
        seasonal_diff = prices_diff.copy()
    if config["use_boxcox"]:
        if config["boxcox_shift"] is None:
            shift = abs(seasonal_diff.min()) + 1e-6
        else:
            shift = config["boxcox_shift"]
        prices_shifted = seasonal_diff + shift
        prices_boxcox, lmbda = boxcox(prices_shifted)
    else:
        prices_boxcox = seasonal_diff.copy()
        lmbda = None
    if config["use_garch"]:
        model = arch_model(prices_boxcox, vol="Garch", p=config["garch_p"], q=config["garch_q"])
        res = model.fit(disp="off")
        garch_volatility = res.conditional_volatility
    else:
        garch_volatility = None
    normalized = (prices_boxcox - np.mean(prices_boxcox)) / np.std(prices_boxcox)
    # normalized = prices_boxcox
    return prices_series, normalized, garch_volatility, lmbda

def plot_series(original, normalized, garch_volatility):
    plt.figure(figsize=(12, 10))
    plt.subplot(3, 1, 1)
    plt.plot(original, label="Оригинальный ряд", color="blue")
    plt.title("Оригинальный временной ряд")
    plt.xlabel("Время")
    plt.ylabel("Цена")
    plt.legend()
    plt.grid()
    plt.subplot(3, 1, 2)
    plt.plot(normalized, label="Нормализованный (Box-Cox + Z-score)", color="red")
    plt.title("Нормализованный ряд")
    plt.xlabel("Время")
    plt.ylabel("Значение")
    plt.legend()
    plt.grid()
    if garch_volatility is not None:
        plt.subplot(3, 1, 3)
        plt.plot(garch_volatility, label="GARCH волатильность", color="green")
        plt.title("GARCH волатильность")
        plt.xlabel("Время")
        plt.ylabel("Волатильность")
        plt.legend()
        plt.grid()
    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    prices = load_data(CONFIG["filename"])
    original, normalized, garch_volatility, lmbda = preprocess_data(prices, CONFIG)
    if lmbda is not None:
        print(f"Оптимальный параметр λ для Box-Cox: {lmbda}")
    plot_series(original, normalized, garch_volatility)
