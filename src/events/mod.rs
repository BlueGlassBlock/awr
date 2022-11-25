mod structs;
use async_trait::async_trait;
use pyo3::{prelude::*, types::PyTuple};
use ricq::client::handler::Handler;
use ricq::client::handler::QEvent;
pub use structs::*;

macro_rules! mk_convert {
    ($($name: ident),*) => {
        fn convert(e: QEvent, py: Python) -> Py<PyAny> {
            match e{
                $(QEvent::$name(inner) => $name::from(inner).into_py(py)),*
            }
        }
    };
}

mk_convert!(
    Login,
    GroupMessage,
    GroupAudioMessage,
    FriendMessage,
    FriendAudioMessage,
    GroupTempMessage,
    GroupRequest,
    SelfInvited,
    NewFriendRequest,
    NewMember,
    GroupMute,
    FriendMessageRecall,
    NewFriend,
    GroupMessageRecall,
    GroupLeave,
    GroupDisband,
    FriendPoke,
    GroupNameUpdate,
    DeleteFriend,
    MemberPermissionChange,
    KickedOffline,
    MSFOffline
);

pub struct PyHandler {
    callback: Py<PyAny>,
}

impl PyHandler {
    pub fn new() -> PyResult<Self> {
        Ok(Self {
            callback: Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let cb_mod = py.import("awr.callback")?;
                Ok(cb_mod.getattr("callback")?.into_py(py))
            })?,
        })
    }
}

#[async_trait]
impl Handler for PyHandler {
    async fn handle(&self, e: QEvent) {
        Python::with_gil(|py| {
            let py_event = convert(e, py);
            let args = PyTuple::new(py, &[py_event]);
            // Using sync function here so that users can be *more careful* when interacting with callbacks.
            self.callback.call1(py, args).unwrap_or(py.None());
        })
    }
}
