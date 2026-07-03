# Maintainer: Paul <paul.sinbud2004@gmail.com>
pkgname=ssh-proxy-gtk
pkgver=0.1.1
pkgrel=1
pkgdesc="SSH Port Forwarding GUI application with GTK4"
arch=('x86_64')
url="https://github.com/Paul-sinbud2004/ssh-proxy-gtk"
license=('MIT')
depends=('gtk4' 'sshpass' 'openssh')
makedepends=('cargo')
source=("https://github.com/Paul-sinbud2004/ssh-proxy-gtk/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
    cd "$pkgname-$pkgver"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname-$pkgver"
    export RUSTFLAGS="-C opt-level=2"
    cargo build --release --locked --target-dir=target
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/ssh_proxy_gtk" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}

check() {
    cd "$pkgname-$pkgver"
    cargo test --release --locked
}