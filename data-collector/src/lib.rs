use {
    gluesql::{
        core::store::{GStore, GStoreMut},
        prelude::Glue,
    },
    std::{fmt::Debug, marker::PhantomData},
};

pub mod error;
pub mod result;

pub use result::Result;

pub struct DataCollector<Row: Debug, Storage: GStore<Row> + GStoreMut<Row>> {
    pub glue: Glue<Row, Storage>,
    _marker: PhantomData<Row>,
}

impl<Row: Debug, Storage: GStore<Row> + GStoreMut<Row>> DataCollector<Row, Storage> {
    pub fn new(storage: Storage) -> Self {
        Self {
            glue: Glue::new(storage),
            _marker: PhantomData,
        }
    }
}

pub trait Scan<Row: Debug> {
    fn scan(&self, table_name: &str) -> Result<Box<dyn Iterator<Item = Row>>>;
}

pub trait Save<Row: Debug> {
    fn save(&mut self, row: Row) -> Result<()>;
}

impl<Row: Debug, Storage: GStore<Row> + GStoreMut<Row>> Scan<Row> for DataCollector<Row, Storage> {
    fn scan(&self, _table_name: &str) -> Result<Box<dyn Iterator<Item = Row>>> {
        unimplemented!()
    }
}

impl<Row: Debug, Storage: GStore<Row> + GStoreMut<Row>> Save<Row> for DataCollector<Row, Storage> {
    fn save(&mut self, _row: Row) -> Result<()> {
        unimplemented!()
    }
}
