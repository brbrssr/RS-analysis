# RS-analysis
***Checking the efficiency/precision of RS-analysis***
--
**💻Creators:**
- brbrssr
- san4ez1337
----
#### 🌲Project tree
```
rsanalysis
├── librslogic.so // or .dll on win 
├── main.py
├──── data
|     ├── price_series.json
|     ├── rs_series.json
|     └── scaled_rs_series.json
└── other_python_files
```
#### ⚙️Public lib functions
- The function ***get_price_series*** takes *const c_char as parameters and returns **mut c_char*.

> pair: *const c_char

> interval: *const c_char

> date: *cosnt c_char

- The function ***free_rust_heap()*** doesn't accept anything and clears the allocated memory.
#### 🧮Json price_series structure
``` Rust
struct CandleData {
    "price": f64,
    "time": i32
}
```
