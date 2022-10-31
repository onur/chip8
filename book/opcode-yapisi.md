# Opcode Yapısı

CHIP-8'in tüm OPCODE'ları 16-bit uzunluğundadır. Bu nedenle çok basit bir
bir OPCODE yapısı tanımlayalım:

```rust,no_run,noplaypen
{{#include ../src/instruction.rs:4:4}}
```

Rust'ta yapılar `struct` anahtar kelimesiyle tanımlanır. Struct'lar:

* `Opcode` yapımızda kullandığımız gibi bir isim sahibi tuple olabilir.
* C dilindeki gibi elemanlar içerebilir.
* Hiç bir eleman içermeden unit biçiminde olabilir.

Biz sadece 16-bitlik bir sayı üzerine eklemeler yapacağımız için, tuple
cinsinden struct kullanıyoruz.

Yapımızın başına gelen `pub` anahtar kelimesi, yapıyı açık hale getirerek
şu an kullandığımız modül dışında da erişimini mümkün kılmaktadır. `pub`
anahtar kelimesi yapılar dışında; enumlar, metodlar, fonksiyonlar ve `type` ile
kendi tanımladığınız tipler için de kullanılabilir.

Rust'ta yer alan primitif tipler (yapımızda kullandığımız
u16), tek haneli bir önek ardından gelen boyut ile tanımlanır. Yani
`u8`: 8-bit boyutunda bir unsigned (sadece pozitif sayıları tutabilen) bir
sayı tipidir. `u16` tahmin edilebileceği üzere 16-bit boyutunda bir
unsigned sayı tipidir. Dilde yer alan diğer primitif tipleri görmek için,
Rust kitabında yer alan
[primitif tipler](https://doc.rust-lang.org/std/index.html#primitives)
bölümüne başvurulabilir.

16-bitlik bu OPCODE'un her bir basamağı (nibble) farklı anlamlara
gelmektedir. Örneğin: `0x71AA` OPCODE'u; 1. registerdeki değere `0xAA`
sayısını eklemeye yarar. Bu nedenle her bir basamağı alabilmek için, Opcode
yapımıza yardımcı metodları ekleyelim:


```rust,no_run,noplaypen
{{#include ../src/instruction.rs:6:31}}
```

Rust'ta yapılara eklenecek metodlar `impl` bloğu içinde yer alır. Aynı
zamanda metodların ilk parametresi: `&self` bu fonksiyonların sadece
oluşturulan bir `Opcode` instance'ı ile çalışacağını belirtir.

Methodlar fonksiyonlar gibidir ve `fn` anahtar kelimesi kullanılarak
tanımlanır. Parametreler, dönüş değeri ve çağrıldığında çalıştırılacak kod
bloğuna sahiptirler. Methodların fonksiyonlardan farkı ise, bir yapı
(struct veya enum) için tanımlanırlar ve ilk parametre o yapıya ait
instance'a ulaşabileceğimiz `self` anahtar kelimesidir. Eğer metod
çalıştırıldığında, instance sahipliğini almak istemiyorsak, `self`
parametresi başına `&` koyularak referans olarak eklenir. Instance'ı referans
olarak sunmayıp sahipliğini alsaydık, bu metodların kullanımının ardından
instance sahipliği kaybolacaktı.

Metod adı ve parametrelerinin ardından döneceği tip `->` dan sonra yazılır.

Yapımız tuple cinsinden bir struct olduğu için, yapı içerisindeki veriye
aynı tuplelarda olduğu gibi `self.0` ile ulaşılır.

Rust'ta primitif tipler (u8, u16 vb.) `as` anahtar kelimesiyle birbirine
dönüştürüleblir. Fakat unutmayın ki boyutu büyük bir sayı (`u16`), `as` anahtar
kelimesiyle daha küçük bir sayıya dönüştürülürken hiç bir uyarı vermeden sayıyı
kırpabilir. Bu nedenle eğer büyük bir sayıyı küçük bir sayıya dönüştürmek
istiyorsanız `u8::from()` metodunu kullanın. Biz geliştirdiğimiz uygulamada
dönüştürülen sayıların 16-bit'ten küçük olduğunu bildiğimiz için `as` anahtar
kelimesini kullanacağız.

Metod isimleri ilk bakışta garip gelebilir fakat, CHIP-8 OPCODE'larında her
zaman soldan ikinci basamak `x`: ilk register numarasını, üçüncü basamak
ise `y`: ikinci register numarasını içermektedir. İleride array cinsinden
olan registerlarda daha rahat çalışabilmek için `x` ve `y` registerlarını
dönen `oxoo`, `ooyo` metodları `usize` dönmektedir (Rust'ta array indisleri
`usize` cinsinden olmalıdır).

Yapılan bitwise işlemleri açıklamak gerekirse, tekrar `0x71AA` örneğini
ele alalım. `0x71AA` sayısında ikinci basamakta yer alan `1` değerini almak
için öncelikle `oxoo` metodunu kullanmamız gerekli. Bu metod şu işlemleri
yapmaktadır:

```
0x71AA     111000110101010  AND
0x0F00     000111100000000
0x0100     000000100000000  >> 8
0x0001     000000000000001  # 16-bit uzunluğundaki sayı, CHIP-8'de sadece 16
--------------------------  # register bulunduğundan 8-bit'e dönüştürülür:
0x01              00000001
```

Ardından `0xAA` sayısı alınabilmesi için `oonn` metodu kullanılır:

```
0x71AA     111000110101010  AND
0x00FF     000000011111111
0x00AA     000000010101010  # Son iki basamak sadece 8-bit uzunluğunda
                            # olabileceğinden çıkan sonuç 8-bit'e dönüştürülür
--------------------------
0xAA              10101010
```

## Sayıların OPCODE'a Dönüştürülmesi

İleride işimize yarayacağı için 16-bitlik bir sayının (`u16`), `Opcode`
yapısına kolayca dönüştürülmesini sağlayan
[`From`](https://doc.rust-lang.org/std/convert/trait.From.html) özelliğini
eklememiz gerekli. Rust'ta yapılara yeni özellikle şu şekilde eklenir:

```rust,no_run,noplaypen
{{#include ../src/instruction.rs:33:37}}
```

`From` özelliği aynı zamanda `Into` özelliğini de otomatikmen eklediğinden,
`Opcode` kullanmamız gereken parametrelerde `Into<Opcode>` genelleyici tipini
kullanmamız yeterli olacak. Bu sayede `Opcode` hem verbose olarak
`Opcode(0xF1AA)` hem de `into` metoduyla direkt bir sayıdan dönüştürülebilir.

Genelleyiciler (generics) ve özellikler (traits) çok kapsamlı bir konudur ve
Rust'ın en önemli bileşenlerindendir. Şimdilik kafanızı çok fazla
karıştırmanıza gerek yok fakat isterseniz Rust kitabında yer alan:
[Özellikler](https://doc.rust-lang.org/rust-by-example/generics/gen_trait.html)
bölümünü okuyabilirsiniz.
