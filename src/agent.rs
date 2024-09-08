use std::fmt::Debug;

pub trait Agent: Debug {
    fn step(&self) -> bool;
}
