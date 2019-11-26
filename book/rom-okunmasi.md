# ROM Okunması

Herhangi bir işlem yapmadan önce emülatör belleğimizde bir CHIP-8 programı
yüklememiz gerekli. Sonuçta emulatörümüz bu CHIP-8 programını çalıştırmaya
yarayacak. Bu işlemi gerçekleştirmek için önce bir dosya açmalı, bu dosya
içerisindeki byteları `Emulator` belleği'ne (memory alanı) yazmamız
gerekli. Öncelikle bu işlemi yapmak için gerekli tipleri modül içinde
kullanalım:

```rust,no_run,noplaypen
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
```

Rust'ta başka bir paket ya da standart kütüphaneden herhangi bir yapı
kullanabilmek için `use` anahtar kelimesi kullanılır. İlk satırda
kullanılan use, `std::fs` modülü içerisinde yer alan `File` yapısını
üzerinde çalıştığımız modül içinde kullanmaya yarar. Bu sayede `File`
yapısında direkt ulaşabiliriz. İkinci satırda `std::io` modülünü kullanıyoruz.
Bu kullanımın ardından modül içerisinde yazacağımız her `io`,
standart kütüphanede yer alan `io` modülünü simgelemektedir.
`Read` özelliği, `File` üzerinde okuma yapmamız için gerekli.Son olarak dosya
yolu değişkeninde kullanacağımız `Path` yapısını kullanılıyoruz.

ROM okuma metodumuzu `impl Emulator` bloğu içerisine tanımlıyoruz:

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:67:73}}
```

`rom_oku` isimli bu metod, `Emulator` instance'ının bir parçası olduğundan
ilk parametresi: `&mut self`'dir. Instance'ın sahipliğini, bu fonksiyona
almamak için, `self` referans olarak kullanılır. Eğer sadece `self` olarak
kullansaydık, instance sahipliği bu fonksiyonun çağrılmasıyla, bu
fonksiyona geçecekti ve bir daha dışarıdan erişime izin verilmeyecekti. Bu
referansın mutable olmasının nedeni ise emulatör içerisinde yer alan bellek
(memory) alanını değiştiriyor olmamızdan kaynaklanıyor. Eğer bu referans
mutable olmasaydı, `Emulator` yapımızda yer alan hiç bir alanı
değiştiremezdik. `path` değişkeni Rust'ta referansdan-referansa dönüşüm
yapmayı mümkün kılan
[`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
özelliğine sahip bir genelleyici seçilmiştir. Bu sayede `path` değişkeni
`AsRef<Path>` özelliğine sahip herhangi bir tip olabilir (String, &str,
OsStr, Path, PathBuf vb.).

Metodumuzda hata yönetimi yapmak için; fonksiyon tanımlamamızda dönüş
değeri, yine standart kütüphanenin io (girdi-çıktı) modülünde yer alan
`Result` tipini kullanıyoruz. Bizim kullandığımız `io::Result` tipi;
`std::result::Result` tipiyle karıştırılmamalıdır. Normal `Result`
tanımlaması, hem dönüş tipi hem de hata tipi gerektirir. Bizim bu metodda
kullanacağımız tüm hata dönebilecek fonksiyonlar `io::Error` tipinde
olduğundan, `io::Result` tipini kullanmamız yeterli. `io::Result` sadece
dönüş için gerekli bir tip gerektirdiğinden ve biz bu metoddan 
`self` ile sahipliğini aldığımız `Emulator` instance'ını döneceğimizden,
dönüş tipi olarak `Emulator` kullanılır.

Dosya
[`let file = File::open(path)?;`](https://doc.rust-lang.org/std/fs/struct.File.html#metod.open)
ile açılıyor. Bu satırda yer alan `?` hata yönetimimiz için gerekli. Bu
fonksiyon normalde
[`io::Result`](https://doc.rust-lang.org/std/io/type.Result.html)
dönüyor ve bu `Result` tipi hata olması
durumunda `Err` (bizim kullandığımız `Result` tipine göre `io::Error`),
olmaması halinde ise `Ok` değeri döner. `?` ile yaptığımız hata kontrolü
sayesinde, bu fonksiyonun `Err` dönmesi durumunda, operatörümüz o noktada
gelen hatayı dönmeye yarıyor. Rust'ın bu gelişmiş hata yönetimi sayesinde,
hata kontrolü için yazılması gereken kod çok daha azalıyor ve bu basit
kullanım Rust'ın hata yönetimini çok daha güçlü hale getiriyor.

[`Read::bytes()`](https://doc.rust-lang.org/std/io/trait.Read.html#metod.bytes)
metodu, okunabilir instance'ı byteları iteratöre çevirmeye yarıyor. For
ile kullandığımız bu iterator hata olması durumunda yine `io::Result` dönen
bir yapıya sahip. Bu nedenle direkt byte'ı kullanmadan önce hata kontrolü
operatörümüz olan `?`'ini kullanıyoruz. CHIP-8'de programlar bellekte
`0x200` adresinden başladığından dolayı, belleğin `0x200` alanından itibaren
yazıyoruz.

İlk başta tanımladığımız gibi metodumuz `io::Result<Emulator>` dönmesi
gerekli. Fonksiyonumuz son satıra ulaştığında herhangi bir hata olmaması
durumunda, sahipliğini aldığımız instance ile birlikte  `Ok(self)` dönüyor.
Bu sayede bu metodu kullanırken sahipliğini aldığımız instance'ı bir zincir
halinde kullanabiliriz.
