use self::RenameRule::*;
use proc_macro_error::abort;
use quote::ToTokens;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum RenameRule {
    UpperCase,
    PascalCase,
    CamelCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

static RENAME_RULES: &[(&str, RenameRule)] = &[
    ("UPPERCASE", UpperCase),
    ("PascalCase", PascalCase),
    ("camelCase", CamelCase),
    ("SCREAMING_SNAKE_CASE", ScreamingSnakeCase),
    ("kebab-case", KebabCase),
    ("SCREAMING-KEBAB-CASE", ScreamingKebabCase),
];

impl RenameRule {
    pub fn new(expr: &syn::Expr) -> Self {
        let expr_str = expr.to_token_stream().to_string().replace(' ', "");
        match RENAME_RULES
            .iter()
            .find(|&(name, _)| *name == expr_str.as_str())
        {
            Some((_, rule)) => rule.clone(),
            None => {
                abort! {
                    expr, "Invalid `rename_all` attribute value";
                    note = "\"{}\" is not supported as `rename_all` value", expr_str;
                    help = "Use one of the \"UPPERCASE\", \"PascalCase\", \"camelCase\", \"SCREAMING_SNAKE_CASE\", \"kebab-case\" and \"SCREAMING-KEBAB-CASE\""
                }
            }
        }
    }

    pub fn apply(&self, field: &str) -> String {
        match self {
            UpperCase => field.to_ascii_uppercase(),
            PascalCase => {
                let mut pascal = String::new();
                let mut capitalize = true;
                for ch in field.chars() {
                    if ch == '_' {
                        capitalize = true;
                    } else if capitalize {
                        pascal.push(ch.to_ascii_uppercase());
                        capitalize = false;
                    } else {
                        pascal.push(ch);
                    }
                }
                pascal
            }
            CamelCase => {
                let pascal = PascalCase.apply(field);
                pascal[..1].to_ascii_lowercase() + &pascal[1..]
            }
            ScreamingSnakeCase => field.to_ascii_uppercase(),
            KebabCase => field.replace('_', "-"),
            ScreamingKebabCase => ScreamingSnakeCase.apply(field).replace('_', "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_fields() {
        for &(original, upper, pascal, camel, screaming, kebab, screaming_kebab) in &[
            (
                "outcome", "OUTCOME", "Outcome", "outcome", "OUTCOME", "outcome", "OUTCOME",
            ),
            (
                "very_tasty",
                "VERY_TASTY",
                "VeryTasty",
                "veryTasty",
                "VERY_TASTY",
                "very-tasty",
                "VERY-TASTY",
            ),
            ("a", "A", "A", "a", "A", "a", "A"),
            ("z42", "Z42", "Z42", "z42", "Z42", "z42", "Z42"),
        ] {
            assert_eq!(UpperCase.apply(original), upper);
            assert_eq!(PascalCase.apply(original), pascal);
            assert_eq!(CamelCase.apply(original), camel);
            assert_eq!(ScreamingSnakeCase.apply(original), screaming);
            assert_eq!(KebabCase.apply(original), kebab);
            assert_eq!(ScreamingKebabCase.apply(original), screaming_kebab);
        }
    }
}
