// @generated
pub mod sf {
    pub mod substreams {
        pub mod rpc {
            // @@protoc_insertion_point(attribute:sf.substreams.rpc.v2)
            pub mod v2 {
                include!("sf.substreams.rpc.v2.rs");
                // @@protoc_insertion_point(sf.substreams.rpc.v2)
            }
        }

        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
}

#[path = "./sf.substreams.tokens.v1.rs"]
pub mod tokens;

#[path = "sf.substreams.sink.database.v1.rs"]
pub mod database;

#[path = "sa.trade.v1.rs"]
pub mod pb_sa_trade;