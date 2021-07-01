mod drain;
mod filter;
mod project;

use crate::sql::Env;
use crate::value::PqlValue;
pub use drain::Drain;
pub use filter::Filter;
pub use project::Projection;

#[derive(Debug, Default)]
pub struct LogicalPlan {
    pub drains: Vec<Drain>,
    pub filter: Filter,
    pub project: Projection,
}

impl LogicalPlan {
    fn excute(self, data: PqlValue, env: &mut Env) -> PqlValue {
        for drain in self.drains {
            drain.excute(env);
        }

        let data = self.filter.execute(data, &env);

        let list = self.project.execute(data);

        PqlValue::Array(list)
    }
}
