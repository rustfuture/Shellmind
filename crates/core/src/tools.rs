use async_trait::async_trait;
use serde_json::json;
use std::future::Future;
use std::pin::Pin;
use tokio::signal::unix::Signal;

use crate::{BaseTool, ConfirmationDetails, ShellmindError, ToolResult};

pub struct ReadFileTool;

#[async_trait]
impl BaseTool for ReadFileTool {
    fn name(&self) -> &'static str {
        "read_file"
    }

    fn display_name(&self) -> &'static str {
        "Read File"
    }

    fn description(&self) -> &'static str {
        "Reads the content of a specified file."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to read."
                }
            },
            "required": ["path"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("path").and_then(|p| p.as_str()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let path = params.get("path").and_then(|p| p.as_str()).unwrap_or("unknown path");
        format!("Read file: {}", path)
    }

    fn should_confirm_execute(&self, _params: &serde_json::Value) -> Option<ConfirmationDetails> {
        None // No confirmation needed for reading files
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let path = params.get("path").and_then(|p| p.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'path' parameter for ReadFileTool".to_string())
            })?;

            match tokio::fs::read_to_string(path).await {
                Ok(content) => Ok(ToolResult::Success(content)),
                Err(e) => Ok(ToolResult::Error(format!("Failed to read file '{}': {}", path, e))),
            }
        })
    }
}

pub struct WriteFileTool;

#[async_trait]
impl BaseTool for WriteFileTool {
    fn name(&self) -> &'static str {
        "write_file"
    }

    fn display_name(&self) -> &'static str {
        "Write File"
    }

    fn description(&self) -> &'static str {
        "Writes content to a specified file."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to write."
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file."
                }
            },
            "required": ["path", "content"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("path").and_then(|p| p.as_str()).is_some() &&
        params.get("content").and_then(|c| c.as_str()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let path = params.get("path").and_then(|p| p.as_str()).unwrap_or("unknown path");
        format!("Write to file: {}", path)
    }

    fn should_confirm_execute(&self, _params: &serde_json::Value) -> Option<ConfirmationDetails> {
        Some(ConfirmationDetails { message: "This will write content to a file. Are you sure?".to_string() })
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let path = params.get("path").and_then(|p| p.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'path' parameter for WriteFileTool".to_string())
            })?;
            let content = params.get("content").and_then(|c| c.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'content' parameter for WriteFileTool".to_string())
            })?;

            match tokio::fs::write(path, content).await {
                Ok(_) => Ok(ToolResult::Success(format!("Successfully wrote to file '{}'.", path))),
                Err(e) => Ok(ToolResult::Error(format!("Failed to write to file '{}': {}", path, e))),
            }
        })
    }
}

pub struct EditTool;

#[async_trait]
impl BaseTool for EditTool {
    fn name(&self) -> &'static str {
        "edit_file"
    }

    fn display_name(&self) -> &'static str {
        "Edit File"
    }

    fn description(&self) -> &'static str {
        "Edits a file by replacing an old string with a new string."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": {
                    "type": "string",
                    "description": "The path to the file to edit."
                },
                "old_string": {
                    "type": "string",
                    "description": "The string to be replaced."
                },
                "new_string": {
                    "type": "string",
                    "description": "The string to replace the old string with."
                }
            },
            "required": ["file_path", "old_string", "new_string"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("file_path").and_then(|p| p.as_str()).is_some() &&
        params.get("old_string").and_then(|o| o.as_str()).is_some() &&
        params.get("new_string").and_then(|n| n.as_str()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let file_path = params.get("file_path").and_then(|p| p.as_str()).unwrap_or("unknown file");
        let old_string = params.get("old_string").and_then(|o| o.as_str()).unwrap_or("unknown old string");
        let new_string = params.get("new_string").and_then(|n| n.as_str()).unwrap_or("unknown new string");
        format!("Edit file '{}': replace \"{}\" with \"{}\"", file_path, old_string, new_string)
    }

    fn should_confirm_execute(&self, _params: &serde_json::Value) -> Option<ConfirmationDetails> {
        Some(ConfirmationDetails { message: "This will modify a file. Are you sure?".to_string() })
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let file_path = params.get("file_path").and_then(|p| p.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'file_path' parameter for EditTool".to_string())
            })?;
            let old_string = params.get("old_string").and_then(|o| o.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'old_string' parameter for EditTool".to_string())
            })?;
            let new_string = params.get("new_string").and_then(|n| n.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'new_string' parameter for EditTool".to_string())
            })?;

            match tokio::fs::read_to_string(file_path).await {
                Ok(content) => {
                    let new_content = content.replace(old_string, new_string);
                    match tokio::fs::write(file_path, new_content).await {
                        Ok(_) => Ok(ToolResult::Success(format!("Successfully edited file '{}'.", file_path))),
                        Err(e) => Ok(ToolResult::Error(format!("Failed to write to file '{}': {}", file_path, e))),
                    }
                },
                Err(e) => Ok(ToolResult::Error(format!("Failed to read file '{}': {}", file_path, e))),
            }
        })
    }
}

