use crate::sql::bindings::Bindings;
use crate::sql::field::Field;
use crate::sql::Expr;
use crate::value::PqlValue;

#[derive(Debug, Clone, PartialEq)]
pub enum WhereCond {
    Eq { expr: Expr, right: String },
    Like { expr: Expr, right: String },
}

impl Default for WhereCond {
    fn default() -> Self {
        Self::Eq {
            expr: Expr::default(),
            right: String::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DWhereCond {
    Eq { field: Field, right: String },
    Like { field: Field, right: String },
}

impl Default for DWhereCond {
    fn default() -> Self {
        Self::Eq {
            field: Field::default(),
            right: String::default(),
        }
    }
}

impl DWhereCond {
    pub fn eval(
        &self,
        left: &PqlValue,
        bindings: &Bindings,
        bindings_for_select: &Bindings,
    ) -> bool {
        todo!();
        // match self {
        //     Self::Eq { field, right } => {
        //         let where_arg_path = field.path.expand_fullpath(&bindings);
        //         let access_path = bindings_for_select
        //             .to_alias(&where_arg_path)
        //             .unwrap_or(where_arg_path.to_owned());
        //         if let Some(value) = left.clone().select_by_path(&access_path) {
        //             value == PqlValue::Str(right.to_owned())
        //         } else {
        //             false
        //         }
        //     }
        //     Self::Like { field, right } => {
        //         let pattern = match (right.starts_with("%"), right.ends_with("%")) {
        //             (true, true) => {
        //                 format!("{}", right.trim_start_matches("%").trim_end_matches("%"))
        //             }
        //             (true, false) => format!("{}$", right.trim_start_matches("%")),
        //             (false, true) => format!("^{}", right.trim_end_matches("%")),
        //             (false, false) => format!("^{}$", right),
        //         };
        //         let re = regex::Regex::new(&pattern).unwrap();

        //         let where_arg_path = field.path.full(&bindings);
        //         let access_path = bindings_for_select
        //             .to_alias(&where_arg_path)
        //             .unwrap_or(where_arg_path.to_owned());
        //         match left.select_by_path(&access_path) {
        //             Some(PqlValue::Str(s)) if re.is_match(&s) => true,
        //             _ => false,
        //         }
        //     }
        // }
    }
}
