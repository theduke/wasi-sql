use std::ops::Deref;

use crate::{
    bindings::sql_v1_alpha1::{self as sys, ValueParam},
    util::{Named, SharedOptional, UnsafeOptional, UnsafeSharedOptional},
};

use rusqlite::types::{ToSqlOutput, ValueRef};
use sys::SqlError;

struct Sqlite {}

#[derive(Debug)]
struct SqliteDriver;

#[derive(Debug)]
enum SqliteConnection {
    Plain(rusqlite::Connection),
    Pool(r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>),
}

impl Deref for SqliteConnection {
    type Target = rusqlite::Connection;

    fn deref(&self) -> &Self::Target {
        match self {
            SqliteConnection::Plain(c) => c,
            SqliteConnection::Pool(c) => c.deref(),
        }
    }
}

impl Named for SqliteConnection {
    const NAME: &'static str = "Connection";
}

type Connection = SharedOptional<SqliteConnection>;

impl From<rusqlite::Connection> for Connection {
    fn from(c: rusqlite::Connection) -> Self {
        SqliteConnection::Plain(c).into()
    }
}

impl From<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>> for Connection {
    fn from(c: r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>) -> Self {
        SqliteConnection::Pool(c).into()
    }
}

#[derive(Debug)]
struct SqliteDb {
    pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
}

#[derive(Debug, Clone)]
struct StatementResource {
    statement: UnsafeSharedOptional<rusqlite::Statement<'static>>,
    /// Exists to mark the connection as shared.
    _con: Connection,
}

impl<'a> Named for rusqlite::Statement<'a> {
    const NAME: &'static str = "Statement";
}

#[derive(Debug)]
struct RowsResource {
    rows: UnsafeSharedOptional<rusqlite::Rows<'static>>,
    statement: StatementResource,
}

impl<'a> Named for rusqlite::Rows<'a> {
    const NAME: &'static str = "Rows";
}

impl<'a> Named for rusqlite::Row<'a> {
    const NAME: &'static str = "Row";
}

#[derive(Debug)]
struct RowResource {
    row: UnsafeOptional<rusqlite::Row<'static>>,
    /// Exists to mark the rows as shared.
    _rows: UnsafeSharedOptional<rusqlite::Rows<'static>>,
}

pub struct State {
    data: Sqlite,
    tables: sys::SqlV1Alpha1Tables<Sqlite>,
}

impl State {
    pub fn new() -> Self {
        Self {
            data: Sqlite {},
            tables: Default::default(),
        }
    }

    pub fn add_to_linker<S>(
        linker: &mut wasmtime::Linker<S>,
        f: impl Fn(&mut S) -> &mut Self + Send + Sync + Copy + 'static,
    ) {
        sys::add_to_linker(linker, move |state| {
            let inner = f(state);
            (&mut inner.data, &mut inner.tables)
        })
        .unwrap();
    }
}

impl<'a> rusqlite::ToSql for ValueParam<'a> {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            ValueParam::Boolean(v) => {
                Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(if *v {
                    1
                } else {
                    0
                })))
            }
            ValueParam::Int8(v) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
                (*v).into(),
            ))),
            ValueParam::Int16(v) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
                (*v).into(),
            ))),
            ValueParam::Int32(v) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
                (*v).into(),
            ))),
            ValueParam::Int64(v) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(*v))),
            ValueParam::Uint8(v) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
                (*v).into(),
            ))),
            ValueParam::Uint16(v) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
                (*v).into(),
            ))),
            ValueParam::Uint32(v) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(
                (*v).into(),
            ))),
            ValueParam::Uint64(v) => {
                let x: i64 = (*v).try_into().map_err(|_| {
                    rusqlite::Error::ToSqlConversionFailure(
                        "invalid parameter Uint64: exceeds sqlite maximum integer range (i64)"
                            .to_string()
                            .into(),
                    )
                })?;
                Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(x)))
            }
            ValueParam::Floating32(v) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Real(
                (*v).into(),
            ))),
            ValueParam::Floating64(v) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Real(
                (*v).into(),
            ))),
            ValueParam::Str(v) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Text(
                v.to_string(),
            ))),
            ValueParam::Bytes(v) => {
                Ok(ToSqlOutput::Owned(rusqlite::types::Value::Blob(v.to_vec())))
            }
            ValueParam::Raw(_raw) => Err(rusqlite::Error::ToSqlConversionFailure(
                "invalid parameter Raw: not supported".to_string().into(),
            )),
            ValueParam::Null => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Null)),
        }
    }
}

