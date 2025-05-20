import ctypes
import json
import matplotlib.pyplot as plt
import time
import seaborn as sns
import pandas as pd
from scipy.stats import boxcox, gaussian_kde
import numpy as np


def inverse_box_cox(y_transformed, lambda_):
    if lambda_ == 0:
        return np.exp(y_transformed)
    else:
        return (lambda_ * y_transformed + 1) ** (1 / lambda_)

def inverse_z_score(z, mu, sigma):
    return z * sigma + mu


"""
    Global parameters
"""
pair = "BTCUSDT".encode('utf-8')                # defined by the user
interval = "1h".encode('utf-8')                 # defined by the user
date = "2025-04-01T00:00:00Z".encode('utf-8')   # defined by the user
min_window = "50".encode('utf-8')               # predetermined
n_iter = "1000".encode('utf-8')                 # configured separately
freq = "7".encode('utf-8')                      # configured separately
alpha = "0.95".encode('utf-8')                  # configured separately
ub = "0.499".encode('utf-8')                    # configured separately
hybrid = "true".encode('utf-8')                 # configured separately
max_iters = "100".encode('utf-8')               # configured separately
nlags = "20".encode('utf-8')                    # configured separately
max_iters_grid = "50".encode('utf-8')           # configured separately

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

rust_lib.get_arfima.argtypes = [ctypes.c_char_p,ctypes.c_char_p,ctypes.c_char_p,ctypes.c_char_p,ctypes.c_char_p,ctypes.c_char_p,ctypes.c_char_p]
rust_lib.get_arfima.restype = ctypes.POINTER(ctypes.c_char)

rust_lib.free_heap.argtypes = [ctypes.POINTER(ctypes.c_char)] # Clean heap function, deleting the allocated memory for error or success messages
rust_lib.free_heap.restype = None

"""
    Table of params
"""
def print_parameter_table(params):
    max_key_len = max(len(k) for k in params)
    print("+" + "-" * (max_key_len + 2) + "+" + "-" * 22 + "+")
    print(f"| {'Parameter'.ljust(max_key_len)} | Value      ".ljust(22) + max_key_len*" " + "|")
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
    "max_iters": max_iters,
}

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
    Test: 
"""
start_time = time.time()
result_test_ptr = rust_lib.get_arfima(freq,alpha,ub,hybrid,max_iters,nlags,max_iters_grid)
result_test = ctypes.string_at(result_test_ptr).decode("utf-8")
print(result_test)
rust_lib.free_heap(result_test_ptr)
print("Time of process[Box-Cox + Z score]: ", time.time() - start_time, "sec\n")
"""
    Visualize 
"""
def variance(data):
    if len(data) == 0:
        raise ValueError("Массив пуст")

    mean = sum(data) / len(data)
    return sum((x - mean) ** 2 for x in data) / len(data)


# 1) Load original price series
with open('./data/price_series.json', 'r') as f:
    original = np.array(json.load(f))

# 2) Load pre‑transformed data from test.json
with open('./data/test.json', 'r') as f:
    loaded = np.array(json.load(f))

with open('./data/preds.json', 'r') as f:
    preds = np.array(json.load(f))

# Helper to plot histogram + KDE
def plot_distribution(ax, data, title, bins=30):
    data = np.asarray(data).ravel()
    ax.hist(data, bins=bins, density=True, alpha=0.6, edgecolor='black')
    if np.std(data) > 0:
        try:
            kde = gaussian_kde(data)
        except np.linalg.LinAlgError:
            data2 = data + 1e-6 * np.random.randn(len(data))
            kde = gaussian_kde(data2)
        xs = np.linspace(data.min(), data.max(), 200)
        ax.plot(xs, kde(xs), lw=2)
    else:
        ax.text(0.5, 0.5, "Constant data\nno KDE",
                ha="center", va="center", transform=ax.transAxes)
    ax.set_title(title)
    ax.set_xlabel('Value')
    ax.set_ylabel('Density')

# 5) Draw four side‑by‑side plots
fig, axes = plt.subplots(1, 2, figsize=(14, 5))

plot_distribution(axes[0], original,'Original Data')
plot_distribution(axes[1], loaded,'Rust Box-Cox + Z-score + Frac_diff')
plt.tight_layout()
plt.show()


def visualize_time_series(originals, preds, title="Time Series Visualization"):
    """
    Визуализирует исходный временной ряд и предсказания, показывая различия.

    Args:
        original (np.array): Исходные данные (100%).
        pred (np.array): Предсказания для последних 30%.
        title (str): Заголовок графика.

    Returns:
        None: Отображает график.
    """
    # Проверка, что входные данные - numpy массивы
    if not isinstance(originals, np.ndarray) or not isinstance(preds, np.ndarray):
        raise TypeError("original и pred должны быть numpy массивами")

    # Проверка на пустые массивы
    if len(originals) == 0:
        raise ValueError("Массив original пустой")
    if len(preds) == 0:
        raise ValueError("Массив pred пустой")

    # Проверка на NaN или бесконечные значения
    if np.any(np.isnan(originals)) or np.any(np.isinf(original)):
        raise ValueError("Массив original содержит NaN или бесконечные значения")
    if np.any(np.isnan(preds)) or np.any(np.isinf(preds)):
        raise ValueError("Массив pred содержит NaN или бесконечные значения")

    # Длина исходного массива
    n = len(originals)

    # Вычисление индекса раздела (70%)
    split_idx = int(n * 0.7)
    expected_pred_len = n - split_idx  # Ожидаемая длина pred (30%)

    # Проверка длины pred
    if len(preds) != expected_pred_len:
        print(f"Предупреждение: pred имеет длину {len(preds)}, ожидалось {expected_pred_len}")
        # Используем фактическую длину pred
        pred_len = len(preds)
    else:
        pred_len = expected_pred_len

    # Создание оси времени
    time = np.arange(n)  # Полная длина для original

    # Ось времени для pred и последних 30% original
    time_pred = np.arange(split_idx, split_idx + pred_len)

    # Проверка, чтобы избежать выхода за границы
    if split_idx + pred_len > n:
        print(f"Предупреждение: pred выходит за границы original, обрезаем до {n - split_idx}")
        pred_len = n - split_idx
        pred = preds[:pred_len]
        time_pred = np.arange(split_idx, split_idx + pred_len)

    # Создание графика
    plt.figure(figsize=(12, 6))

    # Полный исходный ряд (синий)
    plt.plot(time, originals, color='blue', label='Original Data (100%)')

    # Реальные данные для последних 30% (зеленый, ограничено длиной pred)
    plt.plot(time_pred, originals[split_idx:split_idx + pred_len],
             color='green', label='Actual Last 30%', linestyle='--')

    # Предсказания (красный)
    plt.plot(time_pred, preds, color='red', label='Predicted')

    # Оформление графика
    plt.axvline(x=split_idx, color='gray', linestyle=':', label='70% Split')
    plt.xlabel('Time')
    plt.ylabel('Value')
    plt.title(title)
    plt.legend()
    plt.grid(True)
    plt.tight_layout()

    # Отображение графика
    plt.show()

visualize_time_series(loaded, preds, title="Time Series Visualization")