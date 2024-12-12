use std::sync::RwLock;

use diesel::connection::{
    AnsiTransactionManager, Connection, ConnectionSealed, DefaultLoadingMode, LoadConnection,
    SimpleConnection, TransactionManager,
};
use diesel::pg::{Pg, PgQueryBuilder, PgRowByRowLoadingMode};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::RunQueryDsl;
use diesel::{PgConnection, QueryResult};

pub struct MyPgConnection {
    namespace: String,
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    // is_schema_set: RwLock<bool>,
}

pub struct MyTransactioManager;

impl TransactionManager<MyPgConnection> for MyTransactioManager {
    type TransactionStateData =
        <AnsiTransactionManager as TransactionManager<PgConnection>>::TransactionStateData;

    fn begin_transaction(conn: &mut MyPgConnection) -> diesel::prelude::QueryResult<()> {
        AnsiTransactionManager::begin_transaction(&mut *conn.conn)?;
        let 
        let result = diesel::sql_query("SELECT set_config('search_path', $1, true)")
            .bind::<diesel::sql_types::Text, _>(&conn.namespace)
            .execute(&mut *conn.conn)?;
        let result = diesel::sql_query("SELECT set_config('search_path', $1, true)")
            .bind::<diesel::sql_types::Text, _>(&conn.namespace)
            .execute(&mut *conn.conn)?;
        log::info!("{:?}", result);
        Ok(())
    }

    fn rollback_transaction(conn: &mut MyPgConnection) -> diesel::prelude::QueryResult<()> {
        AnsiTransactionManager::rollback_transaction(&mut *conn.conn)
    }

    fn commit_transaction(conn: &mut MyPgConnection) -> diesel::prelude::QueryResult<()> {
        AnsiTransactionManager::commit_transaction(&mut *conn.conn)
    }

    fn transaction_manager_status_mut(
        conn: &mut MyPgConnection,
    ) -> &mut diesel::connection::TransactionManagerStatus {
        AnsiTransactionManager::transaction_manager_status_mut(&mut *conn.conn)
    }
}

// impl Drop for MyPgConnection {
//     fn drop(&mut self) {
//         let _ = diesel::sql_query("SELECT set_config('search_path', 'public', false)")
//             .execute(&mut self.conn);
//     }
// }

impl ConnectionSealed for MyPgConnection {}

impl SimpleConnection for MyPgConnection {
    fn batch_execute(&mut self, query: &str) -> diesel::prelude::QueryResult<()> {
        self.conn.batch_execute(query)
    }
}

impl Connection for MyPgConnection {
    type Backend = Pg;
    type TransactionManager = MyTransactioManager;

    fn establish(database_url: &str) -> diesel::prelude::ConnectionResult<Self> {
        let conn = PooledConnection::establish(database_url)?;
        Ok(MyPgConnection {
            // NOTE: this function will never be used, so namespace here doesn't matter
            namespace: String::new(),
            conn,
            // is_schema_set: false.into(),
        })
    }

    fn execute_returning_count<T>(&mut self, source: &T) -> diesel::prelude::QueryResult<usize>
    where
        T: diesel::query_builder::QueryFragment<Self::Backend> + diesel::query_builder::QueryId,
    {
        // self.set_namespace()?;
        // self.conn.execute_returning_count(source)
        log::info!("{:?}", source.to_sql(&mut PgQueryBuilder::default(), &Pg));
        self.transaction::<usize, diesel::result::Error, _>(|conn| {
            (*conn.conn).execute_returning_count(source)
        })
    }

    fn transaction_state(&mut self,) ->  &mut<Self::TransactionManager as diesel::connection::TransactionManager<Self>>::TransactionStateData{
        self.conn.transaction_state()
    }
}

impl LoadConnection<DefaultLoadingMode> for MyPgConnection {
    type Cursor<'conn, 'query> =
        <PgConnection as LoadConnection<DefaultLoadingMode>>::Cursor<'conn, 'query>;
    type Row<'conn, 'query> =
        <PgConnection as LoadConnection<DefaultLoadingMode>>::Row<'conn, 'query>;

    fn load<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> diesel::prelude::QueryResult<Self::Cursor<'conn, 'query>>
    where
        T: diesel::query_builder::Query
            + diesel::query_builder::QueryFragment<Self::Backend>
            + diesel::query_builder::QueryId
            + 'query,
        Self::Backend: diesel::expression::QueryMetadata<T::SqlType>,
    {
        // self.set_namespace()?;
        // <PgConnection as LoadConnection<DefaultLoadingMode>>::load::<T>(
        //     &mut self.conn,
        //     source,
        // )
        self.transaction::<Self::Cursor<'conn, 'query>, diesel::result::Error, _>(|conn| {
            log::info!("{:?}", source.to_sql(&mut PgQueryBuilder::default(), &Pg));
            <PgConnection as LoadConnection<DefaultLoadingMode>>::load::<T>(&mut *conn.conn, source)
        })
    }
}

// impl LoadConnection<PgRowByRowLoadingMode> for MyPgConnection {
//     type Cursor<'conn, 'query> =
//         <PgConnection as LoadConnection<PgRowByRowLoadingMode>>::Cursor<'conn, 'query>;
//     type Row<'conn, 'query> =
//         <PgConnection as LoadConnection<PgRowByRowLoadingMode>>::Row<'conn, 'query>;
//
//     fn load<'conn, 'query, T>(
//         &'conn mut self,
//         source: T,
//     ) -> diesel::prelude::QueryResult<Self::Cursor<'conn, 'query>>
//     where
//         T: diesel::query_builder::Query
//             + diesel::query_builder::QueryFragment<Self::Backend>
//             + diesel::query_builder::QueryId
//             + 'query,
//         Self::Backend: diesel::expression::QueryMetadata<T::SqlType>,
//     {
//         self.set_namespace()?;
//         <PgConnection as LoadConnection<PgRowByRowLoadingMode>>::load::<T>(
//             &mut self.conn,
//             source,
//         )
//     }
// }

impl MyPgConnection {
    pub fn new(namespace: String, conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        MyPgConnection {
            namespace,
            conn,
            // is_schema_set: false.into(),
        }
    }

    // fn set_namespace(&mut self) -> QueryResult<()> {
    //     let mut is_schema_set = self.is_schema_set.write().unwrap();
    //     if !*is_schema_set {
    //         log::info!("\n\n Setting the schema \n\n");
    //         let _ = diesel::sql_query("SELECT set_config('search_path', $1, false)")
    //             .bind::<diesel::sql_types::Text, _>(&self.namespace)
    //             .execute(&mut self.conn)?;
    //         *is_schema_set = true;
    //     }
    //     Ok(())
    // }
}
