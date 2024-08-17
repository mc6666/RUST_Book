// must set PYTHONHOME and PYTHONPATH
#![allow(unused)]

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        // 引用 sys 套件
        let sys = py.import_bound("sys")?;
        
        // 取得 Python 版本
        let version: String = sys.getattr("version")?.extract()?;

        // 引用 os 套件
        let locals = [("os", py.import_bound("os")?)]
            .into_py_dict_bound(py);
        
        // 取得環境變數
        let code = 
            "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
            
        // 執行 Python 程式
        let user: String = py.eval_bound(code, None, Some(&locals))?
            .extract()?;

        // 顯示使用者帳號及 Python 版本
        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}