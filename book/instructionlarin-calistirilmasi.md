# Instruction'ların Çalıştırılması

Artık bellekten instructionları okuyup kendi tipimiz olan `Instruction`'a
dönüştürdüğümüze göre, çalıştırma aşamasına geçebiliriz. Instructionların
tam olarak ne yaptıklarını ayrıntılı olarak
[Instruction Yapısı](./instruction-yapisi.md)'nda tanımlamıştık. Şimdi bu
`enum`'un tüm bileşenlerini işleyeceğimiz bir `match` deyimi tanımlayalım.

Her bir instruction program counter'ı (`self.pc`) değiştireceğinden,
`match` deyimimiz, her bir işlemden sonra program counter'ın yeni değerini
dönüyor.

Bunun dışında Rust'a ait range operatörünü bir çok yerde kullanıyoruz.
Farklı bir syntax olarak `0..=x` gözünüze çarpabilir. Bu range operatörü *0
dan x + 1'e kadar* sayıları oluşturmaya yarıyor.

CHIP-8'de her bir instruction 2 byte olduğundan, bir çok instruction
program counter'ı 2 byte arttırıyor. Bir sonraki instruction'ı atlamamız
gereken durumlarda ise program counter, 4 byte arttırılıyor.

Bunun dışında overflow olabilecek toplama ve çıkarma işlemlerinde, `+` veya
`-` operatörlerini direkt kullanmak yerine,
[`overflow_add`](https://doc.rust-lang.org/std/primitive.u8.html#method.overflowing_add)
ve
[`overflow_sub`](https://doc.rust-lang.org/std/primitive.u8.html#method.overflowing_sub)
metodlarını kullanacağız. Rust
güvenli bir dil olduğundan overflow durumlarında panikleyerek çıkar.
`overflow_add` ve `overflow_sub` metodları istediğimiz gibi
toplama ve çıkarma işlemini yaptıktan sonra, overflow olursa bu durumu da
dönüyorlar. Zaten CHIP-8'de overflow durumları da `F` registerinde
saklandığı için, bu durumu kolayca `F` registerine atayabiliriz.

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:82:268}}
```


