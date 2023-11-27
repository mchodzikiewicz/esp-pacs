#[doc = "Register `AHB_CONFIG` reader"]
pub type R = crate::R<AHB_CONFIG_SPEC>;
#[doc = "Register `AHB_CONFIG` writer"]
pub type W = crate::W<AHB_CONFIG_SPEC>;
#[doc = "Field `HBURST` reader - set hburst"]
pub type HBURST_R = crate::FieldReader;
#[doc = "Field `HBURST` writer - set hburst"]
pub type HBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MAX_INCR` reader - set max continuous access for incr mode"]
pub type MAX_INCR_R = crate::FieldReader;
#[doc = "Field `MAX_INCR` writer - set max continuous access for incr mode"]
pub type MAX_INCR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - set hburst"]
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - set max continuous access for incr mode"]
    #[inline(always)]
    pub fn max_incr(&self) -> MAX_INCR_R {
        MAX_INCR_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_CONFIG")
            .field("hburst", &format_args!("{}", self.hburst().bits()))
            .field("max_incr", &format_args!("{}", self.max_incr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHB_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - set hburst"]
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<AHB_CONFIG_SPEC> {
        HBURST_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - set max continuous access for incr mode"]
    #[inline(always)]
    #[must_use]
    pub fn max_incr(&mut self) -> MAX_INCR_W<AHB_CONFIG_SPEC> {
        MAX_INCR_W::new(self, 3)
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
#[doc = "AHB config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_CONFIG_SPEC;
impl crate::RegisterSpec for AHB_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_config::R`](R) reader structure"]
impl crate::Readable for AHB_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_config::W`](W) writer structure"]
impl crate::Writable for AHB_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_CONFIG to value 0"]
impl crate::Resettable for AHB_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
