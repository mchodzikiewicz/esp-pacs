#[doc = "Register `ECO_HIGH_APB` reader"]
pub type R = crate::R<ECO_HIGH_APB_SPEC>;
#[doc = "Register `ECO_HIGH_APB` writer"]
pub type W = crate::W<ECO_HIGH_APB_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_RND_ECO_HIGH_APB` reader - Reserved."]
pub type USB_SERIAL_JTAG_RND_ECO_HIGH_APB_R = crate::FieldReader<u32>;
#[doc = "Field `USB_SERIAL_JTAG_RND_ECO_HIGH_APB` writer - Reserved."]
pub type USB_SERIAL_JTAG_RND_ECO_HIGH_APB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn usb_serial_jtag_rnd_eco_high_apb(&self) -> USB_SERIAL_JTAG_RND_ECO_HIGH_APB_R {
        USB_SERIAL_JTAG_RND_ECO_HIGH_APB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_HIGH_APB")
            .field(
                "usb_serial_jtag_rnd_eco_high_apb",
                &format_args!("{}", self.usb_serial_jtag_rnd_eco_high_apb().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECO_HIGH_APB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_rnd_eco_high_apb(
        &mut self,
    ) -> USB_SERIAL_JTAG_RND_ECO_HIGH_APB_W<ECO_HIGH_APB_SPEC> {
        USB_SERIAL_JTAG_RND_ECO_HIGH_APB_W::new(self, 0)
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
#[doc = "Reserved.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eco_high_apb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_high_apb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECO_HIGH_APB_SPEC;
impl crate::RegisterSpec for ECO_HIGH_APB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_high_apb::R`](R) reader structure"]
impl crate::Readable for ECO_HIGH_APB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eco_high_apb::W`](W) writer structure"]
impl crate::Writable for ECO_HIGH_APB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECO_HIGH_APB to value 0xffff_ffff"]
impl crate::Resettable for ECO_HIGH_APB_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
