//! MCP Tools for Leptos
//!
//! Implements the tool handlers for the MCP server.

use crate::docs;

/// Leptos Tools implementation
pub struct LeptosTools {}

impl LeptosTools {
    pub fn new() -> Self {
        Self {}
    }

    /// List all available Leptos documentation sections
    pub fn list_sections(&self) -> String {
        let sections = docs::list_sections();
        let output: Vec<String> = sections
            .iter()
            .map(|s| {
                format!(
                    "* title: {}, use_cases: {}, path: {}",
                    s.title, s.use_cases, s.path
                )
            })
            .collect();
        output.join("\n")
    }

    /// Get documentation content for a specific section
    pub fn get_documentation(&self, section: &str) -> String {
        if let Some(doc) = docs::get_section(section) {
            format!("# {}\n\n{}", doc.title, doc.content)
        } else {
            format!(
                "Section '{}' not found. Use list-sections to see available sections.",
                section
            )
        }
    }

    /// Analyze Leptos code and suggest fixes
    pub fn leptos_autofixer(&self, code: &str) -> String {
        let mut suggestions = Vec::new();

        // TODO: Add more checks
        
        // Check for common issues

        // 1. Check for direct .get() in view without move ||
        if code.contains(".get()") && !code.contains("move ||") && code.contains("view!") {
            suggestions.push(
                "ERROR: Found .get() in view without `move ||`. \
                 Reactive values should use `{move || value.get()}`",
            );
        }

        // 2. Check for signal without destructuring
        if code.contains("let signal =") || code.contains("create_signal") {
            suggestions.push(
                "WARNING: Consider using `let (getter, setter) = signal(value)` pattern for clarity",
            );
        }

        // 3. Check for missing #[component] macro
        if code.contains("-> impl IntoView") && !code.contains("#[component]") {
            suggestions.push(
                "ERROR: Functions returning `impl IntoView` should have #[component] attribute",
            );
        }

        // 4. Check for server function without proper error handling
        if code.contains("#[server") && !code.contains("ServerFnError") {
            suggestions.push("INFO: Server functions should return Result<T, ServerFnError>");
        }

        // 5. Check for deprecated create_signal
        if code.contains("create_signal") {
            suggestions.push(
                "INFO: In Leptos 0.8+, use `signal()` instead of `create_signal()`",
            );
        }

        if suggestions.is_empty() {
            "✓ No issues found. Code looks good!".to_string()
        } else {
            suggestions.join("\n")
        }
    }
}
