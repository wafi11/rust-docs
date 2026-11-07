use bullmq::options::RedisConnectionOptions;

pub fn redis_connection() -> RedisConnectionOptions {
    RedisConnectionOptions::new().cluster_nodes([
        "redis://localhost:7000",
        "redis://localhost:7001",
        "redis://localhost:7002",
        "redis://localhost:7003",
        "redis://localhost:7004",
        "redis://localhost:7005",
    ])
}
