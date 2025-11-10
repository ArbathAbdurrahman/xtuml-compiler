# xtUML Compiler

Compiler berbasis **Rust** untuk menerjemahkan model **xtUML (Executable UML)** dari format **JSON** menjadi kode sumber dalam berbagai bahasa pemrograman seperti **Python**, **Java**, dan **Javascript**.

---

## Tujuan Proyek

Proyek ini bertujuan membangun **model-driven compiler** yang mampu:
1. Membaca model **xtUML** dalam bentuk **JSON**.  
2. Melakukan **analisis semantik** terhadap elemen model (kelas, state, event, dsb).  
3. Menghasilkan **kode sumber otomatis** dari model ke berbagai target bahasa.

Dengan pendekatan ini, pengembang dapat berfokus pada **desain model sistem**, bukan menulis kode secara manual.

---

## Struktur Direktori

```markdown

xtuml-compiler/
│
├── src/
│   ├── main.rs             → Entry point CLI (mengatur input & output)
│   ├── parser.rs           → Parser untuk membaca model xtUML (JSON → Struct)
│   ├── semantic.rs         → Analisis semantik & pembentukan AST
│   ├── generator/          → Modul generator kode
│   │   ├── python.rs       → Generator untuk bahasa Python
│   │   ├── java.rs         → Generator untuk bahasa Java
│   │   └── javascript.rs   → Generator untuk bahasa Javascript
│   └── templates/          → Template file untuk masing-masing bahasa target
│
├── models/
│   └── example_model.json  → Contoh model xtUML dalam format JSON
│
└── output/
└── generated_code/         → Hasil kode sumber yang dihasilkan oleh compiler

````

---

## Cara Menjalankan

### 1. Clone repository
```bash
git clone https://github.com/ArbathAbdurrahman/xtuml-compiler.git
cd xtuml-compiler
````

### 2. Build project

Pastikan kamu sudah menginstal **Rust** dan **Cargo**.
Kemudian jalankan:

```bash
cargo build
```

### 3. Jalankan compiler

Contoh menjalankan compiler untuk model contoh:

```bash
cargo run -- models/example_model.json --lang java
```

Atau untuk bahasa lain:

```bash
cargo run -- models/example_model.json --lang python
cargo run -- models/example_model.json --lang javascript
```

Hasil output akan muncul di folder:

```
output/generated_code/
```

---

## Konsep Dasar xtUML

`xtUML (Executable UML)` adalah pengembangan dari UML yang memungkinkan model sistem:

* Dapat **dijalankan atau disimulasikan**, bukan sekadar digambar.
* Dapat **diterjemahkan langsung menjadi kode sumber** melalui *model compiler*.

Compiler ini berperan sebagai *bridge* antara model xtUML (abstrak) dan kode implementasi (nyata).

---

## Teknologi yang Digunakan

| Komponen              | Deskripsi                                        |
| --------------------- | ------------------------------------------------ |
| **Rust**              | Bahasa utama untuk kecepatan dan keamanan memori |
| **Serde**             | Parsing dan serialisasi JSON                     |
| **Clap**              | Parser CLI untuk argumen `--lang` dan path model |
| **Tera / Handlebars** | Template engine untuk generator kode             |
| **Walkdir / fs**      | File system handling untuk output otomatis       |

---

## Tahapan Kompilasi

```
        JSON Model
            │
            ▼
       [Parser Layer]
    (serde_json → Struct)
            │
            ▼
   [Semantic Analyzer Layer]
  (validasi class, state, event)
            │
            ▼
     [Code Generator Layer]
 (C++ / Java / Python templates)
            │
            ▼
     generated_code/
```

---

## Contoh Format Model JSON

```json
{
  "classes": [
    {
      "name": "Order",
      "attributes": [
        {"name": "id", "type": "int"},
        {"name": "status", "type": "string"}
      ],
      "states": [
        {"name": "Pending"},
        {"name": "Confirmed"},
        {"name": "Shipped"}
      ]
    }
  ],
  "events": [
    {"name": "OrderConfirmed", "trigger": "Order", "action": "status = 'confirmed'"}
  ]
}
```

---

## Contoh Output (Java)

```java
public class Order {
    private int id;
    private String status;

    public void confirmOrder() {
        this.status = "confirmed";
    }
}
```

---

## Roadmap Pengembangan

| Tahap | Deskripsi                                 | Status                 |
|-------|-------------------------------------------|------------------------|
| 1     | Struktur dasar proyek & parser JSON       | Sedang dikerjakan      |
| 2     | Analisis semantik model                   | Sedang dikerjakan      |
| 3     | Generator kode Python / Java / Javascript | Dalam pengembangan     |
| 4     | Validasi event & state machine            | Rencana berikutnya     |
| 5     | Integrasi WebAssembly (xtUML web IDE)     | Rencana jangka panjang |

---

## Kontribusi

Kontribusi sangat terbuka!
Silakan buat **pull request** atau **issue** jika menemukan bug atau ide baru.

Langkah:

1. Fork repository
2. Buat branch fitur baru
3. Lakukan perubahan
4. Ajukan pull request

---

## 📜 Lisensi

Proyek ini dirilis di bawah lisensi **[MIT License](LICENSE)**  silakan gunakan, ubah, dan sebarkan dengan bebas selama mencantumkan atribusi.

---

> “Model once, generate everywhere.”