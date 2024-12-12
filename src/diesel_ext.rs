// use diesel::backend::Backend;
// use diesel::expression::QueryMetadata;
// use diesel::pg::Pg;
// use diesel::query_builder::{AstPass, Query, QueryFragment, QueryId};
// use diesel::{PgConnection, QueryResult, RunQueryDsl, Table};
// 
// #[derive(Debug, Clone, QueryId)]
// pub struct SchemaPrefix<T> {
//     inner: T,
//     schema: String,
// }
// 
// impl<T> QueryFragment<Pg> for SchemaPrefix<T>
// where
//     T: QueryFragment<Pg> + Table,
// {
//     fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
//         out.push_sql(&self.schema);
//         out.push_sql(".");
//         self.inner.walk_ast(out.reborrow())?;
//         Ok(())
//     }
// }
// 
// impl Backend for SchemaPrefix<T> {
// }
// 
// impl<T> Query for SchemaPrefix<T>
// where
//     T: Table + Query
// {
//     type SqlType = T::SqlType;
// }
// 
// impl<T> RunQueryDsl<PgConnection> for SchemaPrefix<T> {}
// 
// impl<T> QueryMetadata<Pg> for SchemaPrefix<T>
// where
//     T: QueryMetadata<Pg> + Table,
// {
//     fn row_metadata(lookup: &mut Pg::MetadataLookup) -> QueryResult<()> {
//         T::row_metadata(lookup);
//         Ok(())
//     }
// }
// 
// pub trait WithSchema: Sized {
//     fn with_schema(self, schema: impl Into<String>) -> SchemaPrefix<Self>;
// }
// 
// impl<T: Table> WithSchema for T {
//     fn with_schema(self, schema: impl Into<String>) -> SchemaPrefix<Self> {
//         SchemaPrefix {
//             inner: self,
//             schema: schema.into(),
//         }
//     }
// }
