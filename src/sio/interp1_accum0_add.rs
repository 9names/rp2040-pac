#[doc = "Register `INTERP1_ACCUM0_ADD` reader"]
pub struct R(crate::R<INTERP1_ACCUM0_ADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP1_ACCUM0_ADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERP1_ACCUM0_ADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERP1_ACCUM0_ADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERP1_ACCUM0_ADD` writer"]
pub struct W(crate::W<INTERP1_ACCUM0_ADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERP1_ACCUM0_ADD_SPEC>;
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
impl From<crate::W<INTERP1_ACCUM0_ADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERP1_ACCUM0_ADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERP1_ACCUM0_ADD` reader - "]
pub type INTERP1_ACCUM0_ADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTERP1_ACCUM0_ADD` writer - "]
pub type INTERP1_ACCUM0_ADD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTERP1_ACCUM0_ADD_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn interp1_accum0_add(&self) -> INTERP1_ACCUM0_ADD_R {
        INTERP1_ACCUM0_ADD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn interp1_accum0_add(&mut self) -> INTERP1_ACCUM0_ADD_W<0> {
        INTERP1_ACCUM0_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Values written here are atomically added to ACCUM0  
 Reading yields lane 0's raw shift and mask value (BASE0 not added).  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [interp1_accum0_add](index.html) module"]
pub struct INTERP1_ACCUM0_ADD_SPEC;
impl crate::RegisterSpec for INTERP1_ACCUM0_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp1_accum0_add::R](R) reader structure"]
impl crate::Readable for INTERP1_ACCUM0_ADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interp1_accum0_add::W](W) writer structure"]
impl crate::Writable for INTERP1_ACCUM0_ADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERP1_ACCUM0_ADD to value 0"]
impl crate::Resettable for INTERP1_ACCUM0_ADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
