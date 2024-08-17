// must set PYTHONHOME and PYTHONPATH
#![allow(unused)]

use pyo3::{prelude::*, create_exception, exceptions::PyException
            , types::IntoPyDict};
use std::fs::File;

fn test1() -> PyResult<()> { 
    // 建立 CustomError 物件
    create_exception!(mymodule, CustomError, PyException);
    Python::with_gil(|py| {
        // 取得 Python 程式
        let code = std::fs::read_to_string("test.py")?;
        println!("code:\n{code}\n");
            
        // 建立 ctx 物件
        let ctx = [("CustomError", py.get_type_bound::<CustomError>())]
            .into_py_dict_bound(py);
        // 執行 Python 程式
        pyo3::py_run!(py, *ctx, &code);
        Ok(())
    })
}

fn test2() -> PyResult<()> { 
    let data = (1,3,2);
    let py_app = include_str!( // 檔案路徑
        concat!(env!("CARGO_MANIFEST_DIR"), "/test2.py")); 
    Python::with_gil(|py| {
        // 命名參數
        let kwargs = [("c", "r")].into_py_dict_bound(py);
        let app: Py<PyAny> = PyModule::from_code_bound(
            py, py_app, "", "")?
            .getattr("run")?  // 取得run函數
            .call((data,), Some(&kwargs))? // 填入參數值
            .into();
        //app.call0(py);  // 呼叫無參數的函數
        //app.call1(py, data); // 呼叫1個參數的函數
        Ok(())
    })
}

fn main() { 
    // 呼叫 Python 檔案
    test1().unwrap();
    
    // 呼叫 Python 函數
    test2().unwrap();
}