pub mod rest {
    pub mod user;
}

pub mod usecase {
    pub mod user;
}

pub mod gateway {
    pub mod user;
}

pub mod port {
    pub mod user;
}

pub mod driver {
    pub mod db_driver;
    pub mod model;
}

pub mod domain {
    pub mod user;
}

pub mod utils {
    pub mod connection_pool;
    pub mod log;
    pub mod set_db;
}
