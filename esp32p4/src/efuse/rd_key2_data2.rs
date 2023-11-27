#[doc = "Register `RD_KEY2_DATA2` reader"]
pub type R = crate::R<RD_KEY2_DATA2_SPEC>;
#[doc = "Field `KEY2_DATA2` reader - Stores the second 32 bits of KEY2."]
pub type KEY2_DATA2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the second 32 bits of KEY2."]
    #[inline(always)]
    pub fn key2_data2(&self) -> KEY2_DATA2_R {
        KEY2_DATA2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY2_DATA2")
            .field("key2_data2", &format_args!("{}", self.key2_data2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY2_DATA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Register $n of BLOCK6 (KEY2).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY2_DATA2_SPEC;
impl crate::RegisterSpec for RD_KEY2_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key2_data2::R`](R) reader structure"]
impl crate::Readable for RD_KEY2_DATA2_SPEC {}
#[doc = "`reset()` method sets RD_KEY2_DATA2 to value 0"]
impl crate::Resettable for RD_KEY2_DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
