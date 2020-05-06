#[derive(Copy, Clone)]
pub struct IrSlot(u32);

impl IrSlot {
    pub fn opcode(&self) -> IrOpcode {
        IrOpcode::from_primitive(((self.0 >> 24) & 0x000000FF) as u8)
    }

    pub fn operand8(&self) -> u8 {
        ((self.0 >> 16) & 0x000000FF) as u8
    }

    pub fn operand16(&self) -> u16 {
        ((self.0 >> 0) & 0x0000FFFF) as u16
    }
}

impl Default for IrSlot {
    fn default() -> Self {
        Self(0)
    }
}

pub struct IrSlotBuilder(u32);

impl IrSlotBuilder {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn opcode(&mut self, value: IrOpcode) -> &mut Self {
        self.0 &= 0x00FFFFFF;
        self.0 |= (value as u32) << 24;
        self
    }

    pub fn operand8(&mut self, value: u8) -> &mut Self {
        self.0 &= 0xFF00FFFF;
        self.0 |= (value as u32) << 16;
        self
    }

    pub fn operand16(&mut self, value: u16) -> &mut Self {
        self.0 &= 0xFFFF0000;
        self.0 |= value as u32;
        self
    }

    pub fn build(&self) -> IrSlot {
        IrSlot(self.0)
    }
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum IrOpcode {
    Nop,
    Ldsptl,
    Ldspth,
    Addca,
    Subca,
    Izsna,
    Dzsna,
    Pcadda,
    Nota,
    Nega,
    Sra,
    Sla,
    Srca,
    Slca,
    Swapa,
    Wdreset,
    Pushaf,
    Popaf,
    Reset,
    Stopsys,
    Stopexe,
    Engint,
    Disgint,
    Ret,
    Reti,
    Mul,
    Xorioa,
    Movioa,
    Movaio,
    Stt16,
    Ldt16,
    Idxmma,
    Idxmam,
    Retk,
    T0snm,
    T1snm,
    Set0m,
    Set1m,
    Addma,
    Subma,
    Addcma,
    Subcma,
    Andma,
    Orma,
    Xorma,
    Movma,
    Addam,
    Subam,
    Addcam,
    Subcam,
    Andam,
    Oram,
    Xoram,
    Movam,
    Addcm,
    Subcm,
    Izsnm,
    Dzsnm,
    Incm,
    Decm,
    Clearm,
    Xchm,
    Notm,
    Negm,
    Srm,
    Slm,
    Srcm,
    Slcm,
    Ceqsnam,
    T0snio,
    T1snio,
    Set0io,
    Set1io,
    Addak,
    Subak,
    Ceqsnak,
    Andak,
    Orak,
    Xorak,
    Movak,
    Goto,
    Call,
}

impl IrOpcode {
    // Function is perfectly safe as long as IrOpcode is non-exhaustive
    fn from_primitive(value: u8) -> IrOpcode {
        unsafe { core::mem::transmute(value) }
    }
}
