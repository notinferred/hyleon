//! Questions as a form generated from a JSON Schema. The schema is the flat form MCP elicitation
//! uses (docs/formats/mcp.md), so the app and an agent's client ask the same questions.

use dioxus::prelude::*;
use serde::Deserialize;
use serde_json::{Map, Value};

const TRIAL_SCHEMA: &str = include_str!("../assets/trial/questions.schema.json");

#[derive(Debug, Clone, PartialEq)]
pub struct Form {
    pub title: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub required: bool,
    pub kind: Kind,
}

/// The field kinds a flat form allows. `catalog` questions are string enums too, so they're
/// choices here.
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Choice(Vec<String>),
    Bool,
    Number {
        whole: bool,
        min: Option<f64>,
        max: Option<f64>,
    },
    Text,
}

#[derive(Deserialize)]
struct RawForm {
    #[serde(rename = "type")]
    kind: String,
    title: Option<String>,
    #[serde(default)]
    properties: Map<String, Value>,
    #[serde(default)]
    required: Vec<String>,
}

#[derive(Deserialize)]
struct RawField {
    #[serde(rename = "type")]
    kind: String,
    title: Option<String>,
    description: Option<String>,
    #[serde(rename = "enum")]
    choices: Option<Vec<String>>,
    minimum: Option<f64>,
    maximum: Option<f64>,
}

/// Parses a schema into a form, or says in plain words why it isn't a flat form.
pub fn parse(schema: &str) -> Result<Form, String> {
    let raw: RawForm =
        serde_json::from_str(schema).map_err(|e| format!("The schema isn't valid: {e}"))?;
    if raw.kind != "object" {
        return Err(format!("A form is an object, not \"{}\".", raw.kind));
    }
    let fields = raw
        .properties
        .into_iter()
        .map(|(name, value)| {
            let field: RawField = serde_json::from_value(value)
                .map_err(|e| format!("\"{name}\" isn't a field a form can show: {e}"))?;
            let kind = match (field.kind.as_str(), field.choices) {
                ("string", Some(choices)) => Kind::Choice(choices),
                ("string", None) => Kind::Text,
                ("boolean", _) => Kind::Bool,
                (number @ ("number" | "integer"), _) => Kind::Number {
                    whole: number == "integer",
                    min: field.minimum,
                    max: field.maximum,
                },
                ("object" | "array", _) => {
                    return Err(format!(
                        "\"{name}\" holds a nested {}, but forms are flat.",
                        field.kind
                    ));
                }
                (other, _) => return Err(format!("\"{name}\" has an unknown type, \"{other}\".")),
            };
            Ok(Field {
                title: field.title.unwrap_or_else(|| name.clone()),
                description: field.description,
                required: raw.required.contains(&name),
                name,
                kind,
            })
        })
        .collect::<Result<_, _>>()?;
    Ok(Form {
        title: raw.title.unwrap_or_default(),
        fields,
    })
}

/// What's wrong with an answer, if anything. Answers arrive as the text of their control.
pub fn problem(field: &Field, answer: &str) -> Option<String> {
    let answer = answer.trim();
    if answer.is_empty() {
        return field.required.then(|| "An answer is needed.".to_string());
    }
    let Kind::Number { whole, min, max } = field.kind else {
        return None;
    };
    let Ok(number) = answer.parse::<f64>() else {
        return Some("Enter a number.".to_string());
    };
    if whole && number.fract() != 0.0 {
        return Some("Enter a whole number.".to_string());
    }
    match (min, max) {
        (Some(min), Some(max)) if !(min..=max).contains(&number) => {
            Some(format!("Enter a number from {min} to {max}."))
        }
        (Some(min), None) if number < min => Some(format!("Enter {min} or more.")),
        (None, Some(max)) if number > max => Some(format!("Enter {max} or less.")),
        _ => None,
    }
}

#[component]
pub fn Questions() -> Element {
    let form = use_hook(|| parse(TRIAL_SCHEMA));
    let form = match form {
        Ok(form) => form,
        Err(why) => return rsx! { p { role: "alert", "{why}" } },
    };
    rsx! { QuestionForm { form } }
}

