#[doc = "Register `IC_DATA_CMD` reader"]
pub struct R(crate::R<IC_DATA_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_DATA_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_DATA_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_DATA_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_DATA_CMD` writer"]
pub struct W(crate::W<IC_DATA_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_DATA_CMD_SPEC>;
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
impl From<crate::W<IC_DATA_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_DATA_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.  

 Reset value: 0x0"]
pub type DAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAT` writer - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.  

 Reset value: 0x0"]
pub type DAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IC_DATA_CMD_SPEC, u8, u8, 8, O>;
#[doc = "Field `CMD` reader - This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.  

 When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.  

 When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.  

 Reset value: 0x0"]
pub type CMD_R = crate::BitReader<CMD_A>;
#[doc = "This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.  

 When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.  

 When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_A {
    #[doc = "0: Master Write Command"]
    WRITE = 0,
    #[doc = "1: Master Read Command"]
    READ = 1,
}
impl From<CMD_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as u8 != 0
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_A {
        match self.bits {
            false => CMD_A::WRITE,
            true => CMD_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == CMD_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == CMD_A::READ
    }
}
#[doc = "Field `CMD` writer - This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.  

 When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.  

 When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.  

 Reset value: 0x0"]
pub type CMD_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, IC_DATA_CMD_SPEC, CMD_A, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "Master Write Command"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMD_A::WRITE)
    }
    #[doc = "Master Read Command"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMD_A::READ)
    }
}
#[doc = "Field `STOP` reader - This bit controls whether a STOP is issued after the byte is sent or received.  

 - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0"]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "This bit controls whether a STOP is issued after the byte is sent or received.  

 - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: Don't Issue STOP after this command"]
    DISABLE = 0,
    #[doc = "1: Issue STOP after this command"]
    ENABLE = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::DISABLE,
            true => STOP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STOP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STOP_A::ENABLE
    }
}
#[doc = "Field `STOP` writer - This bit controls whether a STOP is issued after the byte is sent or received.  

 - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, IC_DATA_CMD_SPEC, STOP_A, O>;
impl<'a, const O: u8> STOP_W<'a, O> {
    #[doc = "Don't Issue STOP after this command"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STOP_A::DISABLE)
    }
    #[doc = "Issue STOP after this command"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STOP_A::ENABLE)
    }
}
#[doc = "Field `RESTART` reader - This bit controls whether a RESTART is issued before the byte is sent or received.  

 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 Reset value: 0x0"]
pub type RESTART_R = crate::BitReader<RESTART_A>;
#[doc = "This bit controls whether a RESTART is issued before the byte is sent or received.  

 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESTART_A {
    #[doc = "0: Don't Issue RESTART before this command"]
    DISABLE = 0,
    #[doc = "1: Issue RESTART before this command"]
    ENABLE = 1,
}
impl From<RESTART_A> for bool {
    #[inline(always)]
    fn from(variant: RESTART_A) -> Self {
        variant as u8 != 0
    }
}
impl RESTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESTART_A {
        match self.bits {
            false => RESTART_A::DISABLE,
            true => RESTART_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RESTART_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RESTART_A::ENABLE
    }
}
#[doc = "Field `RESTART` writer - This bit controls whether a RESTART is issued before the byte is sent or received.  

 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 Reset value: 0x0"]
pub type RESTART_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, IC_DATA_CMD_SPEC, RESTART_A, O>;
impl<'a, const O: u8> RESTART_W<'a, O> {
    #[doc = "Don't Issue RESTART before this command"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESTART_A::DISABLE)
    }
    #[doc = "Issue RESTART before this command"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RESTART_A::ENABLE)
    }
}
#[doc = "Field `FIRST_DATA_BYTE` reader - Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode.  

 Reset value : 0x0  

 NOTE: In case of APB_DATA_WIDTH=8,  

 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit.  

 2. In order to read the 11 bit, the user has to perform the first data byte read \\[7:0\\]
(offset 0x10) and then perform the second read \\[15:8\\]
(offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not).  

 3. The 11th bit is an optional read field, user can ignore 2nd byte read \\[15:8\\]
(offset 0x11) if not interested in FIRST_DATA_BYTE status."]
pub type FIRST_DATA_BYTE_R = crate::BitReader<FIRST_DATA_BYTE_A>;
#[doc = "Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode.  

 Reset value : 0x0  

 NOTE: In case of APB_DATA_WIDTH=8,  

 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit.  

 2. In order to read the 11 bit, the user has to perform the first data byte read \\[7:0\\]
(offset 0x10) and then perform the second read \\[15:8\\]
(offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not).  

 3. The 11th bit is an optional read field, user can ignore 2nd byte read \\[15:8\\]
(offset 0x11) if not interested in FIRST_DATA_BYTE status.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIRST_DATA_BYTE_A {
    #[doc = "0: Sequential data byte received"]
    INACTIVE = 0,
    #[doc = "1: Non sequential data byte received"]
    ACTIVE = 1,
}
impl From<FIRST_DATA_BYTE_A> for bool {
    #[inline(always)]
    fn from(variant: FIRST_DATA_BYTE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIRST_DATA_BYTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIRST_DATA_BYTE_A {
        match self.bits {
            false => FIRST_DATA_BYTE_A::INACTIVE,
            true => FIRST_DATA_BYTE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == FIRST_DATA_BYTE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == FIRST_DATA_BYTE_A::ACTIVE
    }
}
impl R {
    #[doc = "Bits 0:7 - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.  

 When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.  

 When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit controls whether a STOP is issued after the byte is sent or received.  

 - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit controls whether a RESTART is issued before the byte is sent or received.  

 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode.  

 Reset value : 0x0  

 NOTE: In case of APB_DATA_WIDTH=8,  

 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit.  

 2. In order to read the 11 bit, the user has to perform the first data byte read \\[7:0\\]
(offset 0x10) and then perform the second read \\[15:8\\]
(offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not).  

 3. The 11th bit is an optional read field, user can ignore 2nd byte read \\[15:8\\]
(offset 0x11) if not interested in FIRST_DATA_BYTE status."]
    #[inline(always)]
    pub fn first_data_byte(&self) -> FIRST_DATA_BYTE_R {
        FIRST_DATA_BYTE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn dat(&mut self) -> DAT_W<0> {
        DAT_W::new(self)
    }
    #[doc = "Bit 8 - This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master.  

 When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted.  

 When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W<8> {
        CMD_W::new(self)
    }
    #[doc = "Bit 9 - This bit controls whether a STOP is issued after the byte is sent or received.  

 - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<9> {
        STOP_W::new(self)
    }
    #[doc = "Bit 10 - This bit controls whether a RESTART is issued before the byte is sent or received.  

 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W<10> {
        RESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO.  

 The size of the register changes as follows:  

 Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_data_cmd](index.html) module"]
pub struct IC_DATA_CMD_SPEC;
impl crate::RegisterSpec for IC_DATA_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_data_cmd::R](R) reader structure"]
impl crate::Readable for IC_DATA_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_data_cmd::W](W) writer structure"]
impl crate::Writable for IC_DATA_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_DATA_CMD to value 0"]
impl crate::Resettable for IC_DATA_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
