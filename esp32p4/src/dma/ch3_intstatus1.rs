#[doc = "Register `CH3_INTSTATUS1` reader"]
pub type R = crate::R<CH3_INTSTATUS1_SPEC>;
#[doc = "Field `CH3_ECC_PROT_CHMEM_CORRERR_INTSTAT` reader - NA"]
pub type CH3_ECC_PROT_CHMEM_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH3_ECC_PROT_CHMEM_UNCORRERR_INTSTAT` reader - NA"]
pub type CH3_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH3_ECC_PROT_UIDMEM_CORRERR_INTSTAT` reader - NA"]
pub type CH3_ECC_PROT_UIDMEM_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH3_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT` reader - NA"]
pub type CH3_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch3_ecc_prot_chmem_correrr_intstat(&self) -> CH3_ECC_PROT_CHMEM_CORRERR_INTSTAT_R {
        CH3_ECC_PROT_CHMEM_CORRERR_INTSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch3_ecc_prot_chmem_uncorrerr_intstat(&self) -> CH3_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_R {
        CH3_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch3_ecc_prot_uidmem_correrr_intstat(&self) -> CH3_ECC_PROT_UIDMEM_CORRERR_INTSTAT_R {
        CH3_ECC_PROT_UIDMEM_CORRERR_INTSTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch3_ecc_prot_uidmem_uncorrerr_intstat(&self) -> CH3_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_R {
        CH3_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH3_INTSTATUS1")
            .field(
                "ch3_ecc_prot_chmem_correrr_intstat",
                &format_args!("{}", self.ch3_ecc_prot_chmem_correrr_intstat().bit()),
            )
            .field(
                "ch3_ecc_prot_chmem_uncorrerr_intstat",
                &format_args!("{}", self.ch3_ecc_prot_chmem_uncorrerr_intstat().bit()),
            )
            .field(
                "ch3_ecc_prot_uidmem_correrr_intstat",
                &format_args!("{}", self.ch3_ecc_prot_uidmem_correrr_intstat().bit()),
            )
            .field(
                "ch3_ecc_prot_uidmem_uncorrerr_intstat",
                &format_args!("{}", self.ch3_ecc_prot_uidmem_uncorrerr_intstat().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH3_INTSTATUS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_intstatus1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3_INTSTATUS1_SPEC;
impl crate::RegisterSpec for CH3_INTSTATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_intstatus1::R`](R) reader structure"]
impl crate::Readable for CH3_INTSTATUS1_SPEC {}
#[doc = "`reset()` method sets CH3_INTSTATUS1 to value 0"]
impl crate::Resettable for CH3_INTSTATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
