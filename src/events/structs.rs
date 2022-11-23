use pyo3::prelude::*;
use ricq::client::event::EventWithClient;

use ricq::structs as s;
use ricq_core::command::profile_service as ps;
use ricq_core::jce as js;
macro_rules! py_event {
    ($name: ident => $inner: ty) => {
        #[pyclass]
        #[allow(dead_code)]
        pub struct $name {
            e: $inner,
        }

        impl From<EventWithClient<$inner>> for $name {
            fn from(value: EventWithClient<$inner>) -> Self {
                Self { e: value.inner }
            }
        }
    };
}

#[pyclass]
pub struct LoginSuccess {
    #[pyo3(get)]
    uin: i64,
}

impl From<i64> for LoginSuccess {
    fn from(value: i64) -> Self {
        Self { uin: value }
    }
}

py_event!(GroupMessage => s::GroupMessage);
py_event!(GroupAudioMessage => s::GroupAudioMessage);
py_event!(FriendMessage => s::FriendMessage);
py_event!(FriendAudioMessage => s::FriendAudioMessage);
py_event!(GroupTempMessage => s::GroupTempMessage);
py_event!(GroupRequest => ps::JoinGroupRequest);
py_event!(SelfInvited => ps::SelfInvited);
py_event!(NewFriendRequest => ps::NewFriendRequest);
py_event!(NewMember => s::NewMember);
py_event!(GroupMute => s::GroupMute);
py_event!(FriendMessageRecall => s::FriendMessageRecall);
py_event!(GroupMessageRecall => s::GroupMessageRecall);
py_event!(NewFriend => s::FriendInfo);
py_event!(GroupLeave => s::GroupLeave);
py_event!(GroupDisband => s::GroupDisband);
py_event!(FriendPoke => s::FriendPoke);
py_event!(GroupNameUpdate => s::GroupNameUpdate);
py_event!(DeleteFriend => s::DeleteFriend);
py_event!(MemberPermissionChange => s::MemberPermissionChange);
py_event!(KickedOffline => js::RequestPushForceOffline);
py_event!(MSFOffline => js::RequestMSFForceOffline);

#[pymethods]
impl GroupMessage {
    #[getter]
    pub fn sender(&self) -> i64 {
        self.e.from_uin
    }
}