impl rusqlite::types::FromSql for sys::ValueResult {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Null => Ok(sys::ValueResult::Null),
            ValueRef::Integer(v) => Ok(sys::ValueResult::Int64(v)),
            ValueRef::Real(v) => Ok(sys::ValueResult::Floating64(v)),
            ValueRef::Text(v) => {
                if let Ok(s) = std::str::from_utf8(v) {
                    Ok(sys::ValueResult::Str(s.to_string()))
                } else {
                    Ok(sys::ValueResult::Bytes(v.to_vec()))
                }
            }
            ValueRef::Blob(v) => Ok(sys::ValueResult::Bytes(v.to_vec())),
        }
    }
}

fn row_to_values<'a>(column_count: usize, row: &'a rusqlite::Row) -> Vec<sys::ValueResult> {
    let mut values = Vec::new();
    for index in 0..column_count {
        let raw = row.get_ref(index).unwrap();
        let val = match raw {
            ValueRef::Null => sys::ValueResult::Null,
            ValueRef::Integer(v) => sys::ValueResult::Int64(v),
            ValueRef::Real(v) => sys::ValueResult::Floating64(v),
            ValueRef::Text(v) => {
                if let Ok(v) = std::str::from_utf8(v) {
                    sys::ValueResult::Str(v.to_string())
                } else {
                    sys::ValueResult::Bytes(v.to_vec())
                }
            }
            ValueRef::Blob(v) => sys::ValueResult::Bytes(v.to_vec()),
        };
        values.push(val);
    }
    values
}

impl sys::SqlV1Alpha1 for Sqlite {
    type Connection = Connection;
    type Db = SqliteDb;
    type Driver = SqliteDriver;
    type PreparedStatement = StatementResource;
    type Row = RowResource;
    type Rows = RowsResource;

    fn load_driver(&mut self, driver: &str) -> Result<Self::Driver, SqlError> {
        // FIXME: this doesn't make sense... must be on a wrapper trait impl that covers all enabled drivers
        if driver == "sqlite" {
            Ok(SqliteDriver)
        } else {
            Err(SqlError::new_message(format!(
                "unsupported driver: {}",
                driver
            )))
        }
    }

    fn driver_connect_uri(
        &mut self,
        _self_: &Self::Driver,
        uri: &str,
    ) -> Result<Self::Connection, SqlError> {
        let con = rusqlite::Connection::open(uri)
            .map_err(|e| SqlError::new_message(format!("failed to open connection: {}", e)))?;
        Ok(con.into())
    }

    fn driver_connect_args(
        &mut self,
        _self_: &Self::Driver,
        _args: sys::ValueMapParam<'_>,
    ) -> Result<Self::Connection, SqlError> {
        Err(SqlError::new_message(
            "connect-args() is not supported - use connect-uri() instead",
        ))
    }

    fn driver_open_pool_uri(
        &mut self,
        _self_: &Self::Driver,
        uri: &str,
        options: Option<sys::PoolOptions>,
    ) -> Result<Self::Db, SqlError> {
        let manager = if uri == ":memory:" {
            r2d2_sqlite::SqliteConnectionManager::memory()
        } else {
            r2d2_sqlite::SqliteConnectionManager::file(uri)
        };
        let mut builder = r2d2::Pool::builder();
        if let Some(options) = options {
            if let Some(max) = options.max_connections {
                builder = builder.max_size(max);
            }
            if let Some(idle_timeout) = options.idle_timeout {
                builder =
                    builder.max_lifetime(Some(std::time::Duration::from_secs(idle_timeout as u64)));
            }
        }
        let pool = builder
            .build(manager)
            .map_err(|e| SqlError::new_message(format!("failed to open connection pool: {}", e)))?;

        Ok(SqliteDb { pool })
    }

    fn driver_open_pool_args(
        &mut self,
        _self_: &Self::Driver,
        _args: sys::ValueMapParam<'_>,
        _options: Option<sys::PoolOptions>,
    ) -> Result<Self::Db, SqlError> {
        Err(SqlError::new_message(
            "open-pool-args() is not supported - use open-pool-uri() instead",
        ))
    }

    fn db_connection(&mut self, db: &Self::Db) -> Result<Self::Connection, SqlError> {
        let con = db
            .pool
            .get()
            .map_err(|e| SqlError::new_message(format!("failed to get connection: {}", e)))?;
        Ok(con.into())
    }

