# RS-analysis
***Checking the efficiency/precision of RS-analysis***
--
**💻Creators:**
- brbrssr
- san4ez1337
#### 🌲Project tree
```
rsanalysis v0.0.1(beta)
├── librsanalysis.so
├── main.py
├──── data
|     ├── price_series.json
|     ├── rs_series.json
|     └── scaled_rs_series.json
└── other_python_files
```
#### ⚙️Public lib functions
```
rslib.
├── get_price_series(pair,interval,date)
├── get_rs_series(pair,interval,date)
└── get_scaled_rs_series(pair,interval,date,scale)
```
##### 🕹Parameter values
```Rust
struct param {
	pair: String, // ex. "BTCUSDT"
	interval: String, // ex. "1c", "15m","1h" and etc
	date: String, // format: ISO 8601, ex. "2014-07-08T09:10:11Z"
}
```
#### 🧮Json price_series structure
``` Rust
struct CandleData {
    time: i64, //UNIX time
    price: String,
}
```
