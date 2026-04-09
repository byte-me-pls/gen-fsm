# gen-fsm (Genetic Finite State Machine)

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![no_std](https://img.shields.io/badge/no__std-compatible-blue.svg)](#)
[![Educational Project](https://img.shields.io/badge/Project-Educational_Study-purple.svg)](#)
[![Status: Work in Progress](https://img.shields.io/badge/Status-Work_in_Progress-orange.svg)](#)

> 🚧 **WORK IN PROGRESS & STUDY CASE**: Bu proje temelde deneysel bir **öğrenim/çalışma (study case) projesi** olarak geliştirilmektedir ve aktif olarak geliştirilme aşamasındadır. Endüstriyel kullanımdan ziyade potansiyel kapasiteleri keşfetmeyi hedefler. API'ler ve algoritmalar sık sık değişebilir.

**Gen-FSM**, otonom sistemler, robotik ve gömülü cihazlar (embedded IoT) için saf Rust ile geliştirilmiş, **`no_std` uyumlu**, yenilikçi bir Stokastik-Genetik Durum Makinesi kütüphanesidir.

Geleneksel kontrol sistemlerindeki katı, kuralcı (if/else tabanlı) ve öngörülemeyen çevre koşullarında kolayca çuvallayan geçiş mekanizmalarının yerine; durumlar arası geçişleri **olasılıksal (stokastik) matrislerle** yönetir ve bu kuralları **genetik algoritmalar** kullanarak simülasyon ortamında kendi kendine evrimleştirir.

---

## Mimari Yapı

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

## Örnek Kullanım (Study Case: Drone Navigation)

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

## Kurulum ve Derleme

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