#[component]
fn QuestionForm(form: Form) -> Element {
    let count = form.fields.len();
    let mut answers = use_signal(|| vec![String::new(); count]);
    let mut problems = use_signal(|| vec![None::<String>; count]);
    let mut summary = use_signal(String::new);
    let fields = form.fields.clone();

    rsx! {
        form {
            class: "questions",
            aria_labelledby: "questions-title",
            novalidate: true,
            onsubmit: move |evt| {
                evt.prevent_default();
                let found: Vec<_> = fields
                    .iter()
                    .zip(answers.read().iter())
                    .map(|(field, answer)| problem(field, answer))
                    .collect();
                let open = found.iter().flatten().count();
                summary.set(match open {
                    0 => format!("{count} answers recorded."),
                    1 => "1 answer needs attention.".to_string(),
                    n => format!("{n} answers need attention."),
                });
                problems.set(found);
            },
            h2 { id: "questions-title", "{form.title}" }
            for (i , field) in form.fields.iter().enumerate() {
                FieldControl {
                    field: field.clone(),
                    problem: problems.read()[i].clone(),
                    on_answer: move |value: String| answers.write()[i] = value,
                }
            }
            button { r#type: "submit", "Record answers" }
            p { role: "status", "{summary}" }
        }
    }
}

#[component]
fn FieldControl(field: Field, problem: Option<String>, on_answer: EventHandler<String>) -> Element {
    let id = format!("q-{}", field.name);
    let help = format!("{id}-help");
    let error = format!("{id}-error");
    let described = match (&field.description, &problem) {
        (Some(_), Some(_)) => format!("{help} {error}"),
        (Some(_), None) => help.clone(),
        (None, Some(_)) => error.clone(),
        (None, None) => String::new(),
    };
    let invalid = if problem.is_some() { "true" } else { "false" };
    let label = if field.required {
        format!("{} (required)", field.title)
    } else {
        field.title.clone()
    };

    let control = match &field.kind {
        Kind::Choice(choices) => rsx! {
            select {
                id: "{id}",
                required: field.required,
                aria_describedby: "{described}",
                aria_invalid: invalid,
                onchange: move |evt| on_answer.call(evt.value()),
                option { value: "", "Choose…" }
                for choice in choices {
                    option { value: "{choice}", "{choice}" }
                }
            }
        },
        Kind::Bool => rsx! {
            input {
                id: "{id}",
                r#type: "checkbox",
                aria_describedby: "{described}",
                onchange: move |evt| on_answer.call(evt.checked().to_string()),
            }
        },
        Kind::Number { whole, min, max } => rsx! {
            input {
                id: "{id}",
                r#type: "number",
                inputmode: if *whole { "numeric" } else { "decimal" },
                step: if *whole { "1" } else { "any" },
                min: min.map(|m| m.to_string()),
                max: max.map(|m| m.to_string()),
                required: field.required,
                aria_describedby: "{described}",
                aria_invalid: invalid,
                oninput: move |evt| on_answer.call(evt.value()),
            }
        },
        Kind::Text => rsx! {
            input {
                id: "{id}",
                r#type: "text",
                required: field.required,
                aria_describedby: "{described}",
                aria_invalid: invalid,
                oninput: move |evt| on_answer.call(evt.value()),
            }
        },
    };

    let class = if matches!(field.kind, Kind::Bool) {
        "field check"
    } else {
        "field"
    };

    rsx! {
        div { class,
            label { r#for: "{id}", "{label}" }
            {control}
            if let Some(description) = &field.description {
                p { id: "{help}", class: "help", "{description}" }
            }
            if let Some(problem) = &problem {
                p { id: "{error}", class: "problem", "{problem}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_trial_schema_parses_in_order() {
        let form = parse(TRIAL_SCHEMA).expect("the trial schema is a flat form");
        let names: Vec<_> = form.fields.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(
            names,
            ["supply", "motor_power", "drive", "termination", "notes"]
        );
        assert!(matches!(form.fields[0].kind, Kind::Choice(ref c) if c.len() == 2));
        assert!(form.fields[1].required);
        assert!(!form.fields[4].required);
    }

    #[test]
    fn nested_and_unknown_fields_are_refused() {
        let nested = r#"{"type":"object","properties":{"panel":{"type":"object"}}}"#;
        assert!(parse(nested).unwrap_err().contains("forms are flat"));
        let list = r#"{"type":"object","properties":{"tags":{"type":"array"}}}"#;
        assert!(parse(list).unwrap_err().contains("forms are flat"));
        let odd = r#"{"type":"object","properties":{"when":{"type":"date"}}}"#;
        assert!(parse(odd).unwrap_err().contains("unknown type"));
        assert!(parse(r#"{"type":"array"}"#).is_err());
    }

    #[test]
    fn answers_are_checked_against_the_schema() {
        let schema = r#"{"type":"object","required":["address"],"properties":{
            "address":{"type":"integer","minimum":1,"maximum":247},
            "power":{"type":"number","minimum":0.37}}}"#;
        let form = parse(schema).unwrap();
        let (address, power) = (&form.fields[0], &form.fields[1]);
        assert_eq!(
            problem(address, "").as_deref(),
            Some("An answer is needed.")
        );
        assert_eq!(
            problem(address, "248").as_deref(),
            Some("Enter a number from 1 to 247.")
        );
        assert_eq!(
            problem(address, "2.5").as_deref(),
            Some("Enter a whole number.")
        );
        assert_eq!(problem(address, "12"), None);
        assert_eq!(problem(power, ""), None);
        assert_eq!(
            problem(power, "0.2").as_deref(),
            Some("Enter 0.37 or more.")
        );
        assert_eq!(problem(power, "fast").as_deref(), Some("Enter a number."));
    }
}
