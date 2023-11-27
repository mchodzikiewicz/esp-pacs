#[doc = "Register `RD_SYS_PART1_DATA0` reader"]
pub type R = crate::R<RD_SYS_PART1_DATA0_SPEC>;
#[doc = "Field `SYS_DATA_PART1_0` reader - Stores the zeroth 32 bits of the first part of system data."]
pub type SYS_DATA_PART1_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the zeroth 32 bits of the first part of system data."]
    #[inline(always)]
    pub fn sys_data_part1_0(&self) -> SYS_DATA_PART1_0_R {
        SYS_DATA_PART1_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_SYS_PART1_DATA0")
            .field(
                "sys_data_part1_0",
                &format_args!("{}", self.sys_data_part1_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_SYS_PART1_DATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register $n of BLOCK2 (system).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_SYS_PART1_DATA0_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part1_data0::R`](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA0_SPEC {}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA0 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
