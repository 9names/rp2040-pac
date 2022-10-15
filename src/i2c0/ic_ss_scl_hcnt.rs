#[doc = "Register `IC_SS_SCL_HCNT` reader"]
pub struct R(crate::R<IC_SS_SCL_HCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_SS_SCL_HCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_SS_SCL_HCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_SS_SCL_HCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_SS_SCL_HCNT` writer"]
pub struct W(crate::W<IC_SS_SCL_HCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SS_SCL_HCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IC_SS_SCL_HCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SS_SCL_HCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC_SS_SCL_HCNT` reader - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.  

 This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.  

 NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
pub type IC_SS_SCL_HCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IC_SS_SCL_HCNT` writer - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.  

 This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.  

 NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
pub type IC_SS_SCL_HCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IC_SS_SCL_HCNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.  

 This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.  

 NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
    #[inline(always)]
    pub fn ic_ss_scl_hcnt(&self) -> IC_SS_SCL_HCNT_R {
        IC_SS_SCL_HCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.  

 This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.  

 NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
    #[inline(always)]
    pub fn ic_ss_scl_hcnt(&mut self) -> IC_SS_SCL_HCNT_W<0> {
        IC_SS_SCL_HCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standard Speed I2C Clock SCL High Count Register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_ss_scl_hcnt](index.html) module"]
pub struct IC_SS_SCL_HCNT_SPEC;
impl crate::RegisterSpec for IC_SS_SCL_HCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_ss_scl_hcnt::R](R) reader structure"]
impl crate::Readable for IC_SS_SCL_HCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_ss_scl_hcnt::W](W) writer structure"]
impl crate::Writable for IC_SS_SCL_HCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_SS_SCL_HCNT to value 0x28"]
impl crate::Resettable for IC_SS_SCL_HCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x28
    }
}
