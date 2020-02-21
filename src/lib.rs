#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate error_chain;

pub mod scryfall;
pub mod parser;
pub mod db;
pub mod data;
pub mod schema;
