pub async fn set_db() {
    let db_pool = connection_pool::create_pool().await;
    connection_pool::DB_POOL.set(db_pool).unwrap();
}
