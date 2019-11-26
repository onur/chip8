# Instruction'ların Alınması


ROM'umuzu emülatör belleğine yüklediğimize göre, artık OPCODE'ları okumaya
ve onları çalıştırmaya başlayabiliriz.

CHIP-8 OPCODE'ları 2 byte uzunluğundadır. Belleğimizde her bir alan 1 byte
uzunluğunda olduğundan, 2 byte okuyup bunları birleştirmemiz gerekli.
Örnek olarak bellek üzerinde şu iki byte düşünüldüğünde:

```plain
memory[pc]     == 0xA2
memory[pc + 1] == 0xF0
```

Bu iki byte'ı 16-bit'lik bir sayı yapmak için, öncelikle `0xA2` değeri
16-bitlik bir sayıya çevrilir ve ardından 8-bit sol tarafa kaydırılır (left
shift). Ardından bitwise OR ile bir sonraki byte yeni değere eklenir:

```plain
0xA2            10100010           // 8-bitlik değer 16-bit'e çevrilir
0x00A2  0000000010100010 << 8      // Ardından 8-bit sola kaydırılır
0xA200  1010001000000000 | 0x00F0  // Ardından sonraki byte (0xF0) XOR ile eklenir
0xA2F0  1010001011110000
```

Bu işlemi Rust ile `impl Emulator` bloğu içine tanımladığımız metodla
şu şekilde yapabiliriz:

```rust,no_run,noplaypen
{{#include ../src/emulator.rs:76:80}}
```

Bu şekilde program counter'da tutulan bellek alanından 2 byte'lık OPCODE okunur
bitwise OR işlemi ile birbirine eklenir ve tanımladığımız `Instruction` tipine
dönüştürülür. Daha önce `Instruction`'a eklediğimiz `From<u16>` özelliği
sayesinde, `opcode` değişkenine herhangi bir işlem yapmadan `Instruction::new`
metodunda kullanabiliriz.
