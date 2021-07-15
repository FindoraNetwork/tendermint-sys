//----------------------------------------
// Abstract types

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
