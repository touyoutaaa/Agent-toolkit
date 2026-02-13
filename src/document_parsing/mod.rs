pub mod parsers;

use std::path::Path;

/// Supported document formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentFormat {
    Pdf,
    Docx,
    Xlsx,
    Pptx,
    Html,
    Csv,
    Json,
    Xml,
    Txt,
    Markdown,
}

impl DocumentFormat {
    /// Detect format from file extension. Returns None for unsupported formats.
    pub fn from_extension(path: &str) -> Option<Self> {
        let clean_path = path.split('?').next().unwrap_or(path);

        let ext = Path::new(clean_path)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())?;

        match ext.as_str() {
            "pdf" => Some(Self::Pdf),
            "docx" => Some(Self::Docx),
            "xlsx" => Some(Self::Xlsx),
            "pptx" => Some(Self::Pptx),
            "html" | "htm" => Some(Self::Html),
            "csv" => Some(Self::Csv),
            "json" => Some(Self::Json),
            "xml" => Some(Self::Xml),
            "txt" | "text" | "log" => Some(Self::Txt),
            "md" | "markdown" => Some(Self::Markdown),
            _ => None,
        }
    }

    pub fn from_str_name(name: &str) -> Option<Self> {
        let dummy = format!("file.{}", name);
        Self::from_extension(&dummy)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Docx => "docx",
            Self::Xlsx => "xlsx",
            Self::Pptx => "pptx",
            Self::Html => "html",
            Self::Csv => "csv",
            Self::Json => "json",
            Self::Xml => "xml",
            Self::Txt => "txt",
            Self::Markdown => "markdown",
        }
    }
}

/// The result of parsing a document.
#[derive(Debug, Clone)]
pub struct ParsedDocument {
    /// Extracted text content.
    pub text: String,
    /// Optional metadata (page count, sheet names, etc.).
    pub metadata: serde_json::Value,
}
