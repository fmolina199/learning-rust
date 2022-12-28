use actix::prelude::*;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct StrMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct GetMap(pub u64, pub u64);

#[derive(Clone, Message)]
#[rtype(result = "u64")]
pub struct Register(pub Recipient<StrMessage>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Unregister(pub u64);
