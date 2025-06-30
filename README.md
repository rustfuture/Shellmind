# ✨ Shellmind: Your AI-Powered Command Line Companion ✨

Shellmind is an **advanced, open-source, terminal-based AI assistant** meticulously crafted in Rust. Designed for developers, system administrators, and power users, Shellmind transforms your command-line experience by bringing intelligent automation and seamless interaction directly to your fingertips.

Say goodbye to endless man pages and forgotten commands. With Shellmind, you can simply ask, and it delivers!

## 🚀 Features That Empower You

*   **Intuitive Natural Language Interface:** 🗣️ Translate your natural language queries into precise, executable shell commands. Shellmind understands your intent and provides the right command for the job.
*   **Flexible AI Integration:** 🧠 Seamlessly connect with cutting-edge AI models. Supports both **RESTful API** and high-performance **gRPC** communication for Gemini models, ensuring you get the best performance and flexibility.
*   **Dynamic Configuration Management:** ⚙️ Easily view and update your Shellmind settings directly from the CLI. Customize API keys, model names, temperatures, context window sizes, API types, gRPC endpoints, and even the AI's core system prompt.
*   **Real-time Visual Feedback:** 💡 Stay informed with animated spinners and clear status messages that indicate when Shellmind is thinking, processing, or generating responses.
*   **Modular & Extensible Architecture:** 🏗️ Built with a clean, modular design in Rust, making it incredibly easy to extend, customize, and integrate new functionalities or AI services.
*   **Interactive & Non-Interactive Modes:** 💬 Engage in a continuous interactive session for ongoing assistance or use direct CLI commands for quick, one-off queries.
*   **Cross-Platform Compatibility:** 🌐 Runs flawlessly on Linux, macOS, and Windows, providing a consistent experience across your development environments.
*   **Placeholder for Advanced Tools:** 🛠️ Includes foundational modules for future enhancements like secure storage and multimedia processing (e.g., video with `ffmpeg`).

## ⚡ Getting Started: Unleash Shellmind

### Prerequisites

Before you begin, ensure you have:

*   **Rust:** Install Rust via `rustup` from [rustup.rs](https://rustup.rs/).
*   **A Gemini API Key:** Obtain your API key from the [Google AI Studio](https://aistudio.google.com/).
*   **Protocol Buffers Compiler (`protoc`):** Required for gRPC support. Install it via your system's package manager (e.g., `sudo apt-get install protobuf-compiler` on Debian/Ubuntu).

### Installation

1.  **Clone the Repository:**

    ```bash
    git clone https://github.com/your-username/shellmind.git
    cd shellmind
    ```

2.  **Configure Your API Key:**

    Create a `.env` file in the root of the project and add your Gemini API key:

    ```dotenv
    SHELLMIND_API_KEY=YOUR_GEMINI_API_KEY_HERE
    ```

    Alternatively, you can set it as an environment variable or configure it via the CLI after building.

3.  **Build Shellmind:**

    ```bash
    cargo build --release
    ```

### Usage

#### Interactive Mode

For a continuous, interactive AI assistant session:

```bash
cargo run --bin shellmind
```

Type your natural language queries, and Shellmind will provide the corresponding shell commands. Type `exit` to quit.

#### Direct Command Line Queries

For quick, one-off command generation:

```bash
cargo run --bin shellmind prompt --text "list all files in current directory"
```

#### CLI Configuration

View your current configuration:

```bash
cargo run --bin shellmind config show
```

Set a configuration value (e.g., change the model or API type):

```bash
cargo run --bin shellmind config set model_name gemini-1.5-pro
cargo run --bin shellmind config set api_type grpc
cargo run --bin shellmind config set grpc_endpoint https://generativelanguage.googleapis.com
cargo run --bin shellmind config set system_prompt "You are a helpful assistant for Rust programming."
```

## ⚙️ Configuration Options

Shellmind's behavior can be customized via a `config.toml` file located in `~/.shellmind/` or through environment variables prefixed with `SHELLMIND_` (e.g., `SHELLMIND_API_KEY`).

Key configuration options include:

*   `api_key`: Your Gemini API key. (Required)
*   `model_name`: The specific Gemini model to use (e.g., `gemini-1.5-flash`, `gemini-1.5-pro`).
*   `temperature`: Controls the randomness of the AI's output (0.0 to 1.0). Lower values produce more deterministic results.
*   `context_window_size`: The maximum number of previous turns to include in the conversation history sent to the AI.
*   `api_type`: Specifies whether to use `Rest` (default) or `Grpc` for API communication.
*   `grpc_endpoint`: The gRPC endpoint URL if `api_type` is set to `Grpc`.
*   `system_prompt`: The initial instruction or persona given to the AI. Customize this to tailor Shellmind's responses.

## 🤝 Contributing

We welcome contributions! If you're passionate about Rust, AI, and command-line tools, feel free to fork the repository, open issues, or submit pull requests. Please refer to the `CONTRIBUTING.md` (coming soon!) for detailed guidelines.

## 📜 License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.

---

**Shellmind** is an independent project and is not affiliated with Google or the Gemini team. It leverages the publicly available Gemini API for its functionality.