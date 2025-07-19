from sarfimax_model import sarfimax
import numpy as np
import json
import load_data
import common
from sklearn.model_selection import train_test_split
from graphs import visualize_time_series


symbol = "BTCUSDT"
interval = "1h"
since = "2025-06-01T00:00:00Z"
output = "price_series.json"
split_size = 0.7
horizon = 100

common.file_clean(path="price_series.json")
print("File cleaned")
load_data.get_price_series(symbol, interval, since, output)


with open('./price_series.json', 'r') as f:
    input_data = np.array(json.load(f))


train, test = train_test_split(input_data, test_size=split_size)

freq = 7
alpha = 0.95
ub = 0.499
hybrid = True
min_window = 50
rs_iters = 2000
max_box_cox_iters = 200
nlags = 20
max_iters_optim = 20
optimizer = "bic" # "aic", "aic_c" or "bic"



model = sarfimax(
    freq, 
    alpha, 
    ub,
    hybrid, 
    max_box_cox_iters, 
    min_window,
    rs_iters,
    nlags,
    max_iters_optim,
    optimizer
)

model.fit(train)

preds = model.forecast(test, horizon)
params = model.get_params()
psi = model.get_psi()
print("Params[p,Herst,q]: ", params[0], params[1] + 0.5, params[2])
print("Psi vector: ", psi)
print("preds: ", preds)

visualize_time_series(input_data, preds, split_size, title="Time Series Visualization")