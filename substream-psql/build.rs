fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(
            &["sf/substreams/v1/substreams.proto", "sf/substreams/v1/package.proto", "sf/substreams/sink/database/v1/database.proto"],
            &["dependencies/substreams/proto", "dependencies/substreams/proto", "dependencies/substreams-sink-database-changes/proto"],
        )
        .unwrap();
}
