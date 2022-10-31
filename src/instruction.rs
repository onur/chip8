pub type Address = u16;
pub type Register = usize;

pub struct Opcode(u16);

impl Opcode {
    /// OPCODE üzerinde 0x0X00 değerini döner
    fn oxoo(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize
    }

    /// OPCODE üzerinde 0x00Y0 değerini döner
    fn ooyo(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize
    }

    /// OPCODE üzerinde 0x000N değerini döner
    fn ooon(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    /// OPCODE üzerinde 0x00NN değerini döner
    fn oonn(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// OPCODE üzerinde 0x0NNN değerini döner
    fn onnn(&self) -> u16 {
        self.0 & 0x0FFF
    }
}

impl From<u16> for Opcode {
    fn from(opcode: u16) -> Opcode {
        Opcode(opcode)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    /// 0x00E0 CLS: Ekranı temizler
    ClearDisplay,

    /// 0x00EE RET: Alt programdan döner
    Return,

    /// 0x1nnn JP: `nnn` adresine zıplar
    Jump(Address),

    /// 0x2nnn CALL: `nnn` adresindeki alt programı çağırır
    Call(Address),

    /// 0x3xnn SE: Eğer `x` registeri `nn`'e eşitse bir sonraki instruction'ı atlar
    SkipIfEqualsByte(Register, u8),

    /// 0x4xnn SE: Eğer `x` registeri `nn`'e eşit değilse bir sonraki instruction'ı atlar
    SkipIfNotEqualsByte(Register, u8),

    /// 0x5xy0 SE: Eğer `x` registeri `y` registerine eşitse
    /// bir sonraki instruction'ı atlar
    SkipIfEqual(Register, Register),

    /// 0x6xnn LD: `x` registerinin değerini `nn` yapar.
    LoadByte(Register, u8),

    /// 0x7xnn ADD: `x` registerindeki değere `nn` ekler.
    AddByte(Register, u8),

    /// 0x8xy0 LD: `x` registerinin değerini `y` registerinin değerine eşitler.
    Move(Register, Register),

    /// 0x8xy1 OR: `x` registerinin değerini `y` registerinin değeriyle
    /// bitwise OR işlemi yapar.
    Or(Register, Register),

    /// 0x8xy2 AND: `x` registerinin değerini `y` registerinin değeriyle
    /// bitwise AND işlemi yapar.
    And(Register, Register),

    /// 0x8xy3 XOR: `x` registerinin değerini `y` registerinin değeriyle
    /// bitwise XOR işlemi yapar.
    Xor(Register, Register),

    /// 0x8xy4 ADD: `x` registerinin değerini `y` registerinin değeriyle toplar ve `x`
    /// registerinin değerini sonuca eşitler. Çıkan sonuç 8 bitten fazla ise, `F` (carry)
    /// registerinin değerini 1 yapar, değilse 0.
    Add(Register, Register),

    /// 0x8xy5 SUB: `x` registerinin değerini `y` registerinin değerinden çıkarır ve `x`
    /// registerinin değerini sonuca eşitler. Eğer `x` registerindei değer, `y`
    /// registerindeki değerden büyükse, `F` (carry) registerinin değerini 1 yapar,
    /// değilse 0.
    Sub(Register, Register),

    /// 0x8xy6 SHR: `x` registerindeki değeri bir bit sağa kaydırır.
    /// Eğer `x` registerinin son biti 1 ise `F` (carry) registerinin değerini 1 yapar
    /// değilse 0.
    ShiftRight(Register),

    /// 0x8xy7 SUB: `y` registerinin değerini `x` registerinin değerinden çıkarır ve `x`
    /// registerinin değerini sonuca eşitler. Eğer `x` registerindei değer, `y`
    /// registerindeki değerden büyükse, `F` (carry) registerinin değerini 1 yapar,
    /// değilse 0.
    ReverseSub(Register, Register),

    /// 0x8xyE SHR: `x` registerindeki değeri bir bit sola kaydırır.
    /// Eğer `x` registerinin son biti 1 ise `F` (carry) registerinin değerini 1 yapar
    /// değilse 0.
    ShiftLeft(Register),

    /// 0x9xy0 SE: Eğer `x` registeri `y` registerine eşit değilse
    /// bir sonraki instruction'ı atlar
    SkipIfNotEqual(Register, Register),

    /// 0xAnnn LD: `I` registerinin değerini `nnn` yapar.
    LoadI(Address),

    /// 0xBnnn JP: `nnn` ve `V0` registerinin toplamından çıkan sonuca zıplar.
    JumpPlusZero(Address),

    /// 0xCxnn RND: Rastgele üretilen 8 bitlik sayı `nn` ile AND işleminden sonra
    /// çıkan sonuç `x` registerine atanır.
    Random(Register, u8),

    /// 0xDxyn DRW: `x` ve `y` registerinden başayarak `n` adet byte sprite'ı ekranda
    /// gösterir. Çakışma (collision) durumu `F` registerinde tutulur.
    Draw(Register, Register, u8),

    /// 0xEx9E SKP: `x` registerinde yer alan tuş basılırsa
    /// bir sonraki instruction'ı atlar
    SkipIfPressed(Register),

    /// 0xExA1 SKP: `x` registerinde yer alan tuş basılı değilse
    /// bir sonraki instruction'ı atlar
    SkipIfNotPressed(Register),

    /// 0xFx07 LD: `x` registerinin değerini delay timer yapar.
    LoadDelayTimer(Register),

    /// 0xFx0A LD: Bir tuşa basılmasını bekler ve basılan tuşun değerini `x`
    /// registerine atar. Tuş basılana kadar tüm çalıştırma durur.
    WaitForKeyPress(Register),

    /// 0xFx15 LD: Delay timer'ı `x` registerindeki değer yapar.
    SetDelayTimer(Register),

    /// 0xFx18 LD: Sound timer'ı `x` registerindeki değer yapar.
    SetSoundTimer(Register),

    /// 0xFx1E ADD: `I` registerinin değerini `I` ve `x` registerinin toplamı yapar.
    AddToI(Register),

    /// 0xFx29 LD: `I` registerinin değerini `x` registerinde yer alan değerden
    /// gelen sprite yeri yapar.
    LoadSprite(Register),

    /// 0xFx33 LD: `x` registerinin BCD (Binary Coded Decimal) cinsinden değerini:
    /// `I`, `I + 1`, `I + 2` alanlarında saklar.
    BCDRepresentation(Register),

    /// 0xFx55 LD: `I` registerinde yer alan alandan itibaren
    /// `0` dan `x` registerine kadar olan değerleri belleğe kopyalar.
    StoreRegisters(Register),

    /// 0xFx65 LD: `I` registerinde yer alan alandan itibaren
    /// bellekte yer alan değerleri `0` dan `x` registerine kopyalar.
    LoadRegisters(Register),
}

impl Instruction {
    pub fn new<T: Into<Opcode>>(opcode: T) -> Option<Instruction> {
        let opcode = opcode.into();
        match opcode.0 & 0xF000 {
            0x0000 => match opcode.ooon() {
                0x0000 => Some(Instruction::ClearDisplay),
                0x000E => Some(Instruction::Return),
                _ => None,
            },
            0x1000 => Some(Instruction::Jump(opcode.onnn())),
            0x2000 => Some(Instruction::Call(opcode.onnn())),
            0x3000 => Some(Instruction::SkipIfEqualsByte(opcode.oxoo(), opcode.oonn())),
            0x4000 => Some(Instruction::SkipIfNotEqualsByte(
                opcode.oxoo(),
                opcode.oonn(),
            )),
            0x5000 => Some(Instruction::SkipIfEqual(opcode.oxoo(), opcode.ooyo())),
            0x6000 => Some(Instruction::LoadByte(opcode.oxoo(), opcode.oonn())),
            0x7000 => Some(Instruction::AddByte(opcode.oxoo(), opcode.oonn())),
            0x8000 => match opcode.ooon() {
                0x0000 => Some(Instruction::Move(opcode.oxoo(), opcode.ooyo())),
                0x0001 => Some(Instruction::Or(opcode.oxoo(), opcode.ooyo())),
                0x0002 => Some(Instruction::And(opcode.oxoo(), opcode.ooyo())),
                0x0003 => Some(Instruction::Xor(opcode.oxoo(), opcode.ooyo())),
                0x0004 => Some(Instruction::Add(opcode.oxoo(), opcode.ooyo())),
                0x0005 => Some(Instruction::Sub(opcode.oxoo(), opcode.ooyo())),
                0x0006 => Some(Instruction::ShiftRight(opcode.oxoo())),
                0x0007 => Some(Instruction::ReverseSub(opcode.oxoo(), opcode.ooyo())),
                0x000E => Some(Instruction::ShiftLeft(opcode.oxoo())),
                _ => None,
            },
            0x9000 => Some(Instruction::SkipIfNotEqual(opcode.oxoo(), opcode.ooyo())),
            0xA000 => Some(Instruction::LoadI(opcode.onnn())),
            0xB000 => Some(Instruction::JumpPlusZero(opcode.onnn())),
            0xC000 => Some(Instruction::Random(opcode.oxoo(), opcode.oonn())),
            0xD000 => Some(Instruction::Draw(
                opcode.oxoo(),
                opcode.ooyo(),
                opcode.ooon(),
            )),
            0xE000 => match opcode.oonn() {
                0x009E => Some(Instruction::SkipIfPressed(opcode.oxoo())),
                0x00A1 => Some(Instruction::SkipIfNotPressed(opcode.oxoo())),
                _ => None,
            },
            0xF000 => match opcode.oonn() {
                0x0007 => Some(Instruction::LoadDelayTimer(opcode.oxoo())),
                0x000A => Some(Instruction::WaitForKeyPress(opcode.oxoo())),
                0x0015 => Some(Instruction::SetDelayTimer(opcode.oxoo())),
                0x0018 => Some(Instruction::SetSoundTimer(opcode.oxoo())),
                0x001E => Some(Instruction::AddToI(opcode.oxoo())),
                0x0029 => Some(Instruction::LoadSprite(opcode.oxoo())),
                0x0033 => Some(Instruction::BCDRepresentation(opcode.oxoo())),
                0x0055 => Some(Instruction::StoreRegisters(opcode.oxoo())),
                0x0065 => Some(Instruction::LoadRegisters(opcode.oxoo())),
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction, Opcode};

    #[test]
    fn test_opcode() {
        let opcode = Opcode(0x1234);
        assert_eq!(opcode.oxoo(), 0x02);
        assert_eq!(opcode.ooyo(), 0x03);
        assert_eq!(opcode.ooon(), 0x04);
        assert_eq!(opcode.oonn(), 0x34);
        assert_eq!(opcode.onnn(), 0x234);
    }

    #[test]
    fn test_instruction() {
        assert_eq!(Instruction::new(0x00E0), Some(Instruction::ClearDisplay));
        assert_eq!(Instruction::new(0x00EE), Some(Instruction::Return));
        assert_eq!(Instruction::new(0x1AAA), Some(Instruction::Jump(0xAAA)));
        assert_eq!(Instruction::new(0x2AAA), Some(Instruction::Call(0xAAA)));
        assert_eq!(
            Instruction::new(0x31AA),
            Some(Instruction::SkipIfEqualsByte(0x01, 0xAA))
        );
        assert_eq!(
            Instruction::new(0x41AA),
            Some(Instruction::SkipIfNotEqualsByte(0x01, 0xAA))
        );
        assert_eq!(
            Instruction::new(0x5120),
            Some(Instruction::SkipIfEqual(0x01, 0x02))
        );
        assert_eq!(
            Instruction::new(0x61AA),
            Some(Instruction::LoadByte(0x01, 0xAA))
        );
        assert_eq!(
            Instruction::new(0x71AA),
            Some(Instruction::AddByte(0x01, 0xAA))
        );
        assert_eq!(
            Instruction::new(0x8120),
            Some(Instruction::Move(0x01, 0x02))
        );
        assert_eq!(Instruction::new(0x8121), Some(Instruction::Or(0x01, 0x02)));
        assert_eq!(Instruction::new(0x8122), Some(Instruction::And(0x01, 0x02)));
        assert_eq!(Instruction::new(0x8123), Some(Instruction::Xor(0x01, 0x02)));
        assert_eq!(Instruction::new(0x8124), Some(Instruction::Add(0x01, 0x02)));
        assert_eq!(Instruction::new(0x8125), Some(Instruction::Sub(0x01, 0x02)));
        assert_eq!(
            Instruction::new(0x8126),
            Some(Instruction::ShiftRight(0x01))
        );
        assert_eq!(
            Instruction::new(0x8127),
            Some(Instruction::ReverseSub(0x01, 0x02))
        );
        assert_eq!(Instruction::new(0x812E), Some(Instruction::ShiftLeft(0x01)));
        assert_eq!(
            Instruction::new(0x9120),
            Some(Instruction::SkipIfNotEqual(0x01, 0x02))
        );
        assert_eq!(Instruction::new(0xAAAA), Some(Instruction::LoadI(0xAAA)));
        assert_eq!(
            Instruction::new(0xBAAA),
            Some(Instruction::JumpPlusZero(0xAAA))
        );
        assert_eq!(
            Instruction::new(0xC1AA),
            Some(Instruction::Random(0x01, 0xAA))
        );
        assert_eq!(
            Instruction::new(0xD12A),
            Some(Instruction::Draw(0x01, 0x02, 0x0A))
        );
        assert_eq!(
            Instruction::new(0xE19E),
            Some(Instruction::SkipIfPressed(0x01))
        );
        assert_eq!(
            Instruction::new(0xE1A1),
            Some(Instruction::SkipIfNotPressed(0x01))
        );
        assert_eq!(
            Instruction::new(0xF107),
            Some(Instruction::LoadDelayTimer(0x01))
        );
        assert_eq!(
            Instruction::new(0xF10A),
            Some(Instruction::WaitForKeyPress(0x01))
        );
        assert_eq!(
            Instruction::new(0xF115),
            Some(Instruction::SetDelayTimer(0x01))
        );
        assert_eq!(
            Instruction::new(0xF118),
            Some(Instruction::SetSoundTimer(0x01))
        );
        assert_eq!(Instruction::new(0xF11E), Some(Instruction::AddToI(0x01)));
        assert_eq!(
            Instruction::new(0xF129),
            Some(Instruction::LoadSprite(0x01))
        );
        assert_eq!(
            Instruction::new(0xF133),
            Some(Instruction::BCDRepresentation(0x01))
        );
        assert_eq!(
            Instruction::new(0xF155),
            Some(Instruction::StoreRegisters(0x01))
        );
        assert_eq!(
            Instruction::new(0xF165),
            Some(Instruction::LoadRegisters(0x01))
        );
        assert_eq!(Instruction::new(0xFFFF), None);
    }
}
