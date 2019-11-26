# Instuction Yapısı

Bu bölümde Rust'ta `enum` cinsinden yeni bir yapı tanımlama şeklini
göreceğiz. Rust'ta `enum`'lar farklı biçimlerde yapılar içerebilir. Örneğin
bir `enum` içerisinde yer alan yapı: unit benzeri hiç bir eleman içermeyen,
`tuple` cinsinden ya da C benzeri bir `struct` olabilir. Her bir geçerli
`struct` yapısı, aynı zamanda geçerli bir `enum` biçimidir.

Rust'ın yine en güçlü özelliklerinden biri olan pattern matching, `enum`
yapısını kolay bir şekilde parçalayarak işlemeye yarar. Yazacağımız
emülatörün okunabilir olabilmesi için, 16-bitlik sayı olarak okuduğumuz
Opcode'u, `enum` ile okunabilir bir yapıya dönüştüreceğiz.

Bu işlemi yapmadan önce 16-bit uzunluğunda olan ve bellekte bir adresi
ifade eden `Address` tipimizi ve array indislerinde kolayca kullanmamızı
sağlayan `Register` tipimizi tanımlayalım:

```rust,no_run,noplaypen
{{#include ../src/instruction.rs:1:2}}
```

Rust'ta yeni tipler `type` anahtar kelimesiyle tanımlanır. Başka bir tipi
daha okunabilir bir hale getirmeye yarar. Bu sayede artık adres olduğunu
bildiğimiz alanlar için `u16` yerine `Address` tipini kullanabiliriz.

CHIP-8, 35 instructiona sahip bir yorumlayıcı. Bu OPCODE'ların hepsini bir
`enum` içerisinde şu şekilde tanımlayabiliriz:

```rust,no_run,noplaypen
{{#include ../src/instruction.rs:40:168}}
```

`enum` yapımızda içerisinde hiç bir veri tutmayan unit cinsinden ve veri
barındıran tuple cinsinden bileşenleri görebilirsiniz. `enum` elemanları
yukarıda da belirttiğimiz gibi bu iki türden de olabilir.


## OPCODE'un Instruction'a Dönüştürülmesi

Rust'ta `struct`'lara olduğu gibi, `enum`'lara da metod ekleyebilirsiniz.
Elimizde sayı halinde bulunan raw OPCODE'dan yeni bir `Instruction` instance'ı
oluşturmak için `new` metodunu ekleyelim. Rust'ta yeni bir instance
oluşturan metodlar genelde `new` ismiyle adlandırılır. Bu bir zorunluluk
değil, istediğiniz ismi koyabilirsiniz.

```rust,no_run,noplaypen
{{#include ../src/instruction.rs:170:230}}
```

Bu metodda Rust'a ait bir çok özellik bulunuyor. Öncelikle metod imzamızı
inceleyelim:


```rust,no_run,noplaypen
{{#include ../src/instruction.rs:171:171}}
```

`T` tahmin edebileceğiniz gibi bir genelleyici (generic). Genelleyici
tanımları metod adından sonra `<>` içerisinde yapılır. Genelleyicimiz
`Into<Opcode>` özelliğine sahip bir parametre anlamına gelmektedir.
Daha önce OPCODE'a
[eklediğimiz](opcode-yapisi.html#sayıların-opcodea-dönüştürülmesi)
`From` özelliği sayesinde, `opcode` isiminli parametremiz, `Opcode`'a
dönüştürülebilen herhangi bir tip olabilir. Bu sayede bu metodu istersek
16-bitlik bir sayı olarak da çalıştırabiliriz
(`Instruction::new(0xF155)` gibi). Opcode'a eklediğimiz `From` özelliği,
sayının otomatikmen `Opcode` tipine çevrilmesini sağlayacaktır.

Metodumuz aynı zamanda normal bir `Instruction` yerine `Option<Instruction>`
dönmekte. Rust'ta yer alan
[`Option`](https://doc.rust-lang.org/std/option/index.html) tipi;
opsiyonel bir değeri temsil etmektedir. Bu tip herhangi bir `Some`
ya da hiç bir `None` değer taşıyabilir. CHIP-8'de sadece 35 OPCODE
bulunduğundan genen raw OPCODE, `Instruction` tipine dönüştürülürken
bilinen OPCODE'lar için `Some(Instruction)`, bilinmeyenler için
hiç bir değeri olan `None` dönüyoruz.

Metodumuz içerisinde yer alan `let opcode = opcode.into();` satırı,
yukarıda bahsettiğimiz genelleyici ile gelen `Into<Opcode>` özelliğine
sahip `opcode` parametresini `Opcode`'a çevirmeye yarar. Aynı zamanda Rust
gölgelemeye de izin verdiğinden, `opcode` değişkeni bu satırdan sonra
`Opcode` tipine dönüşür.

Ardından gelen kod bloğunda raw OPCODE parçalanarak, okunabilir tipimiz
olan `Instruction`'a dönüştürülüyor. Bu işlemi yaparken yine Rust'ın yine
en önemli özelliklerinden biri olan [pattern
matching](https://doc.rust-lang.org/book/ch06-02-match.html) kullanıyoruz.
`match` C de yer alan switch-case'e çok benzemesine rağmen, `match` edilen
değerin tüm elemanlarını kapsamak zorundadır. Biz bu işlemi yaparken
16-bitlik bir sayı kullandığımızdan, işimize yarayan tüm değerleri aldık ve
geri kalan ve işimize yaramayanlar içinde `_` elemanını kullandık.

`match` bloğumuz `Option<Instruction>` döndüğü sürece,
iç içe istediğimiz kadar `match` kullanabiliriz. Bu nedenle önce en soldaki
nibble kontrol edildikten sonra, aynı nibble ile başlayan OPCODE'lar
tekrar `match` ile kontrol edildip, OPCODE'a uyan bir `Instruction`
tipi oluşturuluyor.

`Instruction` tipimizi içerisinde veri barındıran (daha önce tanımladığımız
`Address`, `Register` vb.) bileşenler barındırdığından, `Instruction` tipi
oluşturulurken bu bileşenlere gerekli değerler atanır. Bu işlemi yaparken
daha önce `Opcode` tipine eklediğimiz yardımcı fonksiyonları kullanıyoruz.

Rust'ta her satır aynı zamanda bir deyim olduğundan ve `match` bloğumuz da
aynı zamanda `Option<Instruction>` döndüğünden, `return` anahtar kelimesini
kullanmamıza gerek yok. Deyim olarak kullanılan bu match bloğunun sonunda
`;` olmamasına dikkat edin.

Son olarak Rust'ta yorum satırları `//` ile başlar. Rust içerisinde çok
gelişmiş bir belgeleme aracı (rustdoc) da bulundurmaktadır. Herhangi bir
tanımdan önce (bu bir metod, fonksiyon, struct, bileşen  ya da alan
olabilir); 3 slash ile (`///`) oluşturacağınız yorum satırı, belgeleme
aracı ile oluşturacağınız belgede o alan için tanımlama yapar. Kodlarken
yazabileceğiniz bu yorum satırları, aynı zamanda herhangi bir Rust
kütüphanesinin belgelemesini de çok kolay bir hale getirir.
