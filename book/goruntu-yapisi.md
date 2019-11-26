# Görüntü Yapısı

Emülatörümüzün temel fonksiyonlarını tamamladıktan sonra, ekrana görüntü
vereceğimiz yapımızın tanımına geçebiliriz. Öncelikle kullanacağımız
yapıları modülümüze ekleyelim:

```rust,no_run,noplaypen
{{#include ../src/display.rs:1:1}}
```

[minifb](https://github.com/emoon/rust_minifb) daha önce de bahsettiğimiz
gibi en yaygın işletim sistemlerinde bir pencere açıp içerisine bir şeyler
çizebileceğimiz basit bir paket. Aynı zamanda basılan tuşları da
alabildiği için bizim için biçilmiş kaftan. Yapıları kullanıma aldıktan
sonra bazı sabitleri tanımlayalım:

```rust,no_run,noplaypen
{{#include ../src/display.rs:3:6}}
```

CHIP-8 64x32 piksel boyutunda bir ekrana sahip. Bu nedenle genişlik (`WIDTH`)
ve yükseklik (`HEIGHT`) olarak iki sabit tanımlaması yaptık. Ayrıca arka
plan rengi ve ön plan rengi olmak üzere iki 32-bit cinsinden sayı
tanımladık. minifb 32-bit cinsinden olan değerleri ekrana cizebilir,
isterseniz burada farklı renkler seçebilirsiniz.

Rust'ta sabit tanımlamaları `const` anahtar kelimesiyle yapılır. Tüm
sabitler `'static` ömrüne sahiptir.

Artık görüntü yapımızı tanımlayabiliriz:

```rust,no_run,noplaypen
{{#include ../src/display.rs:8:11}}
```

CHIP-8 siyah beyaz bir ekrana sahip olduğundan görüntü buffer'ı 8-bitlik
64x32 boyutunda bir matris. Bu buffer içerisinde dolu pikseller için 1, boş
pikseller için 0 değerini kullanacağız. `window` alanı için de minifb'nin
`Window` tipini kullanıyoruz.

### Görüntü Metodlarının Uygulanması

```rust,no_run,noplaypen
{{#include ../src/display.rs:13:13}}
}
```
Bloğunu oluşturarak görüntü metodlarını eklemeye başlayalım.

```rust,no_run,noplaypen
{{#include ../src/display.rs:14:28}}
```

`new` metodumuz yeni bir `Display` instance'ı oluşturmaya yarıyor. Burada
kullanmış olduğumuz `Self`; `Dislay` ile aynı anlama geliyor. İsterseniz
`impl` bloğunda tekrar tekrar tip ismini yazmak yerine `Self` anahtar
kelimesini kullanabilirsiniz. Unutmayın ki birinci harf büyük olmalı ve bu
anahtar kelime, metod argümanlarında kullanılan `self` ile
karıştırılmamalıdır.

[`WindowOptions`](https://docs.rs/minifb/0.13.0/minifb/struct.WindowOptions.html)
yapısı tamamen açık (public) alanlara sahip bir yapı. Bu nedenle bu yapı
için herhangi bir yardımcı metod olmadan direkt instance oluşturabiliriz.
Biz varsayılan ([`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html))
özelliği eklenmiş bu yapının, sadece `scale` alanını değiştireceğimizden,
geri kalan alanların varsayılan değeri alması için `..WindowOptions::default()`
yazımını kullandık. Rust'ta kullanılan bu yazım, yapının tanımlanmamış
diğer alanları için varsayılan değerleri almasını sağlıyor. 64x32 piksel
modern bir bilgisayarda çok küçük olacağı için, `scale` ile belirlediğimiz
değerle, minifb paketi görütüyü belirlenen oranda arttırıyor (bizim
belirlediğimiz X16 oranında).

Metodumuzda şu an için herhangi bir hata yönetimi yapmadığımızdan dolayı,
pencere oluşturulurken karşılaşacağımız olası bir hata durumunda `expect`
ile panikleyip çıkıyoruz.

### Yardımcı Metodlar

```rust,no_run,noplaypen
{{#include ../src/display.rs:30:36}}
```

`is_open` metodu adından da anlaşılabileceği gibi, penceremizin açık olup
olmadığını kontrol ediyor. Aynı zamanda `Esc` tuşunun basılı olup
olmadığını da kontrol eden bu metod `bool` dönüyor. Bu sayede oyuncu
istediği zaman `Esc` tuşuna basarak pencereyi kapatabilir ve emülatörü
sonlandırabilir.

`clear` metodu, yapımız içerisinde yer alan buffer'ı temizlemeye yarıyor.


### Draw Instruction'ı ile Çizim

CHIP-8 draw instruction'ı daha önce belirlediğimiz gibi:

```rust,no_run,noplaypen
{{#include ../src/instruction.rs:125:127}}
```

Şeklinde yapıyor. Aynı zamanda CHIP-8 sadece bu instruction ile collision
(spriteların birbiriyle çakışma) durumunu kontrol edebiliyor. Bu işlemi şu
şekilde yapabiliriz:

```rust,no_run,noplaypen
{{#include ../src/display.rs:38:59}}
```

`sprite` bellekten gelen 8-bitlik verilere sahip bir slice. Öncelikle
for döngümüz her bir sprite için `j` sayacıyla çalışıyor. Ardından ikinci
for döngüsünde 8-bitlik verinin her bir bitini kontrol etmeye başlıyoruz.
`xi` ve `yj` ile yazılacak bit'in ekrandaki pozisyonu belirleniyor.
Ardından her bir bit: `0x80: 10000000` üzerinde çalıştığımız bit kadar (`i`)
sağa kaydırılarak sprite ile XOR işlemi sayesinde herhangi bir çizilecek
piksel varsa 1, yoksa 0 sonucu elde ediliyor. Ardından daha önce aynı
pozisyonda herhangi sprite çizildiyse çakışma değeri olan `collision` 1
yapılıyor.

Son olarak CHIP-8'de spritelar ekrana XOR ile çizildiği için, üzerinde
çalıştığımız piksel 1 ile XOR işlemi ile belirleniyor.

### `buffer`'ın Pencereye Çizimi

minifb `&[u32]` tipinden bir yapıyı ekrana çizebilir. Bu nedenle
`[[0; WIDTH]; HEIGHT]` tipinden olan matrisimizi `[u32]` tipden bir array'a
çevirmemiz gerekli. Bu işlemi daha önce sabit olarak belirlediğimiz
arkaplan ve önplan rengini de kullanarak şu şekilde yapabiliriz:

```rust,no_run,noplaypen
{{#include ../src/display.rs:61:75}}
```

### Referanstan-referansa Dönüştürme ile Pencere Alanının Alımı

İleride yazacağımız emülasyon döngüsü için, `Display` yapımız içerisinde
yer alan `window` alanına ulaşabilmemiz gerekli. Bu işlemi Rust'ın
referanstan-referansa çevirme işlemi yapan
[`AsMut`](https://doc.rust-lang.org/std/convert/trait.AsMut.html)
özelliği ile yapacağız. `Display` yapımıza `AsMut` özelliğini katalım ve
`as_mut` çağrıldığında `Window` tipini dönelim:

```rust,no_run,noplaypen
{{#include ../src/display.rs:78:82}}
```
