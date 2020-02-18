#[doc = "Reader of register DMA0_ITRIG_ENA"]
pub type R = crate::R<u32, super::DMA0_ITRIG_ENA>;
#[doc = "Writer for register DMA0_ITRIG_ENA"]
pub type W = crate::W<u32, super::DMA0_ITRIG_ENA>;
#[doc = "Register DMA0_ITRIG_ENA `reset()`'s with value 0x003f_ffff"]
impl crate::ResetValue for super::DMA0_ITRIG_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x003f_ffff
    }
}
#[doc = "Reader of field `ITRIG_ENA`"]
pub type ITRIG_ENA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ITRIG_ENA`"]
pub struct ITRIG_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIG_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&self) -> ITRIG_ENA_R {
        ITRIG_ENA_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub fn itrig_ena(&mut self) -> ITRIG_ENA_W {
        ITRIG_ENA_W { w: self }
    }
}
