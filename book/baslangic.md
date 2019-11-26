# Başlangıç

Projeye başlamadan önce Rust Programlama Dili derleyicisinin
bilgisayarınızda kurulu olması gerekmektedir. Rust derleyicisini işletim
sisteminize göre [Rust kurulumu](https://www.rust-lang.org/tools/install)
sayfasında anlatıldığı gibi kurabilirsiniz. Bu kurulum bilgisayarınıza Rust
Programlama Dili derleyicisi `rustc` ve paket yöneticisi `cargo`'yu
kuracaktır. Cargo, Rust paketlerinin bağımlılığı çözüp, indirip derlemeye
yarayan bir araçtır.

## Yeni Proje Oluşturması

Cargo aynı zamanda yeni bir proje oluşturmanızı da sağlar. Yeni bir Rust
projesi oluşturmak için komut satırından aşağıdaki komutları kullanın:

```sh
$ cargo new chip8
$ cd chip8
````

İkinci satırdaki komut, çalışma dizinimizi yeni projemizin içerisinde
almaktadır. Bu adımdan sonra `cargo build` ile projenizi derleyebilir,
`cargo run` ile çalıştırabilirsiniz. Yeni oluşturduğumuz projemizde `cargo
run` komutunu kullandığımızda karşımıza `Hello, world!` yazısı çıkacaktır:

```plain
$ cargo run
   Compiling chip8 v0.1.0 (/home/onur/chip8)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/chip8`
Hello, world!
```

Rust paketlerinin meta verisi `Cargo.toml` içerisinde tutulmaktadır. Bu
dosya içinde bağımlılıkları tanımlayabilirsiniz. Şu an için bizim bu dosya
üzerinde yapacağımız bir değişiklik yok, yine de ön bilgi edinmek için bu
dosyayı inceleyebilirsiniz. Cargo hakkında daha fazla bilgi almak için
[Cargo kitabını](https://doc.rust-lang.org/cargo/) okuyabilirsiniz.

Rust Programlama Dili'nde her bir proje aynı zamanda bir pakettir. Bir
paket; kütüphane veya çalıştırılabilir bir program olabilir. Eğer
paketinizin kaynak dizini (src) içerisinde `main.rs` bulunursa, paketiniz
çalıştırılabilir bir program olarak derlenir. `cargo new` ile oluşturulan
bir paket, varsayılan olarak çalıştırılabilir bir program olarak
oluşturulur. `main.rs` yerine `lib.rs` kullanan bir paket, kütüphane olarak
tasarlanmıştır. Kütüphaneler aynı zamanda çalıştırılabilir programlarda
barındırabilir. Biz bu projede çalıştırılabilir bir program yapacağımızdan,
Cargo'nun oluşturduğu `main.rs`'e sadık kalacağız.

## Bağımlılıkların Eklenmesi

Rust'ta paket bağımlılıkları `Cargo.toml` dosyası içerisinde `[dependencies]`
bölümüne eklenir. Bu bölüme eklenen bağımlılıkları cargo, otomatikmen Rust
paket deposu olan [crates.io](https://crates.io) dan indirir derler ve
uygulamızla birleştirir. Biz uygulamamızda sadece iki tane bağımlılık
kullanacağız. Bunları `[dependencies]` bölümüne ekleyelim:

```plain
[dependencies]
rand = "0.7"
minifb = "0.13"
```

`rand` paketi rastgele bir sayı üretmeye yarar, `minifb` paketi en yaygın
işletim sistemlerinde (Windows, OSX ve Linux) basit bir şekilde pencere
oluşturup cizim yapmaya yarar.

Biz projemizde Rust 2018 yayınını kullandığımızdan, paketimiz içine tekrar
`extern crate` tanımlaması yapmamıza gerek yok.