    fn db_execute(
        &mut self,
        db: &Self::Db,
        query: &str,
        args: Vec<ValueParam<'_>>,
    ) -> Result<(), SqlError> {
        // TODO: optimize param conversion
        // (the rusqlite::Params trait is not implemented for regular slices of T: ToSql...)
        let params = args
            .iter()
            .map(|v| -> &dyn rusqlite::ToSql { &*v })
            .collect::<Vec<_>>();

        db.pool
            .get()
            .map_err(|e| SqlError::new_message(format!("failed to get connection: {}", e)))?
            .execute(query, params.as_slice())
            .map_err(|e| SqlError::new_message(format!("failed to execute query: {}", e)))?;

        Ok(())
    }

    fn db_query(
        &mut self,
        db: &Self::Db,
        query: &str,
        args: Vec<ValueParam<'_>>,
    ) -> Result<Self::Rows, SqlError> {
        // TODO: optimize param conversion
        // (the rusqlite::Params trait is not implemented for regular slices of T: ToSql...)
        let params = args
            .iter()
            .map(|v| -> &dyn rusqlite::ToSql { &*v })
            .collect::<Vec<_>>();

        let con_raw = db
            .pool
            .get()
            .map_err(|e| SqlError::new_message(format!("failed to get connection: {}", e)))?;

        let statement_raw = con_raw
            .prepare(query)
            .map_err(|err| SqlError::new_message(err.to_string()))?;

        let statement = StatementResource {
            statement: unsafe { UnsafeSharedOptional::new(std::mem::transmute(statement_raw)) },
            _con: SharedOptional::new(SqliteConnection::Pool(con_raw)),
        };

        let mut stmt_borrow = statement.statement.get_mut_unique()?;

        let rows_raw = stmt_borrow
            .query(params.as_slice())
            .map_err(|e| SqlError::new_message(e.to_string()))?;

        let rows = unsafe { UnsafeSharedOptional::new(std::mem::transmute(rows_raw)) };
        std::mem::drop(stmt_borrow);

        let rows = RowsResource { rows, statement };

        Ok(rows)
    }

    fn connection_execute(
        &mut self,
        self_: &Self::Connection,
        query: &str,
        args: Vec<ValueParam<'_>>,
    ) -> Result<(), SqlError> {
        // TODO: optimize param conversion
        // (the rusqlite::Params trait is not implemented for regular slices of T: ToSql...)
        let params = args
            .iter()
            .map(|v| -> &dyn rusqlite::ToSql { &*v })
            .collect::<Vec<_>>();

        // TODO: return change count?

        self_
            .get_mut_unique()?
            .execute(query, params.as_slice())
            .map_err(|e| SqlError::new_message(format!("failed to execute query: {}", e)))?;
        Ok(())
    }

    fn connection_query(
        &mut self,
        conwrap: &Self::Connection,
        query: &str,
        args: Vec<ValueParam<'_>>,
    ) -> Result<Self::Rows, SqlError> {
        // TODO: optimize param conversion
        // (the rusqlite::Params trait is not implemented for regular slices of T: ToSql...)
        let params = args
            .iter()
            .map(|v| -> &dyn rusqlite::ToSql { &*v })
            .collect::<Vec<_>>();

        // FIXME: move to method on RowsData to ensure correct reference.
        // FIXME: make unsafe block minimal
        let rows = unsafe {
            dbg!("getting connection");
            let con = conwrap.get_mut_unique()?;
            dbg!("got connection");

            let raw_stmt = con
                .prepare(query)
                .map_err(|e| SqlError::new_message(format!("failed to prepare query: {}", e)))?;

            let statement = StatementResource {
                statement: UnsafeSharedOptional::new(std::mem::transmute(raw_stmt)),
                _con: conwrap.clone(),
            };

            let mut stmt_borrow = statement.statement.get_mut_unique()?;

            let raw_rows = stmt_borrow
                .query(params.as_slice())
                .map_err(|e| SqlError::new_message(format!("failed to execute query: {}", e)))?;

            eprintln!("got rows");
            let rows = UnsafeSharedOptional::new(std::mem::transmute(raw_rows));

            std::mem::drop(stmt_borrow);
            let rows = RowsResource { rows, statement };

            eprintln!("finished rows");

            rows
        };

        Ok(rows)
    }

