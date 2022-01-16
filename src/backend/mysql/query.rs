use super::*;

impl QueryBuilder for MysqlQueryBuilder {
    fn prepare_returning(
        &self,
        _returning: &[SelectExpr],
        _sql: &mut SqlWriter,
        _collector: &mut dyn FnMut(Value),
    ) {
    }

    fn prepare_order_expr(
        &self,
        order_expr: &OrderExpr,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        match order_expr.nulls {
            None => (),
            Some(NullOrdering::Last) => {
                self.prepare_simple_expr(&order_expr.expr, sql, collector);
                write!(sql, " IS NULL ASC, ").unwrap()
            }
            Some(NullOrdering::First) => {
                self.prepare_simple_expr(&order_expr.expr, sql, collector);
                write!(sql, " IS NULL DESC, ").unwrap()
            }
        }
        self.prepare_simple_expr(&order_expr.expr, sql, collector);
        write!(sql, " ").unwrap();
        self.prepare_order(&order_expr.order, sql, collector);
    }

    fn prepare_query_statement(
        &self,
        query: &dyn QueryStatementBuilder,
        sql: &mut SqlWriter,
        collector: &mut dyn FnMut(Value),
    ) {
        write!(sql, "{}", query.build_collect_any(self, collector)).unwrap();
    }

    fn prepare_with_clause_recursive_options(
        &self,
        _: &WithClause,
        _: &mut SqlWriter,
        _: &mut dyn FnMut(Value),
    ) {
        // MySQL doesn't support sql recursive with query 'SEARCH' and 'CYCLE' options.
    }

    fn prepare_with_query_clause_materialization(
        &self,
        _: &CommonTableExpression,
        _: &mut SqlWriter,
    ) {
        // MySQL doesn't support declaring materialization in SQL for with query.
    }
}
