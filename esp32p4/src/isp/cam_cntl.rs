#[doc = "Register `CAM_CNTL` reader"]
pub type R = crate::R<CAM_CNTL_SPEC>;
#[doc = "Register `CAM_CNTL` writer"]
pub type W = crate::W<CAM_CNTL_SPEC>;
#[doc = "Field `CAM_EN` reader - write 1 to start recive camera data, write 0 to disable"]
pub type CAM_EN_R = crate::BitReader;
#[doc = "Field `CAM_EN` writer - write 1 to start recive camera data, write 0 to disable"]
pub type CAM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_UPDATE` reader - write 1 to update ISP_CAM_CONF"]
pub type CAM_UPDATE_R = crate::BitReader;
#[doc = "Field `CAM_UPDATE` writer - write 1 to update ISP_CAM_CONF"]
pub type CAM_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_RESET` reader - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
pub type CAM_RESET_R = crate::BitReader;
#[doc = "Field `CAM_RESET` writer - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
pub type CAM_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CLK_INV` reader - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
pub type CAM_CLK_INV_R = crate::BitReader;
#[doc = "Field `CAM_CLK_INV` writer - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
pub type CAM_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to start recive camera data, write 0 to disable"]
    #[inline(always)]
    pub fn cam_en(&self) -> CAM_EN_R {
        CAM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write 1 to update ISP_CAM_CONF"]
    #[inline(always)]
    pub fn cam_update(&self) -> CAM_UPDATE_R {
        CAM_UPDATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
    #[inline(always)]
    pub fn cam_reset(&self) -> CAM_RESET_R {
        CAM_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
    #[inline(always)]
    pub fn cam_clk_inv(&self) -> CAM_CLK_INV_R {
        CAM_CLK_INV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAM_CNTL")
            .field("cam_en", &format_args!("{}", self.cam_en().bit()))
            .field("cam_update", &format_args!("{}", self.cam_update().bit()))
            .field("cam_reset", &format_args!("{}", self.cam_reset().bit()))
            .field("cam_clk_inv", &format_args!("{}", self.cam_clk_inv().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAM_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to start recive camera data, write 0 to disable"]
    #[inline(always)]
    #[must_use]
    pub fn cam_en(&mut self) -> CAM_EN_W<CAM_CNTL_SPEC> {
        CAM_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - write 1 to update ISP_CAM_CONF"]
    #[inline(always)]
    #[must_use]
    pub fn cam_update(&mut self) -> CAM_UPDATE_W<CAM_CNTL_SPEC> {
        CAM_UPDATE_W::new(self, 1)
    }
    #[doc = "Bit 2 - this bit configures cam clk domain reset, 1: reset cam input logic, 0: release reset"]
    #[inline(always)]
    #[must_use]
    pub fn cam_reset(&mut self) -> CAM_RESET_W<CAM_CNTL_SPEC> {
        CAM_RESET_W::new(self, 2)
    }
    #[doc = "Bit 3 - this bit configures the invertion of cam clk from pad. 0: not invert cam clk, 1: invert cam clk"]
    #[inline(always)]
    #[must_use]
    pub fn cam_clk_inv(&mut self) -> CAM_CLK_INV_W<CAM_CNTL_SPEC> {
        CAM_CLK_INV_W::new(self, 3)
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
#[doc = "isp cam source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAM_CNTL_SPEC;
impl crate::RegisterSpec for CAM_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_cntl::R`](R) reader structure"]
impl crate::Readable for CAM_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cam_cntl::W`](W) writer structure"]
impl crate::Writable for CAM_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAM_CNTL to value 0x04"]
impl crate::Resettable for CAM_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
