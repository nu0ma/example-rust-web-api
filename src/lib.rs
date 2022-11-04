pub mod rest {
    pub mod member;
}

pub mod usecase {
    pub mod member;
}

pub mod gateway {
    pub mod member;
}

pub mod port {
    pub mod member;
}

pub mod driver {
    pub mod db_driver;
    pub mod model;
}

pub mod domain {
    pub mod member;
}

pub mod utils {
    pub mod connection_pool;
    pub mod log;
    pub mod set_db;
}
