//! The findings list. Severity is a word and a shape, never color alone (DESIGN.md, Flexibility),
//! and each finding says what's wrong, where, why (its source) and how to fix it.

use dioxus::prelude::*;
use serde::Deserialize;

const TRIAL_FINDINGS: &str = include_str!("../assets/trial/findings.json");

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Finding {
    pub severity: Severity,
    pub what: String,
    #[serde(rename = "where")]
    pub place: String,
    pub why: String,
    pub fix: String,
}

impl Severity {
    fn word(&self) -> &'static str {
        match self {
            Severity::Error => "Error",
            Severity::Warning => "Warning",
            Severity::Info => "Information",
        }
    }

    fn shape(&self) -> &'static str {
        match self {
            Severity::Error => "■",
            Severity::Warning => "▲",
            Severity::Info => "●",
        }
    }
}

#[component]
pub fn Findings() -> Element {
    let findings = use_hook(|| {
        serde_json::from_str::<Vec<Finding>>(TRIAL_FINDINGS).map_err(|e| e.to_string())
    });
    let findings = match findings {
        Ok(findings) => findings,
        Err(why) => return rsx! { p { role: "alert", "The findings couldn't be read: {why}" } },
    };
    let count = findings.len();

    rsx! {
        section { class: "findings", aria_labelledby: "findings-title",
            h2 { id: "findings-title", "Findings ({count})" }
            // Without bullets, WebKit stops calling this a list, so the role says it again.
            ul { role: "list",
                for finding in findings {
                    FindingItem { finding }
                }
            }
        }
    }
}

#[component]
fn FindingItem(finding: Finding) -> Element {
    let word = finding.severity.word();
    let class = format!("finding {}", word.to_lowercase());

    rsx! {
        li { class,
            p { class: "what",
                span { class: "shape", aria_hidden: "true", {finding.severity.shape()} }
                strong { "{word}: " }
                "{finding.what}"
            }
            p { "Where: {finding.place}" }
            p { "Why: {finding.why}" }
            p { "Fix: {finding.fix}" }
        }
    }
}
