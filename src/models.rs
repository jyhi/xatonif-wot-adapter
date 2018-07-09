extern crate diesel;

use diesel::mysql::types::Datetime;


pub struct task_list {
    pub id: u32,
    pub user_id: u32,
    pub start_time: Datetime,
    pub description: String,
}
