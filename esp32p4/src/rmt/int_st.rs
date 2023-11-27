#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `CH0_TX_END_INT_ST` reader - The masked interrupt status bit for CH0_TX_END_INT."]
pub type CH0_TX_END_INT_ST_R = crate::BitReader;
#[doc = "Field `CH1_TX_END_INT_ST` reader - The masked interrupt status bit for CH1_TX_END_INT."]
pub type CH1_TX_END_INT_ST_R = crate::BitReader;
#[doc = "Field `CH2_TX_END_INT_ST` reader - The masked interrupt status bit for CH2_TX_END_INT."]
pub type CH2_TX_END_INT_ST_R = crate::BitReader;
#[doc = "Field `CH3_TX_END_INT_ST` reader - The masked interrupt status bit for CH3_TX_END_INT."]
pub type CH3_TX_END_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_CH0_ERR_INT_ST` reader - The masked interrupt status bit for CH0_ERR_INT."]
pub type TX_CH0_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_CH1_ERR_INT_ST` reader - The masked interrupt status bit for CH1_ERR_INT."]
pub type TX_CH1_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_CH2_ERR_INT_ST` reader - The masked interrupt status bit for CH2_ERR_INT."]
pub type TX_CH2_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_CH3_ERR_INT_ST` reader - The masked interrupt status bit for CH3_ERR_INT."]
pub type TX_CH3_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `CH0_TX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH0_TX_THR_EVENT_INT."]
pub type CH0_TX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `CH1_TX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH1_TX_THR_EVENT_INT."]
pub type CH1_TX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `CH2_TX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH2_TX_THR_EVENT_INT."]
pub type CH2_TX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `CH3_TX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH3_TX_THR_EVENT_INT."]
pub type CH3_TX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `CH0_TX_LOOP_INT_ST` reader - The masked interrupt status bit for CH0_TX_LOOP_INT."]
pub type CH0_TX_LOOP_INT_ST_R = crate::BitReader;
#[doc = "Field `CH1_TX_LOOP_INT_ST` reader - The masked interrupt status bit for CH1_TX_LOOP_INT."]
pub type CH1_TX_LOOP_INT_ST_R = crate::BitReader;
#[doc = "Field `CH2_TX_LOOP_INT_ST` reader - The masked interrupt status bit for CH2_TX_LOOP_INT."]
pub type CH2_TX_LOOP_INT_ST_R = crate::BitReader;
#[doc = "Field `CH3_TX_LOOP_INT_ST` reader - The masked interrupt status bit for CH3_TX_LOOP_INT."]
pub type CH3_TX_LOOP_INT_ST_R = crate::BitReader;
#[doc = "Field `CH4_RX_END_INT_ST` reader - The masked interrupt status bit for CH4_RX_END_INT."]
pub type CH4_RX_END_INT_ST_R = crate::BitReader;
#[doc = "Field `CH5_RX_END_INT_ST` reader - The masked interrupt status bit for CH5_RX_END_INT."]
pub type CH5_RX_END_INT_ST_R = crate::BitReader;
#[doc = "Field `CH6_RX_END_INT_ST` reader - The masked interrupt status bit for CH6_RX_END_INT."]
pub type CH6_RX_END_INT_ST_R = crate::BitReader;
#[doc = "Field `CH7_RX_END_INT_ST` reader - The masked interrupt status bit for CH7_RX_END_INT."]
pub type CH7_RX_END_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_CH4_ERR_INT_ST` reader - The masked interrupt status bit for CH4_ERR_INT."]
pub type RX_CH4_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_CH5_ERR_INT_ST` reader - The masked interrupt status bit for CH5_ERR_INT."]
pub type RX_CH5_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_CH6_ERR_INT_ST` reader - The masked interrupt status bit for CH6_ERR_INT."]
pub type RX_CH6_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_CH7_ERR_INT_ST` reader - The masked interrupt status bit for CH7_ERR_INT."]
pub type RX_CH7_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `CH4_RX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH4_RX_THR_EVENT_INT."]
pub type CH4_RX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `CH5_RX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH5_RX_THR_EVENT_INT."]
pub type CH5_RX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `CH6_RX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH6_RX_THR_EVENT_INT."]
pub type CH6_RX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `CH7_RX_THR_EVENT_INT_ST` reader - The masked interrupt status bit for CH7_RX_THR_EVENT_INT."]
pub type CH7_RX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_ST` reader - The masked interrupt status bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_ST` reader - The masked interrupt status bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end_int_st(&self) -> CH0_TX_END_INT_ST_R {
        CH0_TX_END_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end_int_st(&self) -> CH1_TX_END_INT_ST_R {
        CH1_TX_END_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end_int_st(&self) -> CH2_TX_END_INT_ST_R {
        CH2_TX_END_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end_int_st(&self) -> CH3_TX_END_INT_ST_R {
        CH3_TX_END_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch0_err_int_st(&self) -> TX_CH0_ERR_INT_ST_R {
        TX_CH0_ERR_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch1_err_int_st(&self) -> TX_CH1_ERR_INT_ST_R {
        TX_CH1_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch2_err_int_st(&self) -> TX_CH2_ERR_INT_ST_R {
        TX_CH2_ERR_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch3_err_int_st(&self) -> TX_CH3_ERR_INT_ST_R {
        TX_CH3_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_st(&self) -> CH0_TX_THR_EVENT_INT_ST_R {
        CH0_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_st(&self) -> CH1_TX_THR_EVENT_INT_ST_R {
        CH1_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_st(&self) -> CH2_TX_THR_EVENT_INT_ST_R {
        CH2_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_st(&self) -> CH3_TX_THR_EVENT_INT_ST_R {
        CH3_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The masked interrupt status bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_st(&self) -> CH0_TX_LOOP_INT_ST_R {
        CH0_TX_LOOP_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The masked interrupt status bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_st(&self) -> CH1_TX_LOOP_INT_ST_R {
        CH1_TX_LOOP_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The masked interrupt status bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_st(&self) -> CH2_TX_LOOP_INT_ST_R {
        CH2_TX_LOOP_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The masked interrupt status bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_st(&self) -> CH3_TX_LOOP_INT_ST_R {
        CH3_TX_LOOP_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The masked interrupt status bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch4_rx_end_int_st(&self) -> CH4_RX_END_INT_ST_R {
        CH4_RX_END_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The masked interrupt status bit for CH5_RX_END_INT."]
    #[inline(always)]
    pub fn ch5_rx_end_int_st(&self) -> CH5_RX_END_INT_ST_R {
        CH5_RX_END_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The masked interrupt status bit for CH6_RX_END_INT."]
    #[inline(always)]
    pub fn ch6_rx_end_int_st(&self) -> CH6_RX_END_INT_ST_R {
        CH6_RX_END_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The masked interrupt status bit for CH7_RX_END_INT."]
    #[inline(always)]
    pub fn ch7_rx_end_int_st(&self) -> CH7_RX_END_INT_ST_R {
        CH7_RX_END_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The masked interrupt status bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn rx_ch4_err_int_st(&self) -> RX_CH4_ERR_INT_ST_R {
        RX_CH4_ERR_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The masked interrupt status bit for CH5_ERR_INT."]
    #[inline(always)]
    pub fn rx_ch5_err_int_st(&self) -> RX_CH5_ERR_INT_ST_R {
        RX_CH5_ERR_INT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The masked interrupt status bit for CH6_ERR_INT."]
    #[inline(always)]
    pub fn rx_ch6_err_int_st(&self) -> RX_CH6_ERR_INT_ST_R {
        RX_CH6_ERR_INT_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The masked interrupt status bit for CH7_ERR_INT."]
    #[inline(always)]
    pub fn rx_ch7_err_int_st(&self) -> RX_CH7_ERR_INT_ST_R {
        RX_CH7_ERR_INT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The masked interrupt status bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch4_rx_thr_event_int_st(&self) -> CH4_RX_THR_EVENT_INT_ST_R {
        CH4_RX_THR_EVENT_INT_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The masked interrupt status bit for CH5_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch5_rx_thr_event_int_st(&self) -> CH5_RX_THR_EVENT_INT_ST_R {
        CH5_RX_THR_EVENT_INT_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The masked interrupt status bit for CH6_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch6_rx_thr_event_int_st(&self) -> CH6_RX_THR_EVENT_INT_ST_R {
        CH6_RX_THR_EVENT_INT_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The masked interrupt status bit for CH7_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch7_rx_thr_event_int_st(&self) -> CH7_RX_THR_EVENT_INT_ST_R {
        CH7_RX_THR_EVENT_INT_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The masked interrupt status bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail_int_st(&self) -> TX_CH3_DMA_ACCESS_FAIL_INT_ST_R {
        TX_CH3_DMA_ACCESS_FAIL_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The masked interrupt status bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail_int_st(&self) -> RX_CH7_DMA_ACCESS_FAIL_INT_ST_R {
        RX_CH7_DMA_ACCESS_FAIL_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "ch0_tx_end_int_st",
                &format_args!("{}", self.ch0_tx_end_int_st().bit()),
            )
            .field(
                "ch1_tx_end_int_st",
                &format_args!("{}", self.ch1_tx_end_int_st().bit()),
            )
            .field(
                "ch2_tx_end_int_st",
                &format_args!("{}", self.ch2_tx_end_int_st().bit()),
            )
            .field(
                "ch3_tx_end_int_st",
                &format_args!("{}", self.ch3_tx_end_int_st().bit()),
            )
            .field(
                "tx_ch0_err_int_st",
                &format_args!("{}", self.tx_ch0_err_int_st().bit()),
            )
            .field(
                "tx_ch1_err_int_st",
                &format_args!("{}", self.tx_ch1_err_int_st().bit()),
            )
            .field(
                "tx_ch2_err_int_st",
                &format_args!("{}", self.tx_ch2_err_int_st().bit()),
            )
            .field(
                "tx_ch3_err_int_st",
                &format_args!("{}", self.tx_ch3_err_int_st().bit()),
            )
            .field(
                "ch0_tx_thr_event_int_st",
                &format_args!("{}", self.ch0_tx_thr_event_int_st().bit()),
            )
            .field(
                "ch1_tx_thr_event_int_st",
                &format_args!("{}", self.ch1_tx_thr_event_int_st().bit()),
            )
            .field(
                "ch2_tx_thr_event_int_st",
                &format_args!("{}", self.ch2_tx_thr_event_int_st().bit()),
            )
            .field(
                "ch3_tx_thr_event_int_st",
                &format_args!("{}", self.ch3_tx_thr_event_int_st().bit()),
            )
            .field(
                "ch0_tx_loop_int_st",
                &format_args!("{}", self.ch0_tx_loop_int_st().bit()),
            )
            .field(
                "ch1_tx_loop_int_st",
                &format_args!("{}", self.ch1_tx_loop_int_st().bit()),
            )
            .field(
                "ch2_tx_loop_int_st",
                &format_args!("{}", self.ch2_tx_loop_int_st().bit()),
            )
            .field(
                "ch3_tx_loop_int_st",
                &format_args!("{}", self.ch3_tx_loop_int_st().bit()),
            )
            .field(
                "ch4_rx_end_int_st",
                &format_args!("{}", self.ch4_rx_end_int_st().bit()),
            )
            .field(
                "ch5_rx_end_int_st",
                &format_args!("{}", self.ch5_rx_end_int_st().bit()),
            )
            .field(
                "ch6_rx_end_int_st",
                &format_args!("{}", self.ch6_rx_end_int_st().bit()),
            )
            .field(
                "ch7_rx_end_int_st",
                &format_args!("{}", self.ch7_rx_end_int_st().bit()),
            )
            .field(
                "rx_ch4_err_int_st",
                &format_args!("{}", self.rx_ch4_err_int_st().bit()),
            )
            .field(
                "rx_ch5_err_int_st",
                &format_args!("{}", self.rx_ch5_err_int_st().bit()),
            )
            .field(
                "rx_ch6_err_int_st",
                &format_args!("{}", self.rx_ch6_err_int_st().bit()),
            )
            .field(
                "rx_ch7_err_int_st",
                &format_args!("{}", self.rx_ch7_err_int_st().bit()),
            )
            .field(
                "ch4_rx_thr_event_int_st",
                &format_args!("{}", self.ch4_rx_thr_event_int_st().bit()),
            )
            .field(
                "ch5_rx_thr_event_int_st",
                &format_args!("{}", self.ch5_rx_thr_event_int_st().bit()),
            )
            .field(
                "ch6_rx_thr_event_int_st",
                &format_args!("{}", self.ch6_rx_thr_event_int_st().bit()),
            )
            .field(
                "ch7_rx_thr_event_int_st",
                &format_args!("{}", self.ch7_rx_thr_event_int_st().bit()),
            )
            .field(
                "tx_ch3_dma_access_fail_int_st",
                &format_args!("{}", self.tx_ch3_dma_access_fail_int_st().bit()),
            )
            .field(
                "rx_ch7_dma_access_fail_int_st",
                &format_args!("{}", self.rx_ch7_dma_access_fail_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
