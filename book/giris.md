# Rust ile CHIP-8 Emülatörü Geliştirme

Bu kitap Rust Programlama Dili ve farklı mimariler için emülatör yapımını
öğrenmek isteyenler için yazılmış eğitici bir belgedir. Yazı programlamayı
sıfırdan öğretmeyi amaçlamamaktadır ve okuyucuların daha
önce en az bir programlama dili bildiği varsayılmıştır. Aynı zamanda
emülatör geliştirirken lazım olan bir çok bitdüzeyi (bitwise) işlem
kullanılmıştır. Bitdüzeyi işlemler olabildiğince anlaşılır bir dilde
açıklanmasına rağmen, okuyucuların konu hakkında daha önce bilgili olması
anlamalarını daha da kolaylaştıracaktır.

## Neden CHIP-8 Emülatörü Yazmalıyım?

[CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) sadece 35 opcode'u bulunan
çok basit bir sistemdir. Aynı zamanda bir çok opcode modern CPU'larda da
kullanıldığından bu sistem için geliştirilecek bir emülatör projesi,
gerekse modern CPU'ların çalışma prensibini ve temelinde bir sistem
programlama dili olan Rust'ı öğrenmek için en iyi başlangıç
projelerinden biridir. Yıllardır Rust Programlama Dili'ni öğrenmek
isteyenlere bir CHIP-8 emülatörü yazarak başlamalarını tavsiye ederim.

## Yazar Hakkında

Rust Programlama Dili'ni stabil sürümü çıkmadan, beta sürecinden beri
gündelik yaşamımda kullanmaktayım. Rust için belgeleme aracı olan
[Docs.rs](https://docs.rs)'i geliştirdim ve Mozilla'nın Rust Programlama
Dili organizasyonunda geliştirmesine devam etmekteyim. Bana
ulaşabileceğiniz kanallar:

* GitHub: <https://github.com/onur>
* Twitter: <https://twitter.com/oasln>
* E-posta: <onur@onur.im>

## Kaynak Kodları

Bu kitapta kullanılan tüm kod örneklerine, projenin çalışabilir halinin
tamamına ve kitabın kaynak kodlarına GitHub üzerinden erişilebilirsiniz:
<https://github.com/onur/chip8>. Proje göndereceğiniz her türlü katkıya
açıktır.
