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
- The function ***get_price()*** returns **mut c_char*.
  
  *parameters*
  > symbol: *const c_char,
  
  > interval: *const c_char,

  > date: *const c_char,

  > os: *const c_char,

- The function ***get_rs()*** returns **mut c_char*.
  *parameters*
  > os: *const c_char,

  > min_window: *const c_char,
  
- The function ***free_heap()*** doesn't accept anything and clears the allocated memory.

❄More details on the file ./rs_workspace/readme.md

#### 🧮Json price_series structure
``` Rust
[
{
    "price": f64,
}
]
```
#### 🧮Json rs_series structure
```
[
{
  rs: f64,
  window: usize,
}
]
```
