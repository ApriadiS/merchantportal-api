# --- Tahap 1: Builder (Menggunakan base Alpine untuk kompilasi) ---
FROM rust:1.85.1-alpine AS builder

# Install build-time dependencies untuk Alpine
# TAMBAHKAN build-base untuk C/C++ toolchain lengkap
RUN apk add --no-cache musl-dev openssl-dev pkgconf build-base perl make

WORKDIR /usr/src/app

# Optimasi layer cache: Salin dependensi dulu
COPY Cargo.toml Cargo.lock ./
# Buat dummy project untuk men-cache kompilasi dependensi
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Salin sisa kode dan build aplikasi utama
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release --target=x86_64-unknown-linux-musl

# --- Tahap 2: Runner (Menggunakan base Alpine yang super ringan) ---
FROM alpine:latest

# Wajib: Install run-time dependencies
RUN apk add --no-cache libcrypto3 libssl3 ca-certificates tzdata
# SOLUSI: Prioritaskan IPv4 untuk mengatasi masalah jaringan Docker/WSL
RUN echo "precedence ::ffff:0:0/96 100" >> /etc/gai.conf

WORKDIR /app
# Salin HANYA binary yang sudah jadi dari tahap builder
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/merchantportal-api .

RUN chmod +x ./merchantportal-api
EXPOSE 3000
CMD ["./merchantportal-api"]