    fn connection_prepare(
        &mut self,
        conwrap: &Self::Connection,
        query: &str,
    ) -> Result<Self::PreparedStatement, SqlError> {
        let con = conwrap.get_mut_unique()?;

        let raw_stmt = con
            .prepare(query)
            .map_err(|e| SqlError::new_message(format!("failed to prepare query: {}", e)))?;

        // FIXME: move to method on Connection to ensure correct reference.
        // FIXME: make unsafe block minimal
        let stmt = unsafe {
            let stmt = StatementResource {
                _con: conwrap.clone(),
                statement: UnsafeSharedOptional::new(std::mem::transmute(raw_stmt)),
            };

            stmt
        };

        Ok(stmt)
    }

    fn rows_columns(&mut self, _rows: &Self::Rows) -> Result<Vec<sys::ColumnMeta>, SqlError> {
        Err(SqlError::new_message(
            "sqlite does not support retrieving column types",
        ))
    }

    fn rows_next(&mut self, self_: &Self::Rows) -> Result<Option<Self::Row>, SqlError> {
        let mut rows = self_.rows.get_mut_unique()?;

        let opt = rows
            .next()
            .map_err(|e| SqlError::new_message(format!("failed to get next row: {}", e)))?;

        if let Some(raw_row) = opt {
            // FIXME: move to method on RowsData to ensure correct usage.
            let row = unsafe { UnsafeOptional::new(std::mem::transmute(raw_row)) };
            Ok(Some(RowResource {
                row,
                _rows: self_.rows.clone(),
            }))
        } else {
            Ok(None)
        }
    }

    fn rows_next_values(
        &mut self,
        self_: &Self::Rows,
    ) -> Result<Option<Vec<sys::ValueResult>>, SqlError> {
        let mut inner = self_.rows.get_mut_unique()?;

        let column_count = unsafe { self_.statement.statement.get_shared()?.column_count() };

        let opt = inner
            .next()
            .map_err(|e| SqlError::new_message(format!("failed to get next row: {}", e)))?;

        if let Some(raw_row) = opt {
            let values = row_to_values(column_count, raw_row);

            Ok(Some(values))
        } else {
            Ok(None)
        }
    }

    fn rows_values(&mut self, self_: &Self::Rows) -> Result<Vec<Vec<sys::ValueResult>>, SqlError> {
        let mut rows = self_.rows.get_mut_unique()?;

        let column_count = unsafe { self_.statement.statement.get_shared()?.column_count() };

        let mut row_values = Vec::new();
        loop {
            let row = match rows.next() {
                Ok(Some(r)) => r,
                Ok(None) => {
                    break;
                }
                Err(err) => {
                    return Err(SqlError::new_message(format!(
                        "failed to get next row: {}",
                        err
                    )));
                }
            };

            let values = row_to_values(column_count, row);
            row_values.push(values);
        }

        Ok(row_values)
    }

    fn rows_close(&mut self, self_: &Self::Rows) -> Result<(), SqlError> {
        self_.rows.consume()?;
        Ok(())
    }

