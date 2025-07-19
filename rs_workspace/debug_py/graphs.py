import numpy as np
import matplotlib.pyplot as plt

def visualize_time_series(originals, preds, train_pct=0.7, title="Time Series Visualization"):
    """
    Визуализирует исходный временной ряд и предсказания, показывая их поверх оригинальных данных.

    Args:
        originals (array-like): Полный временной ряд.
        preds (array-like): Значения предсказаний.
        train_pct (float): Доля обучающей выборки от 0 до 1 (например, 0.8 для 80%).
        title (str): Заголовок графика.

    Returns:
        None: Отображает график.
    """
    # Преобразование в numpy-массивы
    originals = np.asarray(originals)
    preds = np.asarray(preds)

    # Проверки входных данных
    if originals.ndim != 1 or preds.ndim != 1:
        raise ValueError("originals и preds должны быть одномерными массивами")
    if not (0 < train_pct < 1):
        raise ValueError("train_pct должен быть числом между 0 и 1")

    n = originals.shape[0]
    if n == 0:
        raise ValueError("originals пустой массив")
    if preds.size == 0:
        raise ValueError("preds пустой массив")

    # Индекс конца обучающей выборки
    split_idx = int(np.floor(n * train_pct))

    # Осн. шкалы времени
    time_full = np.arange(n)

    # Временные метки для предсказаний
    time_preds = np.arange(split_idx, split_idx + preds.size)

    # Проверка границ
    if split_idx + preds.size > n:
        # Обрезаем preds, чтобы не выходить за границы
        preds = preds[: max(0, n - split_idx)]
        time_preds = np.arange(split_idx, split_idx + preds.size)
        print(f"Warning: predictions truncated to length {preds.size} to fit within original series.")

    # Построение графика
    plt.figure(figsize=(10, 5))
    plt.plot(time_full, originals, label='Original Series')
    plt.plot(time_preds, preds, label='Predictions', linestyle='--')
    plt.axvline(x=split_idx, color='gray', linestyle=':', label=f'Train/Test Split at {train_pct*100:.0f}%')
    plt.title(title)
    plt.xlabel('Time')
    plt.ylabel('Value')
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.show()
