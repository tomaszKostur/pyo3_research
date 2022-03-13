1. apache license
2. simple as using macros. in rust cpython it seems to be more complex
3. pyo3 seems to work with stable rust release
4. so libname should be same as output python module (at least by default)
5. there is ‘maturin’ project that claims to have ability to export downloadable and installable modules to python regitstry
6. .so seems to work as release and debug builds
7. pyfunction signatures are not generated automatically. use “test_signature”
8. python classes implemented in rust
9. Allocation of data, Rust side or python side ?