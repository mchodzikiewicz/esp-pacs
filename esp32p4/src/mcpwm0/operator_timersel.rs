#[doc = "Register `OPERATOR_TIMERSEL` reader"]
pub type R = crate::R<OPERATOR_TIMERSEL_SPEC>;
#[doc = "Register `OPERATOR_TIMERSEL` writer"]
pub type W = crate::W<OPERATOR_TIMERSEL_SPEC>;
#[doc = "Field `OPERATOR0_TIMERSEL` reader - Configures which PWM timer will be the timing reference for PWM operator0.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
pub type OPERATOR0_TIMERSEL_R = crate::FieldReader;
#[doc = "Field `OPERATOR0_TIMERSEL` writer - Configures which PWM timer will be the timing reference for PWM operator0.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
pub type OPERATOR0_TIMERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPERATOR1_TIMERSEL` reader - Configures which PWM timer will be the timing reference for PWM operator1.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
pub type OPERATOR1_TIMERSEL_R = crate::FieldReader;
#[doc = "Field `OPERATOR1_TIMERSEL` writer - Configures which PWM timer will be the timing reference for PWM operator1.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
pub type OPERATOR1_TIMERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPERATOR2_TIMERSEL` reader - Configures which PWM timer will be the timing reference for PWM operator2.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
pub type OPERATOR2_TIMERSEL_R = crate::FieldReader;
#[doc = "Field `OPERATOR2_TIMERSEL` writer - Configures which PWM timer will be the timing reference for PWM operator2.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
pub type OPERATOR2_TIMERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Configures which PWM timer will be the timing reference for PWM operator0.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    pub fn operator0_timersel(&self) -> OPERATOR0_TIMERSEL_R {
        OPERATOR0_TIMERSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configures which PWM timer will be the timing reference for PWM operator1.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    pub fn operator1_timersel(&self) -> OPERATOR1_TIMERSEL_R {
        OPERATOR1_TIMERSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configures which PWM timer will be the timing reference for PWM operator2.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    pub fn operator2_timersel(&self) -> OPERATOR2_TIMERSEL_R {
        OPERATOR2_TIMERSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPERATOR_TIMERSEL")
            .field(
                "operator0_timersel",
                &format_args!("{}", self.operator0_timersel().bits()),
            )
            .field(
                "operator1_timersel",
                &format_args!("{}", self.operator1_timersel().bits()),
            )
            .field(
                "operator2_timersel",
                &format_args!("{}", self.operator2_timersel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OPERATOR_TIMERSEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures which PWM timer will be the timing reference for PWM operator0.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    #[must_use]
    pub fn operator0_timersel(&mut self) -> OPERATOR0_TIMERSEL_W<OPERATOR_TIMERSEL_SPEC> {
        OPERATOR0_TIMERSEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Configures which PWM timer will be the timing reference for PWM operator1.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    #[must_use]
    pub fn operator1_timersel(&mut self) -> OPERATOR1_TIMERSEL_W<OPERATOR_TIMERSEL_SPEC> {
        OPERATOR1_TIMERSEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configures which PWM timer will be the timing reference for PWM operator2.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    #[must_use]
    pub fn operator2_timersel(&mut self) -> OPERATOR2_TIMERSEL_W<OPERATOR_TIMERSEL_SPEC> {
        OPERATOR2_TIMERSEL_W::new(self, 4)
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
#[doc = "PWM operator's timer select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`operator_timersel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`operator_timersel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPERATOR_TIMERSEL_SPEC;
impl crate::RegisterSpec for OPERATOR_TIMERSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`operator_timersel::R`](R) reader structure"]
impl crate::Readable for OPERATOR_TIMERSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`operator_timersel::W`](W) writer structure"]
impl crate::Writable for OPERATOR_TIMERSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPERATOR_TIMERSEL to value 0"]
impl crate::Resettable for OPERATOR_TIMERSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
