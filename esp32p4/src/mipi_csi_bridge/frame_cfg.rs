#[doc = "Register `FRAME_CFG` reader"]
pub type R = crate::R<FRAME_CFG_SPEC>;
#[doc = "Register `FRAME_CFG` writer"]
pub type W = crate::W<FRAME_CFG_SPEC>;
#[doc = "Field `VADR_NUM` reader - vadr of frame data."]
pub type VADR_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `VADR_NUM` writer - vadr of frame data."]
pub type VADR_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HADR_NUM` reader - hadr of frame data."]
pub type HADR_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `HADR_NUM` writer - hadr of frame data."]
pub type HADR_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HAS_HSYNC_E` reader - 0: frame data doesn't contain hsync. 1: frame data contains hsync."]
pub type HAS_HSYNC_E_R = crate::BitReader;
#[doc = "Field `HAS_HSYNC_E` writer - 0: frame data doesn't contain hsync. 1: frame data contains hsync."]
pub type HAS_HSYNC_E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VADR_NUM_CHECK` reader - 0: disable vadr check. 1: enable vadr check."]
pub type VADR_NUM_CHECK_R = crate::BitReader;
#[doc = "Field `VADR_NUM_CHECK` writer - 0: disable vadr check. 1: enable vadr check."]
pub type VADR_NUM_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - vadr of frame data."]
    #[inline(always)]
    pub fn vadr_num(&self) -> VADR_NUM_R {
        VADR_NUM_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - hadr of frame data."]
    #[inline(always)]
    pub fn hadr_num(&self) -> HADR_NUM_R {
        HADR_NUM_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - 0: frame data doesn't contain hsync. 1: frame data contains hsync."]
    #[inline(always)]
    pub fn has_hsync_e(&self) -> HAS_HSYNC_E_R {
        HAS_HSYNC_E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: disable vadr check. 1: enable vadr check."]
    #[inline(always)]
    pub fn vadr_num_check(&self) -> VADR_NUM_CHECK_R {
        VADR_NUM_CHECK_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_CFG")
            .field("vadr_num", &format_args!("{}", self.vadr_num().bits()))
            .field("hadr_num", &format_args!("{}", self.hadr_num().bits()))
            .field("has_hsync_e", &format_args!("{}", self.has_hsync_e().bit()))
            .field(
                "vadr_num_check",
                &format_args!("{}", self.vadr_num_check().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRAME_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - vadr of frame data."]
    #[inline(always)]
    #[must_use]
    pub fn vadr_num(&mut self) -> VADR_NUM_W<FRAME_CFG_SPEC> {
        VADR_NUM_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - hadr of frame data."]
    #[inline(always)]
    #[must_use]
    pub fn hadr_num(&mut self) -> HADR_NUM_W<FRAME_CFG_SPEC> {
        HADR_NUM_W::new(self, 12)
    }
    #[doc = "Bit 24 - 0: frame data doesn't contain hsync. 1: frame data contains hsync."]
    #[inline(always)]
    #[must_use]
    pub fn has_hsync_e(&mut self) -> HAS_HSYNC_E_W<FRAME_CFG_SPEC> {
        HAS_HSYNC_E_W::new(self, 24)
    }
    #[doc = "Bit 25 - 0: disable vadr check. 1: enable vadr check."]
    #[inline(always)]
    #[must_use]
    pub fn vadr_num_check(&mut self) -> VADR_NUM_CHECK_W<FRAME_CFG_SPEC> {
        VADR_NUM_CHECK_W::new(self, 25)
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
#[doc = "frame configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frame_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frame_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAME_CFG_SPEC;
impl crate::RegisterSpec for FRAME_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_cfg::R`](R) reader structure"]
impl crate::Readable for FRAME_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frame_cfg::W`](W) writer structure"]
impl crate::Writable for FRAME_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAME_CFG to value 0x011e_01e0"]
impl crate::Resettable for FRAME_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x011e_01e0;
}
