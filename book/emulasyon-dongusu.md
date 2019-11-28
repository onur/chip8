# Emülasyon Döngüsü

Tüm bileşenleri yazdığımıza göre artık emülasyon döngüsünü kurabiliriz.
Öncelikle gerekli kullanımları yapalım:

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:5:6}}
```

CHIP-8 oyunları kendi işlemci hızında çalışabilecek şekilde
programlandığından, modern bir işlemcide çok hızlı çalışacaktır. Bu
problemi her bir instructiondan sonra belirli bir süre bekleyerek
çözebiliriz. Çalışan thread'ımızda hiç bir şey yapmadan bekleyebileceğimiz
`sleep` fonksiyonunu modülümüze ekledik.

Ardıdan emülasyon metodunu `impl Emulator` bloğu içerisine oluşturalım:

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:270:297}}
```

Döngümüz pencere açık olduğu süre boyunca çalışacak.

`Display` tipine eklemiş olduğumuz ve referanstan-referansa dönüştürme
işlemi yapan `AsMut` özelliği sayesinde, `self.display.as_mut()` diyerek,
pencere yapısına ulaşabiliyoruz. minifb'de yer alan
[`update()`](https://docs.rs/minifb/0.13.0/minifb/struct.Window.html#method.update)
metodu, çizim yapmasak bile basılan tuşları alabilmemiz için çalıştırmamız
gereken bir metod.

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:274:279}}
```

Burada penceremizden basılan tuşların bir listesini alıyoruz. Bu fonksiyon
`Option` dönen bir fonksiyon olduğundan, Rust'a ait `if let` sözdizimini
kullanabiliriz. Bu if bloğu sadece `self.display.as_mut().get_keys()`
metodu `Some` döndüğünde çalışır. Ardından klavye alanına basılan tuşu
aktarıyoruz. `get_keys()` metodunun boş dönmesi durumunda basılan tuşu
geri bırakmak için klavye yapımızda kullandığımız `release_key()` metodunun
çalıştırıyoruz.

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:281:282}}
```

Daha önce yazdığımız instructionların okunması ve çalıştırılması işlemi
burada gerçekleştiriliyor. `expect()` metodu `Option` dönen
`instruction_oku()` metodundan `None` dönülmesi durumunda, "Bilinmeyen
Instruction" hata mesajıyla birlikte panikleyip çıkmamızı sağlıyor.

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:284:295}}
```

Son olarak CHIP-8'in timerlarını birer sayı azaltıp, her instructiondan
sonra 5 mili saniye bekleyerek döngümüzü tamamlıyoruz. Herhangi bir ses
arabirimi kullanmadığımızdan, ses için şimdilik ekrana "BEEP!" mesajını
yazdırıyoruz. İsterseniz siz bir ses paketi kullanarak bilgisayardan ses
çıkmasını da sağlayabilirsiniz.

## `main()` fonksiyonu

Artik emülatörümüz tüm bileşenleriyle hazır. Tek yapmamız gereken program
başladığında çalışacak olan `main` fonksiyonunu tanımlamak. Bunun için
program çalıştığında, program argümanlarını almamıza yarayacak
`std::env::args` fonksiyonunu dahil edelim ve main fonksiyonumuzu yazalım:

```rust,no_run,noplaypen
{{#include ../src/main.rs:10:17}}
```

`args().nth(1).unwrap_or_else(|| "brix.ch8".to_string())` söz dizimi,
program argümanlarından ilkini almaya ve eğer herhangi bir argüman
girilmezse de varsayılan olarak `brix.ch8` stringini dönmeye yarıyor. Bu
sayede emülatörümüz istenildiği taktirde farklı bir ROM'u oynatabilir.
`args()` fonksiyonu `String` cinsinden değerler döndüğü için
`"brix.ch8"`'in de `String`'e dönüştürülmesi gerekli. Bu işlem için
`to_string()` metodunu kullandık.

Eğer hatırlarsanız `rom_oku()` metodunu yazarken instance sahipliğini
almıştık ve dönüş değeri olarak yine almış olduğumuz sahipliği dönmüştük.
Bu sayede `expect()` ile hatadan kurtarılan instance'a `emulate()` metodunu
zincir halinde kullanabiliyoruz.
