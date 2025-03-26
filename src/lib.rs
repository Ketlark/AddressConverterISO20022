pub mod app {
    pub mod commands;
    pub mod config;
}

pub mod domain {
    pub mod errors;
    pub mod interfaces;
    pub mod models;
    pub mod repository;
}

pub mod infrastructure {
    pub mod dao;
    pub mod dto;
    pub mod factories;
    pub mod repository;
}

pub mod usecases;
