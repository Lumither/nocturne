// todo: rebuild with static OnceCell

// Database related
pub mod database {
    pub mod var_name {
        pub const URI: &str = "DATABASE_URL";
        pub const USERNAME: &str = "POSTGRES_USER";
        pub const PASSWORD: &str = "POSTGRES_PASSWORD";
        pub const HOST: &str = "POSTGRES_HOST";
        pub const PORT: &str = "POSTGRES_PORT";
        pub const DBNAME: &str = "POSTGRES_DB";
    }

    pub mod default_value {
        pub const USERNAME: &str = "";
        pub const PASSWORD: &str = "";
        pub const HOST: &str = "localhost";
        pub const PORT: &str = "5432";
        pub const DBNAME: &str = "nocturne";
        pub const DATABASE_TYPE: &str = "postgresql";
    }
}

pub mod server {
    pub mod var_name {
        pub const PORT: &str = "BACKEND_PORT";
        pub const HOST: &str = "BACKEND_HOST";
    }

    pub mod default_value {
        pub const PORT: u32 = 3001;
        pub const HOST: &str = "0.0.0.0";
    }
}

pub mod general {
    pub mod var_name {
        pub const LOG_DIR: &str = "LOG_DIR";
        pub const WORK_DIR: &str = "WORK_DIR";
        pub const BLOG_GIT_URL: &str = "BLOG_GIT_URL";
        pub const BLOG_GIT_REMOTE_BRANCH: &str = "BLOG_GIT_REMOTE_BRANCH";
        pub const BLOG_GIT_REMOTE_NAME: &str = "BLOG_GIT_REMOTE_NAME";
    }

    pub mod default_value {
        pub const WORK_DIR: &str = "~/.nocturne";
        pub const BLOG_GIT_REMOTE_BRANCH: &str = "master";
        pub const BLOG_GIT_REMOTE_NAME: &str = "origin";
    }
}
