use serde::Serialize;

use crate::source::Span;

#[derive(Serialize)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Serialize)]
pub struct Label {
    pub span: Span,
    pub message: String,
}

#[derive(Serialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
    pub helps: Vec<String>,
}

pub trait DiagnosticTrait {
    fn severity(&self) -> Severity;
    fn message(&self) -> String;
    fn code(&self) -> Option<&'static str>;
    fn primary_span(&self) -> Span;

    fn labels(&self) -> Vec<Label> {
        Vec::new()
    }

    fn notes(&self) -> Vec<String> {
        Vec::new()
    }

    fn helps(&self) -> Vec<String> {
        Vec::new()
    }
}

pub struct DiagnosticBag {
    items: Vec<Box<dyn DiagnosticTrait>>,
}

pub trait DiagnosticEmitter {
    type Output;

    fn emit(&self, diagnostics: &[Box<dyn DiagnosticTrait>]) -> Self::Output;
}
