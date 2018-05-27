/// DB Wrapper DB.
/// For the moment it supports only postgresql. Further it can be done more universal

use postgres::{Connection, Error, TlsMode};

pub struct DbWrapper {
    connection: Connection,
}

impl DbWrapper {
    pub fn new(dsn: String) -> Result<DbWrapper, Error> {
        let result = Connection::connect(dsn.clone(), TlsMode::None);

        match result {
            Ok(connection) => Ok(DbWrapper {
                connection,
            }),
            Err(error) => Err(error),
        }
    }

    pub fn select (self, sql: &str)  {
        let result = &self.connection.query(sql, &[]).unwrap();
       // return result;
    }
}
