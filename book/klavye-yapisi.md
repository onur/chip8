# Klayve Yapısı

CHIP-8 daha önce bahsettiğimiz gibi:

```plain
+---+---+---+---+
| 1 | 2 | 3 | C |
| 4 | 5 | 6 | D |
| 7 | 8 | 9 | E |
| A | 0 | B | F |
+---+---+---+---+
```

Şeklinde bir klavyeye sahip. Tuşları alabilmek için `minifb::Key` yapısını
kullanacağız. Modern bir bilgisayarda basılan tuşları, CHIP-8'in
anlayabileceği türe çevirmek için basit bir yapı tanımlayalım:

```rust,no_run,noplaypen
{{#include ../src/keyboard.rs:1:3}}
```

Yapımızda herhangi bir tuşun basılı olup olmadığını kontrol edebilmek için
`Option<u8>` kullandık. Bu değer bir tuş basılıysa `Some<u8>`, değilse
`None` değerlerini içerecek.

Bunun ardından klavye metodlarını ekleyelim:

```rust,no_run,noplaypen
{{#include ../src/keyboard.rs:5:43}}
```

`to_chip8_key` metodu, `Key` cinsinden alınan tuşu, CHIP-8'in
anlayabileceği türe çevirmeye yarıyor ve basılan tuş eğer CHIP-8 tuş
takımında yoksa, değeri `None` yapıyor.
