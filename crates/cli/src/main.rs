[package]
name = "shellmind" # Ana çalıştırılabilir dosyanın adı "shellmind" olacak
version = "0.1.0"
edition = "2021"

[dependencies]
# Workspace'den gelen ortak bağımlılıkları kullan
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
colored = { workspace = true }
dotenv = { workspace = true }
reqwest = { workspace = true }

# Diğer crate'lere bağımlılıklar
core = { path = "../core" }       # Core crate'i cli'dan çağrılacağı için
ui = { path = "../ui" }           # UI crate'i de cli'dan çağrılacağı için
# tools = { path = "../tools" }   # İhtiyaca göre eklenecek
# extensions = { path = "../extensions" } # İhtiyaca göre eklenecek