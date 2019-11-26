# Emulatör Yapısı

CHIP-8'in tüm sistem özelliklerini biliyoruz. Bunların hepsini
`emulator.rs` içerisinde yeni bir yapı içerisinde tanımlayalım,
bu sefer yapımızı tanımlarken içerisinde elemanları olan bir yapı
tanımlayacağız:

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:12:42}}
```

Rust static ve strong yazılımlı bir dil olduğundan, tüm alanların tipleri
de tanımlanmak zorundadır. Diğer dillerin aksine tip tanımlamaları sağ
tarafta yer alır.

## Yeni Bir Emulator Instance'ı Oluşturulması

Öncelikle `Emulator` yapımızı, varsayılan değerleriyle bir instance'ını
oluşturan yeni bir fonksiyon tanımlayalım.

```rust,no_run,noplaypen
impl Emulator {
{{#include ../src/emulator.rs:46:65}}
}
```

Bu kod bloğunu satır satır inceleyelim:

`impl` anahtar kelimesi ve yapı adıyla başlayan bu satır, parantezler
içerisinde yer alan fonksiyon ve method tanımlamalarının, bu yapı için
kullanılacağını belirtir. `impl` ile sadece kendi paketiniz içerisinde
oluşturduğunuz yapılar için fonksiyon ve methodlar ekleyebilirsiniz. Eğer
başka bir pakette veya standart kütüphanede yer alan başka bir yapıya yeni
bir fonksiyon eklemek isterseniz, yeni bir yapı oluşturmanız gerekmektedir.

Emulator yapımızın hiç bir alanı açık (public) olmadığından, bu yapının
instance'ını sadece bu yapı içinde tanımlanmış fonksiyon ve metodlardan
erişebilirsiniz. Bu modül dışında Emulator yapısını direkt `Emulator { ... }`
şeklinde oluşturmaya çalışırsanız, alanların public olmadığına dair hata
alırsınız.

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:47:58}}
```

Değişken tanımlamaları `let` anahtar kelimesi ile yapılır. Rust'ta her
değişken varsayılan olarak immutable (içeriği değiştirilemez) haldedir. Biz
fonksiyonumuzun devamında, emülatör belleğine font setini yükleyeceğimizden
içeriği değiştirilebilir bir değişkene ihtiyacımız var. Bu nedenle `let`
tanımlamamızın ardından `mut` anahtar kelimesini kullanıyoruz. `mut`
anahtar kelimesi o değişkeni mutable (içeriği değiştirilebilir) duruma
getirmektedir. CHIP-8 programları belleğin `0x200` alanından
başlayacağından, I ve PC registerları 0x200 yapılır. Onun dışında her alan
0 olacak şekilde bir Emulator instance'ı oluşturulur.

Şu an için `Display` ve `Keyboard` yapılarını görmezden gelin. Görüntü ve
klavye işlemlerinin yapıldığı bu yapılar ileride anlatılacaktır.

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:60:62}}
```

CHIP-8: 0 dan F'e olmak üzere 5 byte uzunluğunda sprite gösterme özelliğine
sahiptir (16\*5=80). Bu font bilgisi daha önce tanımlanmış bir değişken
üzerinden emülatör belleğine yüklenir. `FONTSET` array'ının içeriğini
[kaynak kodlarında](https://github.com/onur/chip8/blob/master/src/emulator.rs#L302-L308)
görebilirsiniz. Rust'ta arraylar
[`Iterator`](https://doc.rust-lang.org/std/iter/index.html#iterator)
özelliğine sahip olduğundan `iter()` metodu yardımıyla kolayca
iterator'e dönüştürülebilir. `enumerate` metodu ise aynı Python'da olduğu
gibi item ile birlikte iterasyon sayacını döner. Bu metod sayaç ve item
değerlerini dönen bir tuple olduğundan, for döngüsü tuple değişken
tanımlamalarıyla kullanılır.

Son olarak fonksiyonumuz içerisinde yer alan `emulator` satırı,
fonksiyonumuzdan `emulator` değişkenimizin dönmesine yarar.
