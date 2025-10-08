# --- Tahap 1: Builder (Untuk Kompilasi) ---
# Kita pakai image Rust resmi untuk membangun aplikasi
FROM rust:latest as builder

# Buat direktori kerja di dalam kontainer
WORKDIR /usr/src/app

# Salin file dependensi terlebih dahulu untuk memanfaatkan cache Docker
COPY Cargo.toml Cargo.lock ./

# Trik untuk men-download dan build dependensi saja
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release

# Sekarang salin semua kode sumber aplikasimu
COPY src ./src

# Hapus file dummy dan build aplikasi utamamu dalam mode rilis (performa maksimal)
RUN rm -f target/release/deps/merchantportal_api*
RUN cargo build --release

# --- Tahap 2: Runner (Untuk Menjalankan) ---
# Kita pakai image Debian yang sangat kecil agar efisien
FROM debian:bullseye-slim

# Update package lists and upgrade to reduce vulnerabilities
RUN apt-get update && apt-get upgrade -y && apt-get clean

# Salin HANYA file hasil kompilasi dari tahap builder
# Pastikan 'merchantportal-api' sesuai dengan nama [package] di Cargo.toml
COPY --from=builder /usr/src/app/target/release/merchantportal-api .

RUN chmod +x ./merchantportal-api
# Beri tahu Docker bahwa aplikasi kita akan berjalan di port 3000
EXPOSE 3000

# Perintah default untuk menjalankan aplikasimu saat kontainer dimulai
CMD ["./merchantportal-api"]