***ARFIMA model with RS-analysis***
--
### 🖊️Description  

#### rs_workspace
This directory contains the entire logic code of a program written in Rust as a python library.  
We work through pyo3 with maturin.
#### rsapp
This directory contains the visualization of time series, rs-analysis and etc.   
##### !ATTENTION!
Rsapp does not support the new version of logic written in rust, because we switched to the pyo3 system and the application developer did not have time to adapt the code.

### 🔨Using the model
Install maturin 
```
pip install maturin
```
Build the module in rs_workspace/sarfimax_model
```
/rs_workspace/sarfimax_model > maturin develop

```
Then you can use the model. Examples in debug_py.
