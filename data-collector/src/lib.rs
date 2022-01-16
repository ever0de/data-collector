use {
    gluesql::core::store::{GStore, GStoreMut},
    std::fmt::Debug,
    std::marker::PhantomData,
};

pub mod error;
pub mod result;

use result::Result;

pub struct DataCollector<Row: Debug, Write: GStoreMut<Row>, Read: GStore<Row>> {
    pub write: Write,
    pub read: Read,
    _marker: PhantomData<Row>,
}

pub trait Scan<Row> {
    fn scan(&self) -> Result<Box<dyn Iterator<Item = Row>>>;
}

pub trait Save<Row> {
    fn save(&mut self, row: Row) -> Result<()>;
}
