# ✨ Shellmind: Yapay Zeka Destekli Komut Satırı Yoldaşınız ✨

Shellmind, Rust ile titizlikle hazırlanmış **gelişmiş, açık kaynaklı, terminal tabanlı bir yapay zeka asistanıdır**. Geliştiriciler, sistem yöneticileri ve komut satırıyla yoğun bir şekilde çalışan herkes için güçlü ve esnek bir araç olarak tasarlanmıştır.

Sonsuz man sayfalarına ve unutulmuş komutlara veda edin. Shellmind ile sadece sorarsınız ve o size cevabı sunar!

## 🚀 Sizi Güçlendiren Özellikler

*   **Sezgisel Doğal Dil Arayüzü:** 🗣️ Doğal dil sorgularınızı kesin, yürütülebilir kabuk komutlarına çevirin. Shellmind niyetinizi anlar ve iş için doğru komutu sağlar.
*   **Esnek Yapay Zeka Entegrasyonu:** 🧠 En son yapay zeka modelleriyle sorunsuz bir şekilde bağlantı kurun. Gemini modelleri için hem **RESTful API** hem de yüksek performanslı **gRPC** iletişimini destekleyerek en iyi performansı ve esnekliği sağlar.
*   **Dinamik Yapılandırma Yönetimi:** ⚙️ Shellmind ayarlarınızı doğrudan CLI'dan kolayca görüntüleyin ve güncelleyin. API anahtarlarını, model adlarını, sıcaklıkları, bağlam penceresi boyutlarını, API türlerini, gRPC uç noktalarını ve hatta yapay zekanın temel sistem istemini özelleştirin.
*   **Gerçek Zamanlı Görsel Geri Bildirim:** 💡 Shellmind'in ne zaman düşündüğünü, işlediğini veya yanıtlar ürettiğini gösteren animasyonlu spinner'lar ve net durum mesajlarıyla bilgi sahibi olun.
*   **Modüler ve Genişletilebilir Mimari:** 🏗️ Rust'ta temiz, modüler bir tasarımla inşa edilmiştir, bu da yeni işlevleri veya yapay zeka hizmetlerini genişletmeyi, özelleştirmeyi ve entegre etmeyi inanılmaz derecede kolaylaştırır.
*   **Etkileşimli ve Etkileşimli Olmayan Modlar:** 💬 Sürekli yardım için sürekli bir etkileşimli oturuma katılın veya hızlı, tek seferlik sorgular için doğrudan CLI komutlarını kullanın.
*   **Çapraz Platform Uyumluluğu:** 🌐 Linux, macOS ve Windows'ta sorunsuz çalışır ve geliştirme ortamlarınızda tutarlı bir deneyim sunar.
*   **Gelişmiş Araçlar için Yer Tutucu:** 🛠️ Güvenli depolama ve multimedya işleme (örn. `ffmpeg` ile video) gibi gelecekteki geliştirmeler için temel modüller içerir.

## ⚡ Başlarken: Shellmind'i Serbest Bırakın

### Ön Koşullar

Başlamadan önce şunlara sahip olduğunuzdan emin olun:

*   **Rust:** `rustup.rs` adresinden `rustup` aracılığıyla Rust'ı yükleyin ([https://rustup.rs/](https://rustup.rs/)).
*   **Bir Gemini API Anahtarı:** API anahtarınızı [Google AI Studio](https://aistudio.google.com/) adresinden edinin.
*   **Protocol Buffers Derleyicisi (`protoc`):** gRPC desteği için gereklidir. Sisteminizin paket yöneticisi aracılığıyla yükleyin (örn. Debian/Ubuntu'da `sudo apt-get install protobuf-compiler`).

### Kurulum

1.  **Depoyu Klonlayın:**

    ```bash
    git clone https://github.com/your-username/shellmind.git
    cd shellmind
    ```

2.  **API Anahtarınızı Yapılandırın:**

    Projenin kök dizininde bir `.env` dosyası oluşturun ve Gemini API anahtarınızı ekleyin:

    ```dotenv
    SHELLMIND_API_KEY=BURAYA_GEMINI_API_ANAHTARINIZI_GIRIN
    ```

    Alternatif olarak, bir ortam değişkeni olarak ayarlayabilir veya derledikten sonra CLI aracılığıyla yapılandırabilirsiniz.

3.  **Shellmind'i Derleyin:**

    ```bash
    cargo build --release
    ```

### Kullanım

#### Etkileşimli Mod

Sürekli, etkileşimli bir yapay zeka asistanı oturumu için:

```bash
cargo run --bin shellmind
```

Doğal dil sorgularınızı yazın ve Shellmind ilgili kabuk komutlarını sağlayacaktır. Çıkmak için `exit` yazın.

#### Doğrudan Komut Satırı Sorguları

Hızlı, tek seferlik komut üretimi için:

```bash
cargo run --bin shellmind prompt --text "mevcut dizindeki tüm dosyaları listele"
```

#### CLI Yapılandırması

Mevcut yapılandırmanızı görüntüleyin:

```bash
cargo run --bin shellmind config show
```

Bir yapılandırma değeri ayarlayın (örn. modeli veya API türünü değiştirin):

```bash
cargo run --bin shellmind config set model_name gemini-1.5-pro
cargo run --bin shellmind config set api_type grpc
cargo run --bin shellmind config set grpc_endpoint https://generativelanguage.googleapis.com
cargo run --bin shellmind config set system_prompt "Rust programlama için faydalı bir asistansın."
```

## ⚙️ Yapılandırma Seçenekleri

Shellmind'in davranışı, `~/.shellmind/` konumunda bulunan bir `config.toml` dosyası veya `SHELLMIND_` önekiyle başlayan ortam değişkenleri (örn. `SHELLMIND_API_KEY`) aracılığıyla özelleştirilebilir.

Temel yapılandırma seçenekleri şunları içerir:

*   `api_key`: Gemini API anahtarınız. (Gerekli)
*   `model_name`: Kullanılacak belirli Gemini modeli (örn. `gemini-1.5-flash`, `gemini-1.5-pro`).
*   `temperature`: Yapay zekanın çıktısının rastgeleliğini kontrol eder (0.0 ila 1.0). Daha düşük değerler daha deterministik sonuçlar üretir.
*   `context_window_size`: Yapay zekaya gönderilen konuşma geçmişine dahil edilecek önceki dönüşlerin maksimum sayısı.
*   `api_type`: API iletişimi için `Rest` (varsayılan) veya `Grpc` kullanılacağını belirtir.
*   `grpc_endpoint`: `api_type` `Grpc` olarak ayarlanmışsa gRPC uç nokta URL'si.
*   `system_prompt`: Yapay zekaya verilen başlangıç talimatı veya kişiliği. Shellmind'in yanıtlarını özelleştirmek için bunu ayarlayın.

## 🤝 Katkıda Bulunma

Katkılarınızı memnuniyetle karşılıyoruz! Rust, yapay zeka ve komut satırı araçları konusunda tutkuluysanız, depoyu çatallamaktan, sorunlar açmaktan veya çekme istekleri göndermekten çekinmeyin. Ayrıntılı yönergeler için lütfen `CONTRIBUTING.md`'ye (yakında!) bakın.

## 📜 Lisans

Bu proje Apache Lisansı 2.0 altında lisanslanmıştır. Ayrıntılar için [LICENSE](LICENSE) dosyasına bakın.

---

**Shellmind** bağımsız bir projedir ve Google veya Gemini ekibiyle ilişkili değildir. İşlevselliği için herkese açık Gemini API'sini kullanır.
