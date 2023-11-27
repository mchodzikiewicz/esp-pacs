#[doc = "Register `SCL_STOP_SETUP` reader"]
pub type R = crate::R<SCL_STOP_SETUP_SPEC>;
#[doc = "Register `SCL_STOP_SETUP` writer"]
pub type W = crate::W<SCL_STOP_SETUP_SPEC>;
#[doc = "Field `REG_SCL_STOP_SETUP_TIME` reader - I2C_SCL_STOP_SETUP_TIME"]
pub type REG_SCL_STOP_SETUP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `REG_SCL_STOP_SETUP_TIME` writer - I2C_SCL_STOP_SETUP_TIME"]
pub type REG_SCL_STOP_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - I2C_SCL_STOP_SETUP_TIME"]
    #[inline(always)]
    pub fn reg_scl_stop_setup_time(&self) -> REG_SCL_STOP_SETUP_TIME_R {
        REG_SCL_STOP_SETUP_TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_STOP_SETUP")
            .field(
                "reg_scl_stop_setup_time",
                &format_args!("{}", self.reg_scl_stop_setup_time().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_STOP_SETUP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8 - I2C_SCL_STOP_SETUP_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn reg_scl_stop_setup_time(&mut self) -> REG_SCL_STOP_SETUP_TIME_W<SCL_STOP_SETUP_SPEC> {
        REG_SCL_STOP_SETUP_TIME_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_stop_setup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_stop_setup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_STOP_SETUP_SPEC;
impl crate::RegisterSpec for SCL_STOP_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_stop_setup::R`](R) reader structure"]
impl crate::Readable for SCL_STOP_SETUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_stop_setup::W`](W) writer structure"]
impl crate::Writable for SCL_STOP_SETUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_STOP_SETUP to value 0x08"]
impl crate::Resettable for SCL_STOP_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