pub struct LSTool;

#[async_trait]
impl BaseTool for LSTool {
    fn name(&self) -> &'static str {
        "list_directory"
    }

    fn display_name(&self) -> &'static str {
        "List Directory"
    }

    fn description(&self) -> &'static str {
        "Lists the contents of a specified directory."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the directory to list."
                }
            },
            "required": ["path"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("path").and_then(|p| p.as_str()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let path = params.get("path").and_then(|p| p.as_str()).unwrap_or("current directory");
        format!("List contents of directory: {}", path)
    }

    fn should_confirm_execute(&self, _params: &serde_json::Value) -> Option<ConfirmationDetails> {
        None // Listing directory contents is generally safe
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let path = params.get("path").and_then(|p| p.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'path' parameter for LSTool".to_string())
            })?;

            let mut entries = tokio::fs::read_dir(path).await
                .map_err(|e| ShellmindError::Other(format!("Failed to read directory '{}': {}", path, e)))?;

            let mut file_names = Vec::new();
            while let Some(entry) = entries.next_entry().await
                .map_err(|e| ShellmindError::Other(format!("Failed to read directory entry: {}", e)))? {
                file_names.push(entry.file_name().to_string_lossy().into_owned());
            }
            file_names.sort();

            Ok(ToolResult::Success(file_names.join("\n")))
        })
    }
}

pub struct GrepTool;

#[async_trait]
impl BaseTool for GrepTool {
    fn name(&self) -> &'static str {
        "search_file_content"
    }

    fn display_name(&self) -> &'static str {
        "Search File Content"
    }

    fn description(&self) -> &'static str {
        "Searches for a regular expression pattern within the content of files in a specified directory."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The absolute path to the directory to search within. If omitted, searches the current working directory."
                },
                "pattern": {
                    "type": "string",
                    "description": "The regular expression (regex) pattern to search for within file contents."
                },
                "include": {
                    "type": "string",
                    "description": "Optional: A glob pattern to filter which files are searched (e.g., *.js, *.{ts,tsx}, src/**). If omitted, searches all files."
                }
            },
            "required": ["pattern"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("pattern").and_then(|p| p.as_str()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let pattern = params.get("pattern").and_then(|p| p.as_str()).unwrap_or("unknown pattern");
        let path = params.get("path").and_then(|p| p.as_str()).unwrap_or("current directory");
        format!("Search for pattern \"{}\" in files under '{}'", pattern, path)
    }

    fn should_confirm_execute(&self, _params: &serde_json::Value) -> Option<ConfirmationDetails> {
        None // Searching file content is generally safe
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let pattern_str = params.get("pattern").and_then(|p| p.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'pattern' parameter for GrepTool".to_string())
            })?;
            let path_str = params.get("path").and_then(|p| p.as_str()).unwrap_or(".");
            let include_glob = params.get("include").and_then(|i| i.as_str());

            let regex = regex::Regex::new(pattern_str)
                .map_err(|e| ShellmindError::Other(format!("Invalid regex pattern: {}", e)))?;

            let mut results = Vec::new();
            let walker = ignore::WalkBuilder::new(path_str)
                .git_ignore(true)
                .build();

            for entry in walker {
                let entry = entry.map_err(|e| ShellmindError::Other(format!("Error walking directory: {}", e)))?;
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    let file_path = entry.path();
                    if let Some(glob_pattern) = include_glob {
                        if !file_path.to_string_lossy().contains(glob_pattern) { // Simple glob check for now
                            continue;
                        }
                    }

                    let content = tokio::fs::read_to_string(file_path).await
                        .map_err(|e| ShellmindError::Other(format!("Failed to read file '{}': {}", file_path.display(), e)))?;

                    for (line_num, line) in content.lines().enumerate() {
                        if regex.is_match(line) {
                            results.push(format!("{}:{}:{}", file_path.display(), line_num + 1, line));
                        }
                    }
                }
            }

            if results.is_empty() {
                Ok(ToolResult::Success("No matches found.".to_string()))
            } else {
                Ok(ToolResult::Success(results.join("\n")))
            }
        })
    }
}

