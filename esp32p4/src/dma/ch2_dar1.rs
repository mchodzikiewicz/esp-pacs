#[doc = "Register `CH2_DAR1` reader"]
pub type R = crate::R<CH2_DAR1_SPEC>;
#[doc = "Register `CH2_DAR1` writer"]
pub type W = crate::W<CH2_DAR1_SPEC>;
#[doc = "Field `CH2_DAR1` reader - NA"]
pub type CH2_DAR1_R = crate::FieldReader<u32>;
#[doc = "Field `CH2_DAR1` writer - NA"]
pub type CH2_DAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch2_dar1(&self) -> CH2_DAR1_R {
        CH2_DAR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_DAR1")
            .field("ch2_dar1", &format_args!("{}", self.ch2_dar1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH2_DAR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_dar1(&mut self) -> CH2_DAR1_W<CH2_DAR1_SPEC> {
        CH2_DAR1_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_dar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_DAR1_SPEC;
impl crate::RegisterSpec for CH2_DAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_dar1::R`](R) reader structure"]
impl crate::Readable for CH2_DAR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2_dar1::W`](W) writer structure"]
impl crate::Writable for CH2_DAR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2_DAR1 to value 0"]
impl crate::Resettable for CH2_DAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
