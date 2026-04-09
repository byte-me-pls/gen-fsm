# 🧬 gen-fsm (Genetic Finite State Machine)

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![no_std](https://img.shields.io/badge/no__std-compatible-blue.svg)](#)
[![Educational Project](https://img.shields.io/badge/Project-Educational_Study-purple.svg)](#)
[![Status: Work in Progress](https://img.shields.io/badge/Status-Work_in_Progress-orange.svg)](#)

> 🚧 **WORK IN PROGRESS & STUDY CASE**: Bu proje temelde deneysel bir **öğrenim/çalışma (study case) projesi** olarak geliştirilmektedir ve aktif olarak geliştirilme aşamasındadır. Endüstriyel kullanımdan ziyade potansiyel kapasiteleri keşfetmeyi hedefler. API'ler ve algoritmalar sık sık değişebilir.

**Gen-FSM**, otonom sistemler, robotik ve gömülü cihazlar (embedded IoT) için saf Rust ile geliştirilmiş, **`no_std` uyumlu**, yenilikçi bir Stokastik-Genetik Durum Makinesi kütüphanesidir.

Geleneksel kontrol sistemlerindeki katı, kuralcı (if/else tabanlı) ve öngörülemeyen çevre koşullarında kolayca çuvallayan geçiş mekanizmalarının yerine; durumlar arası geçişleri **olasılıksal (stokastik) matrislerle** yönetir ve bu kuralları **genetik algoritmalar** kullanarak simülasyon ortamında kendi kendine evrimleştirir.

---

## 🌟 Neden Gen-FSM?

Otonom bir drone veya çizgi izleyen robot kodladığınızı düşünün. Duvara yaklaşınca ne yapacak? Sensörler gürültülü (noisy) bir veri okuduğunda nasıl davranacak?

Klasik `if/else` blokları karmaşık ortamlarda yönetilemez hale gelir. Gen-FSM ile:

1. **Simüle Et ve Evrimleştir**: Sisteminizin hedeflerine (fitness) en uygun olasılık matrisini bilgisayarınızda dakikalar içinde hesaplatın (Evrim Motoru).
2. **Sıfır Maliyet (Zero-Cost)**: Ortaya çıkan bu optimize edilmiş, hafif "Yapay DNA"yı doğrudan mikrodenetleyicilere (STM32, ESP32 vb.) gömün.
3. **Adaptasyon**: Dronelarınıza veya endüstriyel robotlarınıza sadece bir hedefe ulaşmayı değil, hayatta kalmayı ve ortama adapte olmayı öğretin.

## 🏗️ Mimari Yapı

Proje iki ana bileşenden (crate) oluşmaktadır:

### 1. `gen-fsm` (Çekirdek Kütüphane)
- **`no_std`** uyumlu ve sıfır bağımlılığa sahip.
- Sadece mikrodenetleyici veya hedeflenen gömülü sistem üzerinde koşar.
- Dinamik bellek tahsisi (`alloc`) gerektirmez. Geçiş matrisleri düz `f32` dizileri olarak (Flash bellekte) saklanabilir.
- Hızlı stokastik kararlar alabilmek için gömülü uyumlu hafif, yerleşik `Xorshift32` PRNG içerir.

### 2. `gen-fsm-evolve` (Evrim Motoru)
- Bilgisayarınızda (Host) çalışır ve `std` kullanır.
- `rayon` ile çoklu çekirdek üzerinde paralel "fitness" değerlendirmesi yapar.
- Genetik Operatörler: Tournament Seçilimi (Selection), Uniform Crossover, Box-Muller Gauss Mutasyonu.
- Elde edilen başarılı "DNA" sonuçlarını doğrudan **Rust Const Array**, **C Header**, Binary veya JSON olarak dışarı aktarabilir (Export).

---

## 🚀 Örnek Kullanım (Study Case: Drone Navigation)

Proje içerisinde yer alan `drone_nav` örneği, Gen-FSM'in gücünü göstermek için tasarlanmıştır. Engellerle dolu 2 boyutlu bir ızgara (grid) ortamında, drone hedefine ulaşmak için hangi sensör verisinde (Bağlam/Context) nasıl davranması (Durum/State) gerektiğini kendi kendine öğrenir.

Örneği çalıştırmak için:

```bash
cargo run --release --example drone_nav
```

**Evrim işlemi tamamlandıktan sonra üretilen Rust kodu (DNA) örneği:**
```rust
use gen_fsm::FsmDna;

pub const DRONE_DNA: [f32; 64] = [
    // State geçiş olasılıkları (Algoritma tarafından optimize edildi)
    0.803456, 0.150000, 0.046544, 0.000000, 
    // ...
];
```

## 🛠️ Kurulum ve Derleme

Bu proje `cargo` çalışma alanlarına (workspace) göre yapılandırılmıştır.

```bash
# Repoyu klonlayıp içine girin
git clone https://github.com/yourusername/gen-fsm.git
cd gen-fsm

# Projeyi Derleyin (Windows üzerinde MSYS2/MinGW toolchain kullanılabilir)
cargo build --all-targets

# Tüm birim testlerini (Genetik algoritmalar ve Matrix yapıları vb.) çalıştırın
cargo test --all-targets
```

## ⏳ Gelecek Planları & Yol Haritası (Roadmap)

- [x] Çekirdek Stokastik FSM Motoru (`no_std` uyumlu).
- [x] Evrim Motoru ve Genetik Algoritma Yapıları.
- [x] Gömülü Sistemler İçin C & Rust DNA Exporter.
- [x] İlk Study Case (Örnek: `drone_nav`).
- [ ] İkinci Study Case (Örnek: Çizgi İzleyen / Line Follower Robot).
- [ ] Detaylı API Dokümantasyonunun (`rustdoc`) hazırlanması.
- [ ] GitHub Actions CI/CD (Sürekli Entegrasyon) ve otomatik test pipeline'larının kurulması.

## 📄 Lisans

Bu proje, açık kaynaklı olarak geliştirilmektedir. Lisans detayları daha sonra eklenecektir.
