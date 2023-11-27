#[doc = "Register `SPI_DIN_NUM` reader"]
pub type R = crate::R<SPI_DIN_NUM_SPEC>;
#[doc = "Register `SPI_DIN_NUM` writer"]
pub type W = crate::W<SPI_DIN_NUM_SPEC>;
#[doc = "Field `SPI_DIN0_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN0_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN0_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN0_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN1_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN1_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN1_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN1_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN2_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN2_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN2_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN2_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_DIN3_NUM` reader - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN3_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_DIN3_NUM` writer - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type SPI_DIN3_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din0_num(&self) -> SPI_DIN0_NUM_R {
        SPI_DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din1_num(&self) -> SPI_DIN1_NUM_R {
        SPI_DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din2_num(&self) -> SPI_DIN2_NUM_R {
        SPI_DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_din3_num(&self) -> SPI_DIN3_NUM_R {
        SPI_DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DIN_NUM")
            .field(
                "spi_din0_num",
                &format_args!("{}", self.spi_din0_num().bits()),
            )
            .field(
                "spi_din1_num",
                &format_args!("{}", self.spi_din1_num().bits()),
            )
            .field(
                "spi_din2_num",
                &format_args!("{}", self.spi_din2_num().bits()),
            )
            .field(
                "spi_din3_num",
                &format_args!("{}", self.spi_din3_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_DIN_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_din0_num(&mut self) -> SPI_DIN0_NUM_W<SPI_DIN_NUM_SPEC> {
        SPI_DIN0_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_din1_num(&mut self) -> SPI_DIN1_NUM_W<SPI_DIN_NUM_SPEC> {
        SPI_DIN1_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_din2_num(&mut self) -> SPI_DIN2_NUM_W<SPI_DIN_NUM_SPEC> {
        SPI_DIN2_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by SPI module clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_din3_num(&mut self) -> SPI_DIN3_NUM_W<SPI_DIN_NUM_SPEC> {
        SPI_DIN3_NUM_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI input delay number configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_din_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_din_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DIN_NUM_SPEC;
impl crate::RegisterSpec for SPI_DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_din_num::R`](R) reader structure"]
impl crate::Readable for SPI_DIN_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_din_num::W`](W) writer structure"]
impl crate::Writable for SPI_DIN_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_DIN_NUM to value 0"]
impl crate::Resettable for SPI_DIN_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
