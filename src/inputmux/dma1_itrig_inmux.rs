#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA1_ITRIG_INMUX {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `INP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPR {
    #[doc = "Pin interrupt 0"]
    VAL0,
    #[doc = "Pin interrupt 1"]
    VAL1,
    #[doc = "Pin interrupt 2"]
    VAL2,
    #[doc = "Pin interrupt 3"]
    VAL3,
    #[doc = "Timer CTIMER0 Match 0"]
    VAL4,
    #[doc = "Timer CTIMER0 Match 1"]
    VAL5,
    #[doc = "Timer CTIMER2 Match 0"]
    VAL6,
    #[doc = "Timer CTIMER4 Match 0"]
    VAL7,
    #[doc = "DMA1 output trigger mux 0"]
    VAL8,
    #[doc = "DMA1 output trigger mux 1"]
    VAL9,
    #[doc = "DMA1 output trigger mux 2"]
    VAL10,
    #[doc = "DMA1 output trigger mux 3"]
    VAL11,
    #[doc = "SCT0 DMA request 0"]
    VAL12,
    #[doc = "SCT0 DMA request 1"]
    VAL13,
    #[doc = "HASH DMA RX trigger"]
    VAL14,
    #[doc = "None"]
    VAL15,
}
impl INPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPR::VAL0 => 0,
            INPR::VAL1 => 1,
            INPR::VAL2 => 2,
            INPR::VAL3 => 3,
            INPR::VAL4 => 4,
            INPR::VAL5 => 5,
            INPR::VAL6 => 6,
            INPR::VAL7 => 7,
            INPR::VAL8 => 8,
            INPR::VAL9 => 9,
            INPR::VAL10 => 10,
            INPR::VAL11 => 11,
            INPR::VAL12 => 12,
            INPR::VAL13 => 13,
            INPR::VAL14 => 14,
            INPR::VAL15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPR {
        match value {
            0 => INPR::VAL0,
            1 => INPR::VAL1,
            2 => INPR::VAL2,
            3 => INPR::VAL3,
            4 => INPR::VAL4,
            5 => INPR::VAL5,
            6 => INPR::VAL6,
            7 => INPR::VAL7,
            8 => INPR::VAL8,
            9 => INPR::VAL9,
            10 => INPR::VAL10,
            11 => INPR::VAL11,
            12 => INPR::VAL12,
            13 => INPR::VAL13,
            14 => INPR::VAL14,
            15 => INPR::VAL15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline]
    pub fn is_val0(&self) -> bool {
        *self == INPR::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline]
    pub fn is_val1(&self) -> bool {
        *self == INPR::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline]
    pub fn is_val2(&self) -> bool {
        *self == INPR::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline]
    pub fn is_val3(&self) -> bool {
        *self == INPR::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline]
    pub fn is_val4(&self) -> bool {
        *self == INPR::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline]
    pub fn is_val5(&self) -> bool {
        *self == INPR::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline]
    pub fn is_val6(&self) -> bool {
        *self == INPR::VAL6
    }
    #[doc = "Checks if the value of the field is `VAL7`"]
    #[inline]
    pub fn is_val7(&self) -> bool {
        *self == INPR::VAL7
    }
    #[doc = "Checks if the value of the field is `VAL8`"]
    #[inline]
    pub fn is_val8(&self) -> bool {
        *self == INPR::VAL8
    }
    #[doc = "Checks if the value of the field is `VAL9`"]
    #[inline]
    pub fn is_val9(&self) -> bool {
        *self == INPR::VAL9
    }
    #[doc = "Checks if the value of the field is `VAL10`"]
    #[inline]
    pub fn is_val10(&self) -> bool {
        *self == INPR::VAL10
    }
    #[doc = "Checks if the value of the field is `VAL11`"]
    #[inline]
    pub fn is_val11(&self) -> bool {
        *self == INPR::VAL11
    }
    #[doc = "Checks if the value of the field is `VAL12`"]
    #[inline]
    pub fn is_val12(&self) -> bool {
        *self == INPR::VAL12
    }
    #[doc = "Checks if the value of the field is `VAL13`"]
    #[inline]
    pub fn is_val13(&self) -> bool {
        *self == INPR::VAL13
    }
    #[doc = "Checks if the value of the field is `VAL14`"]
    #[inline]
    pub fn is_val14(&self) -> bool {
        *self == INPR::VAL14
    }
    #[doc = "Checks if the value of the field is `VAL15`"]
    #[inline]
    pub fn is_val15(&self) -> bool {
        *self == INPR::VAL15
    }
}
#[doc = "Values that can be written to the field `INP`"]
pub enum INPW {
    #[doc = "Pin interrupt 0"]
    VAL0,
    #[doc = "Pin interrupt 1"]
    VAL1,
    #[doc = "Pin interrupt 2"]
    VAL2,
    #[doc = "Pin interrupt 3"]
    VAL3,
    #[doc = "Timer CTIMER0 Match 0"]
    VAL4,
    #[doc = "Timer CTIMER0 Match 1"]
    VAL5,
    #[doc = "Timer CTIMER2 Match 0"]
    VAL6,
    #[doc = "Timer CTIMER4 Match 0"]
    VAL7,
    #[doc = "DMA1 output trigger mux 0"]
    VAL8,
    #[doc = "DMA1 output trigger mux 1"]
    VAL9,
    #[doc = "DMA1 output trigger mux 2"]
    VAL10,
    #[doc = "DMA1 output trigger mux 3"]
    VAL11,
    #[doc = "SCT0 DMA request 0"]
    VAL12,
    #[doc = "SCT0 DMA request 1"]
    VAL13,
    #[doc = "HASH DMA RX trigger"]
    VAL14,
    #[doc = "None"]
    VAL15,
}
impl INPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPW::VAL0 => 0,
            INPW::VAL1 => 1,
            INPW::VAL2 => 2,
            INPW::VAL3 => 3,
            INPW::VAL4 => 4,
            INPW::VAL5 => 5,
            INPW::VAL6 => 6,
            INPW::VAL7 => 7,
            INPW::VAL8 => 8,
            INPW::VAL9 => 9,
            INPW::VAL10 => 10,
            INPW::VAL11 => 11,
            INPW::VAL12 => 12,
            INPW::VAL13 => 13,
            INPW::VAL14 => 14,
            INPW::VAL15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPW<'a> {
    w: &'a mut W,
}
impl<'a> _INPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin interrupt 0"]
    #[inline]
    pub fn val0(self) -> &'a mut W {
        self.variant(INPW::VAL0)
    }
    #[doc = "Pin interrupt 1"]
    #[inline]
    pub fn val1(self) -> &'a mut W {
        self.variant(INPW::VAL1)
    }
    #[doc = "Pin interrupt 2"]
    #[inline]
    pub fn val2(self) -> &'a mut W {
        self.variant(INPW::VAL2)
    }
    #[doc = "Pin interrupt 3"]
    #[inline]
    pub fn val3(self) -> &'a mut W {
        self.variant(INPW::VAL3)
    }
    #[doc = "Timer CTIMER0 Match 0"]
    #[inline]
    pub fn val4(self) -> &'a mut W {
        self.variant(INPW::VAL4)
    }
    #[doc = "Timer CTIMER0 Match 1"]
    #[inline]
    pub fn val5(self) -> &'a mut W {
        self.variant(INPW::VAL5)
    }
    #[doc = "Timer CTIMER2 Match 0"]
    #[inline]
    pub fn val6(self) -> &'a mut W {
        self.variant(INPW::VAL6)
    }
    #[doc = "Timer CTIMER4 Match 0"]
    #[inline]
    pub fn val7(self) -> &'a mut W {
        self.variant(INPW::VAL7)
    }
    #[doc = "DMA1 output trigger mux 0"]
    #[inline]
    pub fn val8(self) -> &'a mut W {
        self.variant(INPW::VAL8)
    }
    #[doc = "DMA1 output trigger mux 1"]
    #[inline]
    pub fn val9(self) -> &'a mut W {
        self.variant(INPW::VAL9)
    }
    #[doc = "DMA1 output trigger mux 2"]
    #[inline]
    pub fn val10(self) -> &'a mut W {
        self.variant(INPW::VAL10)
    }
    #[doc = "DMA1 output trigger mux 3"]
    #[inline]
    pub fn val11(self) -> &'a mut W {
        self.variant(INPW::VAL11)
    }
    #[doc = "SCT0 DMA request 0"]
    #[inline]
    pub fn val12(self) -> &'a mut W {
        self.variant(INPW::VAL12)
    }
    #[doc = "SCT0 DMA request 1"]
    #[inline]
    pub fn val13(self) -> &'a mut W {
        self.variant(INPW::VAL13)
    }
    #[doc = "HASH DMA RX trigger"]
    #[inline]
    pub fn val14(self) -> &'a mut W {
        self.variant(INPW::VAL14)
    }
    #[doc = "None"]
    #[inline]
    pub fn val15(self) -> &'a mut W {
        self.variant(INPW::VAL15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline]
    pub fn inp(&self) -> INPR {
        INPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 15 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline]
    pub fn inp(&mut self) -> _INPW {
        _INPW { w: self }
    }
}