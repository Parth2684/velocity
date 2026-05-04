pkgname=velocity
pkgver=0.1.0
pkgrel=1
pkgdesc="Fastest File / Folder Sharing App"
arch=('x86_64')

depends=(
  'webkit2gtk-4.1'
  'xdotool'
  'libappindicator'
)

options=(!debug)

source=("Velocity::file://$(pwd)/src-tauri/target/release/Velocity" "velocity.desktop")

sha256sums=('SKIP' 'SKIP')

package() {
    install -Dm755 "$srcdir/Velocity" "$pkgdir/usr/bin/velocity"
    install -Dm644 "$srcdir/velocity.desktop" \
            "$pkgdir/usr/share/applications/velocity.desktop"
}
