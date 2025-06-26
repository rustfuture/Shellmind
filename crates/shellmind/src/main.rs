use core::ShellmindConfig;
use ui::display_banner;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Yapılandırmayı yükle
    let config = ShellmindConfig::load()?;

    // Banner'ı göster
    display_banner();

    println!("Shellmind başlatıldı. Çıkmak için 'exit' yazın.");

    // Ana interaktif döngü
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        // TODO: Kullanıcı girdisini işlemek ve core crate'ini çağırmak için mantık ekle
        println!("Girdiniz: {}", input);
        // Örnek: core::process_input(input).await?;
    }

    println!("Shellmind kapatılıyor.");

    Ok(())
}

// TODO: core ve ui crate'lerinden gerekli fonksiyonları içe aktarın ve kullanın.
// Şu an için sadece temel yapıyı oluşturduk.