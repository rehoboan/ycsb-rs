use crate::db::DB;
use sled;
use anyhow::Result;
use std::collections::HashMap;


pub struct Sled {
    db: sled::Db
}

impl Sled{
    pub fn new() -> Result<Self> {
        let db = sled::open("test-sled")?;
        Ok(Self {db})
    }
}

impl DB for Sled {
    fn init(&self) -> Result<()> {
        Ok(())
    }

    fn insert(&self, table: &str, key: &str, values: &HashMap<&str, String>) -> Result<()> {
        let insert_table = self.db.open_tree(table)?;
        for value in values.keys() {
            insert_table.insert(key,value)
        }
        Ok(())
    }

    fn read(&self, table: &str, key: &str, result: &mut HashMap<String, String>) -> Result<()> {
        let select_table = self.db.open_tree(table)?;
        match  select_table.get(key)? {
            None => Ok(()),
            Some(Values) => {
                for value in Values{
                    result.insert(key.into_string(),value)
                }
                Ok(())
            }

        }
    }
}


