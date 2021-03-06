#[doc = "Reader of register CCBUF_DITH5_MODE[%s]"]
pub type R = crate::R<u32, super::CCBUF_DITH5_MODE>;
#[doc = "Writer for register CCBUF_DITH5_MODE[%s]"]
pub type W = crate::W<u32, super::CCBUF_DITH5_MODE>;
#[doc = "Register CCBUF_DITH5_MODE[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CCBUF_DITH5_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DITHERBUF`"]
pub type DITHERBUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DITHERBUF`"]
pub struct DITHERBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHERBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CCBUF`"]
pub type CCBUF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CCBUF`"]
pub struct CCBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 5)) | (((value as u32) & 0x0007_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&self) -> DITHERBUF_R {
        DITHERBUF_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CCBUF_R {
        CCBUF_R::new(((self.bits >> 5) & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dithering Buffer Cycle Number"]
    #[inline(always)]
    pub fn ditherbuf(&mut self) -> DITHERBUF_W {
        DITHERBUF_W { w: self }
    }
    #[doc = "Bits 5:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&mut self) -> CCBUF_W {
        CCBUF_W { w: self }
    }
}
