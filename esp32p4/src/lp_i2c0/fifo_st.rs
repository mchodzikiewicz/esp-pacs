#[doc = "Register `FIFO_ST` reader"]
pub type R = crate::R<FIFO_ST_SPEC>;
#[doc = "Field `RXFIFO_RADDR` reader - Represents the offset address of the APB reading from RXFIFO"]
pub type RXFIFO_RADDR_R = crate::FieldReader;
#[doc = "Field `RXFIFO_WADDR` reader - Represents the offset address of i2c module receiving data and writing to RXFIFO."]
pub type RXFIFO_WADDR_R = crate::FieldReader;
#[doc = "Field `TXFIFO_RADDR` reader - Represents the offset address of i2c module reading from TXFIFO."]
pub type TXFIFO_RADDR_R = crate::FieldReader;
#[doc = "Field `TXFIFO_WADDR` reader - Represents the offset address of APB bus writing to TXFIFO."]
pub type TXFIFO_WADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents the offset address of the APB reading from RXFIFO"]
    #[inline(always)]
    pub fn rxfifo_raddr(&self) -> RXFIFO_RADDR_R {
        RXFIFO_RADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - Represents the offset address of i2c module receiving data and writing to RXFIFO."]
    #[inline(always)]
    pub fn rxfifo_waddr(&self) -> RXFIFO_WADDR_R {
        RXFIFO_WADDR_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Represents the offset address of i2c module reading from TXFIFO."]
    #[inline(always)]
    pub fn txfifo_raddr(&self) -> TXFIFO_RADDR_R {
        TXFIFO_RADDR_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - Represents the offset address of APB bus writing to TXFIFO."]
    #[inline(always)]
    pub fn txfifo_waddr(&self) -> TXFIFO_WADDR_R {
        TXFIFO_WADDR_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_ST")
            .field(
                "rxfifo_raddr",
                &format_args!("{}", self.rxfifo_raddr().bits()),
            )
            .field(
                "rxfifo_waddr",
                &format_args!("{}", self.rxfifo_waddr().bits()),
            )
            .field(
                "txfifo_raddr",
                &format_args!("{}", self.txfifo_raddr().bits()),
            )
            .field(
                "txfifo_waddr",
                &format_args!("{}", self.txfifo_waddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "FIFO status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_ST_SPEC;
impl crate::RegisterSpec for FIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_st::R`](R) reader structure"]
impl crate::Readable for FIFO_ST_SPEC {}
#[doc = "`reset()` method sets FIFO_ST to value 0"]
impl crate::Resettable for FIFO_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
