#[doc = "Register `EDPI_TE_HW_CFG` reader"]
pub type R = crate::R<EDPI_TE_HW_CFG_SPEC>;
#[doc = "Register `EDPI_TE_HW_CFG` writer"]
pub type W = crate::W<EDPI_TE_HW_CFG_SPEC>;
#[doc = "Field `HW_TEAR_EFFECT_ON` reader - NA"]
pub type HW_TEAR_EFFECT_ON_R = crate::BitReader;
#[doc = "Field `HW_TEAR_EFFECT_ON` writer - NA"]
pub type HW_TEAR_EFFECT_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW_TEAR_EFFECT_GEN` reader - NA"]
pub type HW_TEAR_EFFECT_GEN_R = crate::BitReader;
#[doc = "Field `HW_TEAR_EFFECT_GEN` writer - NA"]
pub type HW_TEAR_EFFECT_GEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW_SET_SCAN_LINE` reader - NA"]
pub type HW_SET_SCAN_LINE_R = crate::BitReader;
#[doc = "Field `HW_SET_SCAN_LINE` writer - NA"]
pub type HW_SET_SCAN_LINE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_LINE_PARAMETER` reader - NA"]
pub type SCAN_LINE_PARAMETER_R = crate::FieldReader<u16>;
#[doc = "Field `SCAN_LINE_PARAMETER` writer - NA"]
pub type SCAN_LINE_PARAMETER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn hw_tear_effect_on(&self) -> HW_TEAR_EFFECT_ON_R {
        HW_TEAR_EFFECT_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn hw_tear_effect_gen(&self) -> HW_TEAR_EFFECT_GEN_R {
        HW_TEAR_EFFECT_GEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn hw_set_scan_line(&self) -> HW_SET_SCAN_LINE_R {
        HW_SET_SCAN_LINE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:31 - NA"]
    #[inline(always)]
    pub fn scan_line_parameter(&self) -> SCAN_LINE_PARAMETER_R {
        SCAN_LINE_PARAMETER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDPI_TE_HW_CFG")
            .field(
                "hw_tear_effect_on",
                &format_args!("{}", self.hw_tear_effect_on().bit()),
            )
            .field(
                "hw_tear_effect_gen",
                &format_args!("{}", self.hw_tear_effect_gen().bit()),
            )
            .field(
                "hw_set_scan_line",
                &format_args!("{}", self.hw_set_scan_line().bit()),
            )
            .field(
                "scan_line_parameter",
                &format_args!("{}", self.scan_line_parameter().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EDPI_TE_HW_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hw_tear_effect_on(&mut self) -> HW_TEAR_EFFECT_ON_W<EDPI_TE_HW_CFG_SPEC> {
        HW_TEAR_EFFECT_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hw_tear_effect_gen(&mut self) -> HW_TEAR_EFFECT_GEN_W<EDPI_TE_HW_CFG_SPEC> {
        HW_TEAR_EFFECT_GEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hw_set_scan_line(&mut self) -> HW_SET_SCAN_LINE_W<EDPI_TE_HW_CFG_SPEC> {
        HW_SET_SCAN_LINE_W::new(self, 4)
    }
    #[doc = "Bits 16:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn scan_line_parameter(&mut self) -> SCAN_LINE_PARAMETER_W<EDPI_TE_HW_CFG_SPEC> {
        SCAN_LINE_PARAMETER_W::new(self, 16)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edpi_te_hw_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edpi_te_hw_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EDPI_TE_HW_CFG_SPEC;
impl crate::RegisterSpec for EDPI_TE_HW_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edpi_te_hw_cfg::R`](R) reader structure"]
impl crate::Readable for EDPI_TE_HW_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`edpi_te_hw_cfg::W`](W) writer structure"]
impl crate::Writable for EDPI_TE_HW_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDPI_TE_HW_CFG to value 0"]
impl crate::Resettable for EDPI_TE_HW_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
