#[doc = "Register `CH%s_CONF1` reader"]
pub type R = crate::R<CH_CONF1_SPEC>;
#[doc = "Register `CH%s_CONF1` writer"]
pub type W = crate::W<CH_CONF1_SPEC>;
#[doc = "Field `DUTY_START_CH` reader - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
pub type DUTY_START_CH_R = crate::BitReader;
#[doc = "Field `DUTY_START_CH` writer - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
pub type DUTY_START_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
    #[inline(always)]
    pub fn duty_start_ch(&self) -> DUTY_START_CH_R {
        DUTY_START_CH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_CONF1")
            .field(
                "duty_start_ch",
                &format_args!("{}", self.duty_start_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - Configures whether the duty cycle fading configurations take effect.\\\\0: Not take effect\\\\1: Take effect"]
    #[inline(always)]
    #[must_use]
    pub fn duty_start_ch(&mut self) -> DUTY_START_CH_W<CH_CONF1_SPEC> {
        DUTY_START_CH_W::new(self, 31)
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
#[doc = "Configuration register 1 for channel %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_CONF1_SPEC;
impl crate::RegisterSpec for CH_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_conf1::R`](R) reader structure"]
impl crate::Readable for CH_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_conf1::W`](W) writer structure"]
impl crate::Writable for CH_CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_CONF1 to value 0"]
impl crate::Resettable for CH_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
