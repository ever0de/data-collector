use {
    async_trait::async_trait,
    gluesql::{
        core::store::{GStore, GStoreMut},
        prelude::Glue,
    },
    std::{
        fmt::Debug,
        marker::{PhantomData, Send, Sync},
    },
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

    pub async fn run(&mut self) -> Result<()> {
        todo!()
    }
}

#[async_trait]
pub trait Scan<Row: Debug> {
    async fn scan(&self, table_name: &str) -> Result<Box<dyn Iterator<Item = Row>>>;
}

#[async_trait]
pub trait Save<Row: Debug> {
    async fn save(&mut self, row: Row) -> Result<()>;
}

#[async_trait]
impl<Row: Debug + Sync, Storage: GStore<Row> + GStoreMut<Row> + Sync> Scan<Row>
    for DataCollector<Row, Storage>
{
    async fn scan(&self, _table_name: &str) -> Result<Box<dyn Iterator<Item = Row>>> {
        unimplemented!()
    }
}

#[async_trait]
impl<Row: Debug + Sync + Send, Storage: GStore<Row> + GStoreMut<Row> + Sync + Send> Save<Row>
    for DataCollector<Row, Storage>
{
    async fn save(&mut self, _row: Row) -> Result<()> {
        unimplemented!()
    }
}
