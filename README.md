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
