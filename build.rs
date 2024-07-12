fn main() {
    tonic_build::configure()
        .include_file("_includes.rs")
        .build_server(false)
        .compile(
            &[
                "proto/authzed/api/v1/permission_service.proto",
                "proto/authzed/api/v1/watch_service.proto",
                "proto/authzed/api/v1/experimental_service.proto",
                "proto/authzed/api/v1/schema_service.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