pub struct GlobTool;

#[async_trait]
impl BaseTool for GlobTool {
    fn name(&self) -> &'static str {
        "glob"
    }

    fn display_name(&self) -> &'static str {
        "Glob Search"
    }

    fn description(&self) -> &'static str {
        "Finds files matching specific glob patterns."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The glob pattern to match against (e.g., src/**/*.py, docs/*.md)."
                },
                "path": {
                    "type": "string",
                    "description": "Optional: The absolute path to the directory to search within. If omitted, searches the root directory."
                }
            },
            "required": ["pattern"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("pattern").and_then(|p| p.as_str()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let pattern = params.get("pattern").and_then(|p| p.as_str()).unwrap_or("unknown pattern");
        let path = params.get("path").and_then(|p| p.as_str()).unwrap_or("current directory");
        format!("Find files matching pattern \"{}\" in '{}'", pattern, path)
    }

    fn should_confirm_execute(&self, _params: &serde_json::Value) -> Option<ConfirmationDetails> {
        None // Glob search is generally safe
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let pattern_str = params.get("pattern").and_then(|p| p.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'pattern' parameter for GlobTool".to_string())
            })?;
            let path_str = params.get("path").and_then(|p| p.as_str()).unwrap_or(".");

            let mut results = Vec::new();
            let glob_pattern = format!("{}/{}", path_str, pattern_str);

            for entry in glob::glob(&glob_pattern)
                .map_err(|e| ShellmindError::Other(format!("Invalid glob pattern: {}", e)))? {
                match entry {
                    Ok(path) => results.push(path.to_string_lossy().into_owned()),
                    Err(e) => return Err(ShellmindError::Other(format!("Error matching glob pattern: {}", e))),
                }
            }
            results.sort();

            if results.is_empty() {
                Ok(ToolResult::Success("No matches found.".to_string()))
            } else {
                Ok(ToolResult::Success(results.join("\n")))
            }
        })
    }
}

pub struct ShellTool;

#[async_trait]
impl BaseTool for ShellTool {
    fn name(&self) -> &'static str {
        "run_shell_command"
    }

    fn display_name(&self) -> &'static str {
        "Run Shell Command"
    }

    fn description(&self) -> &'static str {
        "Executes a given shell command."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The exact bash command to execute."
                },
                "description": {
                    "type": "string",
                    "description": "Brief description of the command for the user. Be specific and concise."
                }
            },
            "required": ["command"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("command").and_then(|c| c.as_str()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let command = params.get("command").and_then(|c| c.as_str()).unwrap_or("unknown command");
        let description = params.get("description").and_then(|d| d.as_str()).unwrap_or("no description");
        format!("Run shell command: '{}' (Description: {})", command, description)
    }

    fn should_confirm_execute(&self, params: &serde_json::Value) -> Option<ConfirmationDetails> {
        let command = params.get("command").and_then(|c| c.as_str()).unwrap_or("unknown command");
        Some(ConfirmationDetails { message: format!("This will execute the command: '{}'. Are you sure?", command) })
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let command_str = params.get("command").and_then(|c| c.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'command' parameter for ShellTool".to_string())
            })?;

            let output = if cfg!(target_os = "windows") {
                tokio::process::Command::new("cmd")
                    .args(&["/C", command_str])
                    .output()
                    .await
                    .map_err(|e| ShellmindError::Other(format!("Failed to execute command: {}", e)))?
            } else {
                tokio::process::Command::new("sh")
                    .arg("-c")
                    .arg(command_str)
                    .output()
                    .await
                    .map_err(|e| ShellmindError::Other(format!("Failed to execute command: {}", e)))?
            };

            if output.status.success() {
                Ok(ToolResult::Success(String::from_utf8_lossy(&output.stdout).to_string()))
            } else {
                Ok(ToolResult::Error(format!("Command failed with exit code {:?}: {}", output.status.code(), String::from_utf8_lossy(&output.stderr))))
            }
        })
    }
}

pub struct WebFetchTool;

#[async_trait]
impl BaseTool for WebFetchTool {
    fn name(&self) -> &'static str {
        "web_fetch"
    }

    fn display_name(&self) -> &'static str {
        "Web Fetch"
    }

    fn description(&self) -> &'static str {
        "Fetches content from a specified URL."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to fetch content from."
                }
            },
            "required": ["url"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("url").and_then(|u| u.as_str()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let url = params.get("url").and_then(|u| u.as_str()).unwrap_or("unknown URL");
        format!("Fetch content from URL: {}", url)
    }

    fn should_confirm_execute(&self, _params: &serde_json::Value) -> Option<ConfirmationDetails> {
        None // Fetching web content is generally safe
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let url = params.get("url").and_then(|u| u.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'url' parameter for WebFetchTool".to_string())
            })?;

            match reqwest::get(url).await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.text().await {
                            Ok(text) => Ok(ToolResult::Success(text)),
                            Err(e) => Ok(ToolResult::Error(format!("Failed to read response text: {}", e))),
                        }
                    } else {
                        Ok(ToolResult::Error(format!("Failed to fetch URL: {} (Status: {})", url, response.status())))
                    }
                },
                Err(e) => Ok(ToolResult::Error(format!("Failed to send request to URL: {}", e))),
            }
        })
    }
}

pub struct WebSearchTool;

#[async_trait]
impl BaseTool for WebSearchTool {
    fn name(&self) -> &'static str {
        "google_web_search"
    }

    fn display_name(&self) -> &'static str {
        "Google Web Search"
    }

    fn description(&self) -> &'static str {
        "Performs a web search using Google Search (via the Gemini API) and returns the results."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query to find information on the web."
                }
            },
            "required": ["query"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("query").and_then(|q| q.as_str()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let query = params.get("query").and_then(|q| q.as_str()).unwrap_or("unknown query");
        format!("Search the web for: {}", query)
    }

    fn should_confirm_execute(&self, _params: &serde_json::Value) -> Option<ConfirmationDetails> {
        None // Web search is generally safe
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let query = params.get("query").and_then(|q| q.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'query' parameter for WebSearchTool".to_string())
            })?;

            // Placeholder for actual Google Web Search API call
            // In a real scenario, this would involve calling the Gemini API with a search tool request.
            // For now, we'll return a dummy result.
            Ok(ToolResult::Success(format!("Search results for '{}': [Dummy result from Google Search]", query)))
        })
    }
}

pub struct MemoryTool;

#[async_trait]
impl BaseTool for MemoryTool {
    fn name(&self) -> &'static str {
        "save_memory"
    }

    fn display_name(&self) -> &'static str {
        "Save Memory"
    }

    fn description(&self) -> &'static str {
        "Saves a specific piece of information or fact to your long-term memory."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "fact": {
                    "type": "string",
                    "description": "The specific fact or piece of information to remember."
                }
            },
            "required": ["fact"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("fact").and_then(|f| f.as_str()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let fact = params.get("fact").and_then(|f| f.as_str()).unwrap_or("unknown fact");
        format!("Save to memory: {}", fact)
    }

    fn should_confirm_execute(&self, _params: &serde_json::Value) -> Option<ConfirmationDetails> {
        None // Saving to memory is generally safe
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let fact = params.get("fact").and_then(|f| f.as_str()).ok_or_else(|| {
                ShellmindError::Other("Missing 'fact' parameter for MemoryTool".to_string())
            })?;

            // In a real scenario, this would write to a persistent memory store.
            // For now, we'll just acknowledge the save.
            Ok(ToolResult::Success(format!("Fact saved to memory: '{}'.", fact)))
        })
    }
}

pub struct ReadManyFilesTool;

#[async_trait]
impl BaseTool for ReadManyFilesTool {
    fn name(&self) -> &'static str {
        "read_many_files"
    }

    fn display_name(&self) -> &'static str {
        "Read Many Files"
    }

    fn description(&self) -> &'static str {
        "Reads content from multiple files specified by paths or glob patterns."
    }

    fn parameter_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "paths": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "An array of glob patterns or paths to files/directories."
                }
            },
            "required": ["paths"]
        })
    }

    fn validate_tool_params(&self, params: &serde_json::Value) -> bool {
        params.get("paths").and_then(|p| p.as_array()).is_some()
    }

    fn get_description(&self, params: &serde_json::Value) -> String {
        let paths = params.get("paths").and_then(|p| p.as_array()).map(|arr| {
            arr.iter().filter_map(|v| v.as_str()).collect::<Vec<&str>>().join(", ")
        }).unwrap_or("unknown paths".to_string());
        format!("Read content from multiple files: {}", paths)
    }

    fn should_confirm_execute(&self, _params: &serde_json::Value) -> Option<ConfirmationDetails> {
        None // Reading files is generally safe
    }

    fn execute(&self, params: serde_json::Value, _signal: Option<Signal>) -> Pin<Box<dyn Future<Output = Result<ToolResult, ShellmindError>> + Send>> {
        Box::pin(async move {
            let paths_json = params.get("paths").and_then(|p| p.as_array()).ok_or_else(|| {
                ShellmindError::Other("Missing 'paths' parameter for ReadManyFilesTool".to_string())
            })?;

            let mut all_content = Vec::new();

            for path_json in paths_json {
                let path_str = path_json.as_str().ok_or_else(|| {
                    ShellmindError::Other("Invalid path in 'paths' array for ReadManyFilesTool".to_string())
                })?;

                // Handle glob patterns
                if path_str.contains('*') || path_str.contains('?') || path_str.contains('[') {
                    for entry in glob::glob(path_str)
                        .map_err(|e| ShellmindError::Other(format!("Invalid glob pattern '{}': {}", path_str, e)))? {
                        match entry {
                            Ok(path) => {
                                if path.is_file() {
                                    match tokio::fs::read_to_string(&path).await {
                                        Ok(content) => all_content.push(format!("--- {} ---
{}", path.display(), content)),
                                        Err(e) => all_content.push(format!("--- {} ---
Error reading file: {}", path.display(), e)),
                                    }
                                }
                            },
                            Err(e) => all_content.push(format!("Error matching glob entry: {}", e)),
                        }
                    }
                } else { // Handle direct file/directory paths
                    let path = std::path::PathBuf::from(path_str);
                    if path.is_file() {
                        match tokio::fs::read_to_string(&path).await {
                            Ok(content) => all_content.push(format!("--- {} ---
{}", path.display(), content)),
                            Err(e) => all_content.push(format!("--- {} ---
Error reading file: {}", path.display(), e)),
                        }
                    } else if path.is_dir() {
                        for entry in walkdir::WalkDir::new(&path) {
                            let entry = entry.map_err(|e| ShellmindError::Other(format!("Error walking directory: {}", e)))?;
                            if entry.file_type().is_file() {
                                let file_path = entry.path();
                                match tokio::fs::read_to_string(file_path).await {
                                    Ok(content) => all_content.push(format!("--- {} ---
{}", file_path.display(), content)),
                                    Err(e) => all_content.push(format!("--- {} ---
Error reading file: {}", file_path.display(), e)),
                                }
                            }
                        }
                    } else {
                        all_content.push(format!("--- {} ---
File or directory not found.", path.display()));
                    }
                }
            }

            if all_content.is_empty() {
                Ok(ToolResult::Success("No readable files found.".to_string()))
            } else {
                Ok(ToolResult::Success(all_content.join("\n")))
            }
        })
    }
}
