#![feature(type_alias_impl_trait)]

use pyo3::prelude::*;
use pyo3_built::pyo3_built;

pub mod client;
mod device;
pub mod login;
mod loguru;
pub mod message;
mod utils;
mod events;

#[pyfunction]
pub fn init_log(module: &PyModule) -> PyResult<()> {
    // 设置日志输出
    loguru::init(module)?;
    Ok(())
}

/// 构建信息。
#[allow(dead_code)]
pub mod build {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[pymodule]
#[doc(hidden)]
pub fn awr(py: Python, m: &PyModule) -> PyResult<()> {
    // 初始化
    m.add_function(wrap_pyfunction!(init_log, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__build__", pyo3_built!(py, build))?;
    m.add_function(wrap_pyfunction!(loguru::getframe, m)?)?;
    m.add_class::<login::Account>()?;
    // 客户端
    m.add_class::<client::Client>()?;
    // 消息元素
    m.add_class::<message::elements::At>()?;
    m.add_class::<message::elements::Face>()?;
    // 消息内容
    m.add_class::<message::content::MessageContent>()?;

    // Events
    m.add_class::<events::GroupMessage>()?;
    Ok(())
}
