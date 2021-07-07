pub use crate::planner::LogicalPlan;
pub use crate::sql::clause::Limit;
pub use crate::sql::clause::OrderBy;
use crate::sql::Env;
use crate::sql::Sql;
use crate::value::PqlValue;

pub fn evaluate<'a>(sql: Sql, data: PqlValue) -> PqlValue {
    let mut env = Env::default();
    let plan = LogicalPlan::from(sql);
    let result = plan.execute(data, &mut env);
    result
}
