pub mod other;

#[path = "other2.rs"]
pub mod other2;

pub mod other3 {
    #[path = "other4.rs"]
    pub mod other4;
}
