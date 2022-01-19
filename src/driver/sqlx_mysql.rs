#[macro_export]
macro_rules! bind_params_sqlx_mysql {
    ( $query:expr, $params:expr ) => {{
        let mut query = $query;
        for value in $params.iter() {
            macro_rules! bind {
                ( $v: expr, $ty: ty ) => {
                    match $v {
                        Some(v) => query.bind((*v as $ty)),
                        None => query.bind(None::<$ty>),
                    }
                };
            }
            macro_rules! bind_box {
                ( $v: expr, $ty: ty ) => {
                    match $v {
                        Some(v) => query.bind(v.as_ref()),
                        None => query.bind(None::<$ty>),
                    }
                };
            }
            query = match value {
                Value::Bool(v) => bind!(v, bool),
                Value::TinyInt(v) => bind!(v, i8),
                Value::SmallInt(v) => bind!(v, i16),
                Value::Int(v) => bind!(v, i32),
                Value::BigInt(v) => bind!(v, i64),
                Value::TinyUnsigned(v) => bind!(v, u8),
                Value::SmallUnsigned(v) => bind!(v, u16),
                Value::Unsigned(v) => bind!(v, u32),
                Value::BigUnsigned(v) => bind!(v, u64),
                Value::Float(v) => bind!(v, f32),
                Value::Double(v) => bind!(v, f64),
                Value::String(v) => bind_box!(v, String),
                Value::Bytes(v) => bind_box!(v, Vec<u8>),
                _ => {
                    if value.is_json() {
                        query.bind(value.as_ref_json())
                    } else if value.is_date() {
                        query.bind(value.as_ref_date())
                    } else if value.is_time() {
                        query.bind(value.as_ref_time())
                    } else if value.is_date_time() {
                        query.bind(value.as_ref_date_time())
                    } else if value.is_date_time_utc() {
                        query.bind(value.as_ref_date_time_utc())
                    } else if value.is_date_time_with_time_zone() {
                        query.bind(value.as_naive_utc_in_string())
                    } else if value.is_decimal() {
                        query.bind(value.as_ref_decimal())
                    } else if value.is_big_decimal() {
                        query.bind(value.as_ref_big_decimal())
                    } else if value.is_uuid() {
                        query.bind(value.as_ref_uuid())
                    } else {
                        unimplemented!();
                    }
                }
            };
        }
        query
    }};
}

#[macro_export]
macro_rules! sea_query_driver_mysql {
    () => {
        mod sea_query_driver_mysql {
            use sqlx::{mysql::MySqlArguments, query::Query, query::QueryAs, MySql};
            use $crate::{Value, Values};

            type SqlxQuery<'a> = Query<'a, MySql, MySqlArguments>;
            type SqlxQueryAs<'a, T> = QueryAs<'a, MySql, T, MySqlArguments>;

            pub fn bind_query<'a>(query: SqlxQuery<'a>, params: &'a Values) -> SqlxQuery<'a> {
                $crate::bind_params_sqlx_mysql!(query, params.0)
            }

            pub fn bind_query_as<'a, T>(
                query: SqlxQueryAs<'a, T>,
                params: &'a Values,
            ) -> SqlxQueryAs<'a, T> {
                $crate::bind_params_sqlx_mysql!(query, params.0)
            }
        }
    };
}
