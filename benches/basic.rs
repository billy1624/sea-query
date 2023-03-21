use criterion::{criterion_group, criterion_main, Criterion};
use sea_query::{tests_cfg::*, *};

fn vanilla() -> String {
    format!(
        "SELECT `{}` from `{}` where `character` = {}",
        "character",
        "character".to_owned(),
        123
    )
}

fn select() -> SelectStatement {
    Query::select()
        .column(Char::Character)
        .from(Char::Table)
        .and_where(Expr::col(Char::Character).eq(123))
        .to_owned()
}

fn select_and_build() {
    select().build(MysqlQueryBuilder);
}

fn select_and_to_string() {
    select().to_string(MysqlQueryBuilder);
}

const NAMES: [i32; 1_000_000] = [0; 1_000_000];

fn mut_iter() {
    let mut sql = String::new();
    let mut names_iter = NAMES.iter();

    if let Some(name) = names_iter.next() {
        write!(sql, "{}", name).unwrap();
    }

    for name in names_iter {
        write!(sql, ", ").unwrap();
        write!(sql, "{}", name).unwrap();
    }
}

fn fold() {
    let mut sql = String::new();

    NAMES.iter().fold(true, |first, name| {
        if !first {
            write!(sql, ", ").unwrap();
        }
        write!(sql, "{}", name).unwrap();
        false
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("vanilla", |b| b.iter(vanilla));
    c.bench_function("select", |b| b.iter(select));
    c.bench_function("select_and_build", |b| b.iter(select_and_build));
    c.bench_function("select_and_to_string", |b| b.iter(select_and_to_string));
    c.bench_function("mut_iter", |b| b.iter(mut_iter));
    c.bench_function("fold", |b| b.iter(fold));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
