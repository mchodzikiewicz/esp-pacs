#[doc = "Register `EXT_WAKEUP_SEL` reader"]
pub type R = crate::R<EXT_WAKEUP_SEL_SPEC>;
#[doc = "Register `EXT_WAKEUP_SEL` writer"]
pub type W = crate::W<EXT_WAKEUP_SEL_SPEC>;
#[doc = "Field `EXT_WAKEUP_SEL` reader - need_des"]
pub type EXT_WAKEUP_SEL_R = crate::FieldReader<u32>;
#[doc = "Field `EXT_WAKEUP_SEL` writer - need_des"]
pub type EXT_WAKEUP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_sel(&self) -> EXT_WAKEUP_SEL_R {
        EXT_WAKEUP_SEL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_SEL")
            .field(
                "ext_wakeup_sel",
                &format_args!("{}", self.ext_wakeup_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_sel(&mut self) -> EXT_WAKEUP_SEL_W<EXT_WAKEUP_SEL_SPEC> {
        EXT_WAKEUP_SEL_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP_SEL_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_sel::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup_sel::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP_SEL to value 0"]
impl crate::Resettable for EXT_WAKEUP_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
