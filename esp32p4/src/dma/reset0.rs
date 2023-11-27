#[doc = "Register `RESET0` reader"]
pub type R = crate::R<RESET0_SPEC>;
#[doc = "Register `RESET0` writer"]
pub type W = crate::W<RESET0_SPEC>;
#[doc = "Field `DMAC_RST` reader - NA"]
pub type DMAC_RST_R = crate::BitReader;
#[doc = "Field `DMAC_RST` writer - NA"]
pub type DMAC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dmac_rst(&self) -> DMAC_RST_R {
        DMAC_RST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET0")
            .field("dmac_rst", &format_args!("{}", self.dmac_rst().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESET0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_rst(&mut self) -> DMAC_RST_W<RESET0_SPEC> {
        DMAC_RST_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET0_SPEC;
impl crate::RegisterSpec for RESET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset0::R`](R) reader structure"]
impl crate::Readable for RESET0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset0::W`](W) writer structure"]
impl crate::Writable for RESET0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET0 to value 0"]
impl crate::Resettable for RESET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