    fn row_get_bool(&mut self, self_: &Self::Row, index: u32) -> Result<bool, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_int8(&mut self, self_: &Self::Row, index: u32) -> Result<i8, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_int16(&mut self, self_: &Self::Row, index: u32) -> Result<i16, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_int32(&mut self, self_: &Self::Row, index: u32) -> Result<i32, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_int64(&mut self, self_: &Self::Row, index: u64) -> Result<i64, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_uint8(&mut self, self_: &Self::Row, index: u32) -> Result<u8, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_uint16(&mut self, self_: &Self::Row, index: u32) -> Result<u16, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_uint32(&mut self, self_: &Self::Row, index: u32) -> Result<u32, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_uint64(&mut self, self_: &Self::Row, index: u64) -> Result<u64, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_float32(&mut self, self_: &Self::Row, index: u32) -> Result<f32, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_float64(&mut self, self_: &Self::Row, index: u32) -> Result<f64, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_string(&mut self, self_: &Self::Row, index: u32) -> Result<String, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_bytes(&mut self, self_: &Self::Row, index: u32) -> Result<Vec<u8>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_raw(&mut self, self_: &Self::Row, index: u32) -> Result<Vec<u8>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_bool(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<bool>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_int8(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<i8>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_int16(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<i16>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_int32(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<i32>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_int64(
        &mut self,
        self_: &Self::Row,
        index: u64,
    ) -> Result<Option<i64>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_uint8(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<u8>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_uint16(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<u16>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_uint32(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<u32>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_uint64(
        &mut self,
        self_: &Self::Row,
        index: u64,
    ) -> Result<Option<u64>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_float32(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<f32>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_float64(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<f64>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_string(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<String>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_bytes(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<Vec<u8>>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_optional_raw(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<Option<Vec<u8>>, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn row_get_value(
        &mut self,
        self_: &Self::Row,
        index: u32,
    ) -> Result<sys::ValueResult, SqlError> {
        let row = self_.row.get_mut()?;
        row.get::<usize, _>(index.try_into().unwrap())
            .map_err(|e| SqlError::new_message(format!("{}", e)))
    }

    fn prepared_statement_execute(
        &mut self,
        self_: &Self::PreparedStatement,
        args: Vec<ValueParam<'_>>,
    ) -> Result<(), SqlError> {
        let mut s = self_.statement.get_mut_unique()?;

        // TODO: optimize param conversion
        // (the rusqlite::Params trait is not implemented for regular slices of T: ToSql...)
        let params = args
            .iter()
            .map(|v| -> &dyn rusqlite::ToSql { &*v })
            .collect::<Vec<_>>();

        // TODO: return change count?

        s.execute(params.as_slice())
            .map_err(|e| SqlError::new_message(format!("{}", e)))?;
        Ok(())
    }

    fn prepared_statement_query(
        &mut self,
        self_: &Self::PreparedStatement,
        args: Vec<ValueParam<'_>>,
    ) -> Result<Self::Rows, SqlError> {
        // TODO: optimize param conversion
        // (the rusqlite::Params trait is not implemented for regular slices of T: ToSql...)
        let params = args
            .iter()
            .map(|v| -> &dyn rusqlite::ToSql { &*v })
            .collect::<Vec<_>>();

        let mut stmt_borrow = self_.statement.get_mut_unique()?;

        let raw_rows = stmt_borrow
            .query(params.as_slice())
            .map_err(|e| SqlError::new_message(e.to_string()))?;

        // FIXME: move to method on RowsData to ensure correct reference.
        let rows = unsafe { UnsafeSharedOptional::new(std::mem::transmute(raw_rows)) };

        let rows = RowsResource {
            rows,
            statement: self_.clone(),
        };

        Ok(rows)
    }

    fn drop_connection(&mut self, state: Self::Connection) {
        state.consume().unwrap();
        drop(state);
    }

    fn drop_db(&mut self, state: Self::Db) {
        // TODO: should these fail if the resource is still shared?
        drop(state);
    }

    fn drop_driver(&mut self, state: Self::Driver) {
        drop(state);
    }

    fn drop_prepared_statement(&mut self, state: Self::PreparedStatement) {
        state.statement.consume().unwrap();
        drop(state);
    }

    fn drop_row(&mut self, state: Self::Row) {
        drop(state);
    }

    fn drop_rows(&mut self, state: Self::Rows) {
        state.rows.consume().unwrap();
        drop(state);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn tests_path() -> PathBuf {
        std::env::var("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .unwrap()
            .parent()
            .unwrap()
            .join("target/wasm32-wasi/debug/examples")
    }

    struct StateWithWasi {
        sqlite: State,
        wasi: wasmtime_wasi::WasiCtx,
    }

    #[test]
    fn test_wasmtime_sqlite() {
        let engine = wasmtime::Engine::default();
        let path = tests_path().join("sqlite.wasm");

        let module = wasmtime::Module::from_file(&engine, &dbg!(path)).unwrap();

        let mut linker = wasmtime::Linker::<StateWithWasi>::new(&engine);

        wasmtime_wasi::sync::add_to_linker(&mut linker, |s| &mut s.wasi).unwrap();

        sys::add_to_linker(&mut linker, |state| {
            (&mut state.sqlite.data, &mut state.sqlite.tables)
        })
        .unwrap();

        let state = State {
            data: Sqlite {},
            tables: Default::default(),
        };

        let wasi = wasmtime_wasi::sync::WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build();

        let full = StateWithWasi {
            sqlite: state,
            wasi,
        };

        let mut store = wasmtime::Store::new(&engine, full);

        let instance = linker.instantiate(&mut store, &module).unwrap();

        let main = instance.get_func(&mut store, "main").unwrap();
        let mut out: [wasmtime::Val; 1] = [0i32.into()];
        main.call(&mut store, &[0i32.into(), 0i32.into()], &mut out)
            .unwrap();
    }
}
