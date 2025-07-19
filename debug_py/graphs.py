import numpy as np
import matplotlib.pyplot as plt


def visualize_time_series(originals, preds, train_pct=0.7, title="Time Series Visualization"):
    originals = np.asarray(originals)
    preds = np.asarray(preds)

    n = originals.shape[0]
    if n == 0:
        raise ValueError("originals is empty")
    if preds.size == 0:
        raise ValueError("preds is empty")

    split_idx = int(np.floor(n * train_pct))
    time_full = np.arange(n)
    time_preds = np.arange(split_idx, split_idx + preds.size)

    if split_idx + preds.size > n:
        preds = preds[: max(0, n - split_idx)]
        time_preds = np.arange(split_idx, split_idx + preds.size)
        print(f"Warning: predictions truncated to length {preds.size} to fit within original series.")

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
