# RS-analysis
***Checking the efficiency/precision of RS-analysis***
--
**💻Creators:**
- brbrssr
- san4ez1337
----
### 🌲Project tree
```
rsapp
├── librslogic.so // for Linux
├── rslogic.dll // for win
├── main.py
├──── data
|     ├── price_series.json
|     ├── rs_series.json
|     └── scaled_rs_series.json
└── other_python_files
```
### 🖊️Description 
#### rs_workspace
This directory contains the entire logic code of a program written in Rust as a library. 
More information about the code structure can be found in the file ./rs_workspace/readme.md
#### rsapp
This directory contains the visualization of time series, rs-analysis and etc. 
Also more information in ./rsapp/readme.md
#### debug 
This folder contains the test scripts used in development. 
They do not have complex algorithms and are needed as an example of Python working with a self-written Rust library.
**to be continued**
