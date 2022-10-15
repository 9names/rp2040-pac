#[doc = "Register `PLATFORM` reader"]
pub struct R(crate::R<PLATFORM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLATFORM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLATFORM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLATFORM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ASIC` reader - Indicates the platform is an ASIC"]
pub type ASIC_R = crate::BitReader<bool>;
#[doc = "Field `FPGA` reader - Indicates the platform is an FPGA"]
pub type FPGA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates the platform is an ASIC"]
    #[inline(always)]
    pub fn asic(&self) -> ASIC_R {
        ASIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the platform is an FPGA"]
    #[inline(always)]
    pub fn fpga(&self) -> FPGA_R {
        FPGA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Indicates the type of platform in use  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [platform](index.html) module"]
pub struct PLATFORM_SPEC;
impl crate::RegisterSpec for PLATFORM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [platform::R](R) reader structure"]
impl crate::Readable for PLATFORM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PLATFORM to value 0x05"]
impl crate::Resettable for PLATFORM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
