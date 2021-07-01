use crate::sql::restrict;
use crate::sql::Env;
use crate::sql::Expr;

use crate::sql::WhereCond;
use crate::value::PqlValue;

#[derive(Debug, Default, Clone)]
pub struct Filter(pub Option<Box<WhereCond>>);

impl Filter {
    pub fn execute(self, data: PqlValue, env: &Env) -> PqlValue {
        match &self.0 {
            None => data,
            Some(box WhereCond::Eq { expr, right }) => match expr {
                Expr::Path(selector) => {
                    let selector = selector.expand_fullpath2(&env);
                    let cond = WhereCond::Eq {
                        expr: expr.to_owned(),
                        right: right.to_owned(),
                    };
                    restrict(Some(data.to_owned()), &selector, &Some(cond))
                        .expect("restricted value")
                }
                _ => {
                    todo!();
                }
            },
            Some(box WhereCond::Like { expr, right }) => match expr {
                Expr::Path(selector) => {
                    let path = selector.expand_fullpath2(&env);
                    let cond = WhereCond::Like {
                        expr: expr.to_owned(),
                        right: right.to_owned(),
                    };
                    restrict(Some(data.to_owned()), &path, &Some(cond)).expect("restricted value")
                }
                _ => {
                    todo!();
                }
            },
            _ => {
                dbg!(&self);
                todo!()
            }
        }
    }
}
