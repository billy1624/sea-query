use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use sea_query::{ColumnDef, Expr, Func, Iden, Order, PostgresQueryBuilder, Query, Table};
use sqlx::{PgPool, Row};

sea_query::sea_query_driver_postgres!();
use sea_query_driver_postgres::{bind_query, bind_query_as};
use serde_json::{json, Value as Json};
use uuid::Uuid;

#[async_std::main]
async fn main() {
    let connection = PgPool::connect("postgres://sea:sea@127.0.0.1/query")
        .await
        .unwrap();
    let mut pool = connection.try_acquire().unwrap();

    let mut stmt = Query::update();

    stmt.table(Character::Table);

    let val = Some(vec![(Character::FontSize, 24i32.into())]);
    if let Some(v) = val {
        stmt.values(v);
    }
    stmt.and_where(Expr::col(Character::Id).eq(1));

    let (sql, values) = stmt.build(PostgresQueryBuilder);

    let result = bind_query(sqlx::query(&sql), &values)
        .execute(&mut pool)
        .await;
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    Uuid,
    Character,
    FontSize,
    Meta,
    Decimal,
    BigDecimal,
    Created,
}
