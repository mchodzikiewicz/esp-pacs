#[doc = "Register `CRYPTO_CTRL` reader"]
pub type R = crate::R<CRYPTO_CTRL_SPEC>;
#[doc = "Register `CRYPTO_CTRL` writer"]
pub type W = crate::W<CRYPTO_CTRL_SPEC>;
#[doc = "Field `REG_ENABLE_SPI_MANUAL_ENCRYPT` reader - NA"]
pub type REG_ENABLE_SPI_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `REG_ENABLE_SPI_MANUAL_ENCRYPT` writer - NA"]
pub type REG_ENABLE_SPI_MANUAL_ENCRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ENABLE_DOWNLOAD_DB_ENCRYPT` reader - NA"]
pub type REG_ENABLE_DOWNLOAD_DB_ENCRYPT_R = crate::BitReader;
#[doc = "Field `REG_ENABLE_DOWNLOAD_DB_ENCRYPT` writer - NA"]
pub type REG_ENABLE_DOWNLOAD_DB_ENCRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ENABLE_DOWNLOAD_G0CB_DECRYPT` reader - NA"]
pub type REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_R = crate::BitReader;
#[doc = "Field `REG_ENABLE_DOWNLOAD_G0CB_DECRYPT` writer - NA"]
pub type REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT` reader - NA"]
pub type REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT` writer - NA"]
pub type REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_enable_spi_manual_encrypt(&self) -> REG_ENABLE_SPI_MANUAL_ENCRYPT_R {
        REG_ENABLE_SPI_MANUAL_ENCRYPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_enable_download_db_encrypt(&self) -> REG_ENABLE_DOWNLOAD_DB_ENCRYPT_R {
        REG_ENABLE_DOWNLOAD_DB_ENCRYPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_enable_download_g0cb_decrypt(&self) -> REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_R {
        REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_enable_download_manual_encrypt(&self) -> REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R {
        REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYPTO_CTRL")
            .field(
                "reg_enable_spi_manual_encrypt",
                &format_args!("{}", self.reg_enable_spi_manual_encrypt().bit()),
            )
            .field(
                "reg_enable_download_db_encrypt",
                &format_args!("{}", self.reg_enable_download_db_encrypt().bit()),
            )
            .field(
                "reg_enable_download_g0cb_decrypt",
                &format_args!("{}", self.reg_enable_download_g0cb_decrypt().bit()),
            )
            .field(
                "reg_enable_download_manual_encrypt",
                &format_args!("{}", self.reg_enable_download_manual_encrypt().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CRYPTO_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_enable_spi_manual_encrypt(
        &mut self,
    ) -> REG_ENABLE_SPI_MANUAL_ENCRYPT_W<CRYPTO_CTRL_SPEC> {
        REG_ENABLE_SPI_MANUAL_ENCRYPT_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_enable_download_db_encrypt(
        &mut self,
    ) -> REG_ENABLE_DOWNLOAD_DB_ENCRYPT_W<CRYPTO_CTRL_SPEC> {
        REG_ENABLE_DOWNLOAD_DB_ENCRYPT_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_enable_download_g0cb_decrypt(
        &mut self,
    ) -> REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_W<CRYPTO_CTRL_SPEC> {
        REG_ENABLE_DOWNLOAD_G0CB_DECRYPT_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_enable_download_manual_encrypt(
        &mut self,
    ) -> REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<CRYPTO_CTRL_SPEC> {
        REG_ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W::new(self, 3)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crypto_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crypto_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYPTO_CTRL_SPEC;
impl crate::RegisterSpec for CRYPTO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_ctrl::R`](R) reader structure"]
impl crate::Readable for CRYPTO_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crypto_ctrl::W`](W) writer structure"]
impl crate::Writable for CRYPTO_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYPTO_CTRL to value 0"]
impl crate::Resettable for CRYPTO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
