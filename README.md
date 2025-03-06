# RS-analysis
***Checking the efficiency/precision of RS-analysis***
--
**💻Creators:**
- brbrssr
- san4ez1337
#### 🌲Project tree
```
rsanalysis v0.0.1(beta)
├── librslogic.so
├── main.py
├──── data
|     ├── price_series.json
|     ├── rs_series.json
|     └── scaled_rs_series.json
└── other_python_files
```
#### ⚙️Public lib functions
```
.
├── get_price_series(
|   pair: *const c_char,
|   interval: *const c_char,
|   date: *const c_char,
|   ) -> *mut c_char
|
├──free_rust_heap(*mut c_char)
|
└── ...
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
    "price": f64,
    "time": i32
}
```
