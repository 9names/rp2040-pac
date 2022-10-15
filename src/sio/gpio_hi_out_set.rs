#[doc = "Register `GPIO_HI_OUT_SET` writer"]
pub struct W(crate::W<GPIO_HI_OUT_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_HI_OUT_SET_SPEC>;
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
impl From<crate::W<GPIO_HI_OUT_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_HI_OUT_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_HI_OUT_SET` writer - Perform an atomic bit-set on GPIO_HI_OUT, i.e. `GPIO_HI_OUT |= wdata`"]
pub type GPIO_HI_OUT_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_HI_OUT_SET_SPEC, u8, u8, 6, O>;
impl W {
    #[doc = "Bits 0:5 - Perform an atomic bit-set on GPIO_HI_OUT, i.e. `GPIO_HI_OUT |= wdata`"]
    #[inline(always)]
    pub fn gpio_hi_out_set(&mut self) -> GPIO_HI_OUT_SET_W<0> {
        GPIO_HI_OUT_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI output value set  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_hi_out_set](index.html) module"]
pub struct GPIO_HI_OUT_SET_SPEC;
impl crate::RegisterSpec for GPIO_HI_OUT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpio_hi_out_set::W](W) writer structure"]
impl crate::Writable for GPIO_HI_OUT_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_HI_OUT_SET to value 0"]
impl crate::Resettable for GPIO_HI_OUT_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
