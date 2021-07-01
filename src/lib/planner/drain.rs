use crate::sql::Env;

use crate::sql::Field;

#[derive(Debug, Default, Clone)]
pub struct Drain(pub Vec<Field>);

impl Drain {
    pub fn excute(self, env: &mut Env) {
        for field in self.0 {
            if let Some(alias) = field.alias {
                env.insert(&alias, &field.expr);
            }
        }
    }
}
