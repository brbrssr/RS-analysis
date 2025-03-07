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
├── librslogic.so
├── rslogic.dll
├── main.py
├──── data
|     ├── price_series.json
|     ├── rs_series.json
|     └── scaled_rs_series.json
└── other_python_files
```
#### ⚙️Public lib functions
- The function ***get_price*** takes *const c_char as parameters and returns **mut c_char*.

> pair: *const c_char

> interval: *const c_char

> date: *cosnt c_char

- The function ***free_heap()*** doesn't accept anything and clears the allocated memory.
#### 🧮Json price_series structure
``` Rust
[
{
    "price": f64,
}
]
```
