#[doc = "Register `HP_TCM_INT_ENA` reader"]
pub type R = crate::R<HP_TCM_INT_ENA_SPEC>;
#[doc = "Register `HP_TCM_INT_ENA` writer"]
pub type W = crate::W<HP_TCM_INT_ENA_SPEC>;
#[doc = "Field `HP_TCM_PARITY_ERR_INT_ENA` reader - need_des"]
pub type HP_TCM_PARITY_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_TCM_PARITY_ERR_INT_ENA` writer - need_des"]
pub type HP_TCM_PARITY_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_tcm_parity_err_int_ena(&self) -> HP_TCM_PARITY_ERR_INT_ENA_R {
        HP_TCM_PARITY_ERR_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_TCM_INT_ENA")
            .field(
                "hp_tcm_parity_err_int_ena",
                &format_args!("{}", self.hp_tcm_parity_err_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_TCM_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_tcm_parity_err_int_ena(
        &mut self,
    ) -> HP_TCM_PARITY_ERR_INT_ENA_W<HP_TCM_INT_ENA_SPEC> {
        HP_TCM_PARITY_ERR_INT_ENA_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_tcm_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_tcm_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_TCM_INT_ENA_SPEC;
impl crate::RegisterSpec for HP_TCM_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_tcm_int_ena::R`](R) reader structure"]
impl crate::Readable for HP_TCM_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_tcm_int_ena::W`](W) writer structure"]
impl crate::Writable for HP_TCM_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_TCM_INT_ENA to value 0"]
impl crate::Resettable for HP_TCM_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
