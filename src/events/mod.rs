mod structs;
use pyo3::prelude::*;
use ricq::client::handler::QEvent;
pub use structs::*;

pub fn convert(e: QEvent, py: Python<'_>) -> Py<PyAny> {
    match e {
        QEvent::Login(inner) => LoginSuccess::from(inner).into_py(py),
        QEvent::GroupMessage(inner) => GroupMessage::from(inner).into_py(py),
        QEvent::GroupAudioMessage(inner) => GroupAudioMessage::from(inner).into_py(py),
        QEvent::FriendMessage(inner) => FriendMessage::from(inner).into_py(py),
        QEvent::FriendAudioMessage(inner) => FriendAudioMessage::from(inner).into_py(py),
        QEvent::GroupTempMessage(inner) => GroupTempMessage::from(inner).into_py(py),
        QEvent::GroupRequest(inner) => GroupRequest::from(inner).into_py(py),
        QEvent::SelfInvited(inner) => SelfInvited::from(inner).into_py(py),
        QEvent::NewFriendRequest(inner) => NewFriendRequest::from(inner).into_py(py),
        QEvent::NewMember(inner) => NewMember::from(inner).into_py(py),
        QEvent::GroupMute(inner) => GroupMute::from(inner).into_py(py),
        QEvent::FriendMessageRecall(inner) => FriendMessageRecall::from(inner).into_py(py),
        QEvent::NewFriend(inner) => NewFriend::from(inner).into_py(py),
        QEvent::GroupMessageRecall(inner) => GroupMessageRecall::from(inner).into_py(py),
        QEvent::GroupLeave(inner) => GroupLeave::from(inner).into_py(py),
        QEvent::GroupDisband(inner) => GroupDisband::from(inner).into_py(py),
        QEvent::FriendPoke(inner) => FriendPoke::from(inner).into_py(py),
        QEvent::GroupNameUpdate(inner) => GroupNameUpdate::from(inner).into_py(py),
        QEvent::DeleteFriend(inner) => DeleteFriend::from(inner).into_py(py),
        QEvent::MemberPermissionChange(inner) => MemberPermissionChange::from(inner).into_py(py),
        QEvent::KickedOffline(inner) => KickedOffline::from(inner).into_py(py),
        QEvent::MSFOffline(inner) => MSFOffline::from(inner).into_py(py),
    }
}
