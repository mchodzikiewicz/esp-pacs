#[doc = "Register `HP_L2_MEM_L2_RAM_ECC` reader"]
pub type R = crate::R<HP_L2_MEM_L2_RAM_ECC_SPEC>;
#[doc = "Register `HP_L2_MEM_L2_RAM_ECC` writer"]
pub type W = crate::W<HP_L2_MEM_L2_RAM_ECC_SPEC>;
#[doc = "Field `HP_REG_L2_RAM_UNIT0_ECC_EN` reader - NA"]
pub type HP_REG_L2_RAM_UNIT0_ECC_EN_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_RAM_UNIT0_ECC_EN` writer - NA"]
pub type HP_REG_L2_RAM_UNIT0_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_L2_RAM_UNIT1_ECC_EN` reader - NA"]
pub type HP_REG_L2_RAM_UNIT1_ECC_EN_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_RAM_UNIT1_ECC_EN` writer - NA"]
pub type HP_REG_L2_RAM_UNIT1_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_L2_RAM_UNIT2_ECC_EN` reader - NA"]
pub type HP_REG_L2_RAM_UNIT2_ECC_EN_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_RAM_UNIT2_ECC_EN` writer - NA"]
pub type HP_REG_L2_RAM_UNIT2_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_L2_RAM_UNIT3_ECC_EN` reader - NA"]
pub type HP_REG_L2_RAM_UNIT3_ECC_EN_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_RAM_UNIT3_ECC_EN` writer - NA"]
pub type HP_REG_L2_RAM_UNIT3_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_L2_RAM_UNIT4_ECC_EN` reader - NA"]
pub type HP_REG_L2_RAM_UNIT4_ECC_EN_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_RAM_UNIT4_ECC_EN` writer - NA"]
pub type HP_REG_L2_RAM_UNIT4_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_L2_RAM_UNIT5_ECC_EN` reader - NA"]
pub type HP_REG_L2_RAM_UNIT5_ECC_EN_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_RAM_UNIT5_ECC_EN` writer - NA"]
pub type HP_REG_L2_RAM_UNIT5_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_ram_unit0_ecc_en(&self) -> HP_REG_L2_RAM_UNIT0_ECC_EN_R {
        HP_REG_L2_RAM_UNIT0_ECC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_ram_unit1_ecc_en(&self) -> HP_REG_L2_RAM_UNIT1_ECC_EN_R {
        HP_REG_L2_RAM_UNIT1_ECC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_ram_unit2_ecc_en(&self) -> HP_REG_L2_RAM_UNIT2_ECC_EN_R {
        HP_REG_L2_RAM_UNIT2_ECC_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_ram_unit3_ecc_en(&self) -> HP_REG_L2_RAM_UNIT3_ECC_EN_R {
        HP_REG_L2_RAM_UNIT3_ECC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_ram_unit4_ecc_en(&self) -> HP_REG_L2_RAM_UNIT4_ECC_EN_R {
        HP_REG_L2_RAM_UNIT4_ECC_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_ram_unit5_ecc_en(&self) -> HP_REG_L2_RAM_UNIT5_ECC_EN_R {
        HP_REG_L2_RAM_UNIT5_ECC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_L2_MEM_L2_RAM_ECC")
            .field(
                "hp_reg_l2_ram_unit0_ecc_en",
                &format_args!("{}", self.hp_reg_l2_ram_unit0_ecc_en().bit()),
            )
            .field(
                "hp_reg_l2_ram_unit1_ecc_en",
                &format_args!("{}", self.hp_reg_l2_ram_unit1_ecc_en().bit()),
            )
            .field(
                "hp_reg_l2_ram_unit2_ecc_en",
                &format_args!("{}", self.hp_reg_l2_ram_unit2_ecc_en().bit()),
            )
            .field(
                "hp_reg_l2_ram_unit3_ecc_en",
                &format_args!("{}", self.hp_reg_l2_ram_unit3_ecc_en().bit()),
            )
            .field(
                "hp_reg_l2_ram_unit4_ecc_en",
                &format_args!("{}", self.hp_reg_l2_ram_unit4_ecc_en().bit()),
            )
            .field(
                "hp_reg_l2_ram_unit5_ecc_en",
                &format_args!("{}", self.hp_reg_l2_ram_unit5_ecc_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_L2_MEM_L2_RAM_ECC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_ram_unit0_ecc_en(
        &mut self,
    ) -> HP_REG_L2_RAM_UNIT0_ECC_EN_W<HP_L2_MEM_L2_RAM_ECC_SPEC> {
        HP_REG_L2_RAM_UNIT0_ECC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_ram_unit1_ecc_en(
        &mut self,
    ) -> HP_REG_L2_RAM_UNIT1_ECC_EN_W<HP_L2_MEM_L2_RAM_ECC_SPEC> {
        HP_REG_L2_RAM_UNIT1_ECC_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_ram_unit2_ecc_en(
        &mut self,
    ) -> HP_REG_L2_RAM_UNIT2_ECC_EN_W<HP_L2_MEM_L2_RAM_ECC_SPEC> {
        HP_REG_L2_RAM_UNIT2_ECC_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_ram_unit3_ecc_en(
        &mut self,
    ) -> HP_REG_L2_RAM_UNIT3_ECC_EN_W<HP_L2_MEM_L2_RAM_ECC_SPEC> {
        HP_REG_L2_RAM_UNIT3_ECC_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_ram_unit4_ecc_en(
        &mut self,
    ) -> HP_REG_L2_RAM_UNIT4_ECC_EN_W<HP_L2_MEM_L2_RAM_ECC_SPEC> {
        HP_REG_L2_RAM_UNIT4_ECC_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_ram_unit5_ecc_en(
        &mut self,
    ) -> HP_REG_L2_RAM_UNIT5_ECC_EN_W<HP_L2_MEM_L2_RAM_ECC_SPEC> {
        HP_REG_L2_RAM_UNIT5_ECC_EN_W::new(self, 5)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_l2_ram_ecc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_l2_ram_ecc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_L2_MEM_L2_RAM_ECC_SPEC;
impl crate::RegisterSpec for HP_L2_MEM_L2_RAM_ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_l2_mem_l2_ram_ecc::R`](R) reader structure"]
impl crate::Readable for HP_L2_MEM_L2_RAM_ECC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_l2_mem_l2_ram_ecc::W`](W) writer structure"]
impl crate::Writable for HP_L2_MEM_L2_RAM_ECC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_L2_MEM_L2_RAM_ECC to value 0"]
impl crate::Resettable for HP_L2_MEM_L2_RAM_ECC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
