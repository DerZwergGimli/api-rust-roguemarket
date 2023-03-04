#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseChanges {
    #[prost(message, repeated, tag="1")]
    pub table_changes: ::prost::alloc::vec::Vec<TableChange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableChange {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pk: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub ordinal: u64,
    #[prost(enumeration="table_change::Operation", tag="4")]
    pub operation: i32,
    #[prost(message, repeated, tag="5")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
}
/// Nested message and enum types in `TableChange`.
pub mod table_change {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        /// Protobuf default should not be used, this is used so that the consume can ensure that the value was actually specified
        Unset = 0,
        Create = 1,
        Update = 2,
        Delete = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_value: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub old_value: ::prost::alloc::string::String,
}
