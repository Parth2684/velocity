pkgname=Velocity
pkgver=0.1.0
pkgrel=1
pkgdesc="Fastest File / Folder Sharing App"
arch=('x86_64')

depends=(
  'webkit2gtk-4.1'
  'xdotool'
  'libappindicator'
)

source=("velocity::file://$(pwd)/src-tauri/target/release/velocity")

sha256sums=('SKIP')

package() {
    install -Dm755 "$srcdir/velocity" "$pkgdir/usr/bin/velocity"
}
