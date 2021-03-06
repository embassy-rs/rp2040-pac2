#[doc = "FIFO levels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flevel(pub u32);
impl Flevel {
    pub const fn tx0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    pub fn set_tx0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    pub const fn rx0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    pub fn set_rx0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    pub const fn tx1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    pub fn set_tx1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    pub const fn rx1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    pub fn set_rx1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    pub const fn tx2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    pub fn set_tx2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    pub const fn rx2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    pub fn set_rx2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    pub const fn tx3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    pub fn set_tx3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    pub const fn rx3(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    pub fn set_rx3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Flevel {
    fn default() -> Flevel {
        Flevel(0)
    }
}
#[doc = "FIFO status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstat(pub u32);
impl Fstat {
    #[doc = "State machine RX FIFO is full"]
    pub const fn rxfull(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine RX FIFO is full"]
    pub fn set_rxfull(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "State machine RX FIFO is empty"]
    pub const fn rxempty(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine RX FIFO is empty"]
    pub fn set_rxempty(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "State machine TX FIFO is full"]
    pub const fn txfull(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine TX FIFO is full"]
    pub fn set_txfull(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "State machine TX FIFO is empty"]
    pub const fn txempty(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine TX FIFO is empty"]
    pub fn set_txempty(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Fstat {
    fn default() -> Fstat {
        Fstat(0)
    }
}
#[doc = "Interrupt Enable for irq0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    pub const fn sm0_rxnempty(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_sm0_rxnempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    pub const fn sm1_rxnempty(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    pub fn set_sm1_rxnempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    pub const fn sm2_rxnempty(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    pub fn set_sm2_rxnempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    pub const fn sm3_rxnempty(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    pub fn set_sm3_rxnempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    pub const fn sm0_txnfull(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    pub fn set_sm0_txnfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    pub const fn sm1_txnfull(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    pub fn set_sm1_txnfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    pub const fn sm2_txnfull(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    pub fn set_sm2_txnfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    pub const fn sm3_txnfull(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    pub fn set_sm3_txnfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    pub const fn sm0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    pub fn set_sm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    pub const fn sm1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    pub fn set_sm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    pub const fn sm2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    pub fn set_sm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    pub const fn sm3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    pub fn set_sm3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Intr {
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Clock divider register for state machine 1 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmClkdiv(pub u32);
impl SmClkdiv {
    #[doc = "Fractional part of clock divider"]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Fractional part of clock divider"]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Effective frequency is sysclk/int. Value of 0 is interpreted as max possible value"]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Effective frequency is sysclk/int. Value of 0 is interpreted as max possible value"]
    pub fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SmClkdiv {
    fn default() -> SmClkdiv {
        SmClkdiv(0)
    }
}
#[doc = "FIFO debug register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdebug(pub u32);
impl Fdebug {
    #[doc = "State machine has stalled on full RX FIFO. Write 1 to clear."]
    pub const fn rxstall(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine has stalled on full RX FIFO. Write 1 to clear."]
    pub fn set_rxstall(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "RX FIFO underflow has occurred. Write 1 to clear."]
    pub const fn rxunder(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "RX FIFO underflow has occurred. Write 1 to clear."]
    pub fn set_rxunder(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "TX FIFO overflow has occurred. Write 1 to clear."]
    pub const fn txover(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "TX FIFO overflow has occurred. Write 1 to clear."]
    pub fn set_txover(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "State machine has stalled on empty TX FIFO. Write 1 to clear."]
    pub const fn txstall(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine has stalled on empty TX FIFO. Write 1 to clear."]
    pub fn set_txstall(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Fdebug {
    fn default() -> Fdebug {
        Fdebug(0)
    }
}
#[doc = "Instruction currently being executed by state machine 0 Write to execute an instruction immediately (including jumps) and then resume execution."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmInstr(pub u32);
impl SmInstr {
    pub const fn instr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    pub fn set_instr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SmInstr {
    fn default() -> SmInstr {
        SmInstr(0)
    }
}
#[doc = "The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgCfginfo(pub u32);
impl DbgCfginfo {
    #[doc = "The depth of the state machine TX/RX FIFOs, measured in words. Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double this depth."]
    pub const fn fifo_depth(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "The depth of the state machine TX/RX FIFOs, measured in words. Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double this depth."]
    pub fn set_fifo_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "The number of state machines this PIO instance is equipped with."]
    pub const fn sm_count(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "The number of state machines this PIO instance is equipped with."]
    pub fn set_sm_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "The size of the instruction memory, measured in units of one instruction"]
    pub const fn imem_size(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "The size of the instruction memory, measured in units of one instruction"]
    pub fn set_imem_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for DbgCfginfo {
    fn default() -> DbgCfginfo {
        DbgCfginfo(0)
    }
}
#[doc = "Current instruction address of state machine 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmAddr(pub u32);
impl SmAddr {
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    pub fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for SmAddr {
    fn default() -> SmAddr {
        SmAddr(0)
    }
}
#[doc = "PIO control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enable state machine"]
    pub const fn sm_enable(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Enable state machine"]
    pub fn set_sm_enable(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Clear internal SM state which is otherwise difficult to access (e.g. shift counters). Self-clearing."]
    pub const fn sm_restart(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear internal SM state which is otherwise difficult to access (e.g. shift counters). Self-clearing."]
    pub fn set_sm_restart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Force clock dividers to restart their count and clear fractional accumulators. Restart multiple dividers to synchronise them."]
    pub const fn clkdiv_restart(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Force clock dividers to restart their count and clear fractional accumulators. Restart multiple dividers to synchronise them."]
    pub fn set_clkdiv_restart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Ctrl {
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrqForce(pub u32);
impl IrqForce {
    pub const fn irq_force(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    pub fn set_irq_force(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for IrqForce {
    fn default() -> IrqForce {
        IrqForce(0)
    }
}
#[doc = "State machine pin control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmPinctrl(pub u32);
impl SmPinctrl {
    #[doc = "The virtual pin corresponding to OUT bit 0"]
    pub const fn out_base(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "The virtual pin corresponding to OUT bit 0"]
    pub fn set_out_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "The virtual pin corresponding to SET bit 0"]
    pub const fn set_base(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "The virtual pin corresponding to SET bit 0"]
    pub fn set_set_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "The virtual pin corresponding to delay field bit 0"]
    pub const fn sideset_base(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "The virtual pin corresponding to delay field bit 0"]
    pub fn set_sideset_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "The virtual pin corresponding to IN bit 0"]
    pub const fn in_base(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "The virtual pin corresponding to IN bit 0"]
    pub fn set_in_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "The number of pins asserted by an OUT. Value of 0 -> 32 pins"]
    pub const fn out_count(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "The number of pins asserted by an OUT. Value of 0 -> 32 pins"]
    pub fn set_out_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "The number of pins asserted by a SET. Max of 5"]
    pub const fn set_count(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "The number of pins asserted by a SET. Max of 5"]
    pub fn set_set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "The number of delay bits co-opted for side-set. Inclusive of the enable bit, if present."]
    pub const fn sideset_count(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "The number of delay bits co-opted for side-set. Inclusive of the enable bit, if present."]
    pub fn set_sideset_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for SmPinctrl {
    fn default() -> SmPinctrl {
        SmPinctrl(0)
    }
}
#[doc = "Write-only access to instruction memory location 10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InstrMem(pub u32);
impl InstrMem {
    pub const fn instr_mem(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    pub fn set_instr_mem(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for InstrMem {
    fn default() -> InstrMem {
        InstrMem(0)
    }
}
#[doc = "Execution/behavioural settings for state machine 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmExecctrl(pub u32);
impl SmExecctrl {
    #[doc = "Comparison level for the MOV x, STATUS instruction"]
    pub const fn status_n(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Comparison level for the MOV x, STATUS instruction"]
    pub fn set_status_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Comparison used for the MOV x, STATUS instruction."]
    pub const fn status_sel(&self) -> super::vals::SmExecctrlStatusSel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SmExecctrlStatusSel(val as u8)
    }
    #[doc = "Comparison used for the MOV x, STATUS instruction."]
    pub fn set_status_sel(&mut self, val: super::vals::SmExecctrlStatusSel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
    }
    #[doc = "After reaching wrap_top, execution is wrapped to this address."]
    pub const fn wrap_bottom(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x1f;
        val as u8
    }
    #[doc = "After reaching wrap_top, execution is wrapped to this address."]
    pub fn set_wrap_bottom(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
    }
    #[doc = "After reaching this address, execution is wrapped to wrap_bottom. If the instruction is a jump, and the jump condition is true, the jump takes priority."]
    pub const fn wrap_top(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "After reaching this address, execution is wrapped to wrap_bottom. If the instruction is a jump, and the jump condition is true, the jump takes priority."]
    pub fn set_wrap_top(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Continuously assert the most recent OUT/SET to the pins"]
    pub const fn out_sticky(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Continuously assert the most recent OUT/SET to the pins"]
    pub fn set_out_sticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "If 1, use a bit of OUT data as an auxiliary write enable When used in conjunction with OUT_STICKY, writes with an enable of 0 will deassert the latest pin write. This can create useful masking/override behaviour due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)"]
    pub const fn inline_out_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, use a bit of OUT data as an auxiliary write enable When used in conjunction with OUT_STICKY, writes with an enable of 0 will deassert the latest pin write. This can create useful masking/override behaviour due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)"]
    pub fn set_inline_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Which data bit to use for inline OUT enable"]
    pub const fn out_en_sel(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "Which data bit to use for inline OUT enable"]
    pub fn set_out_en_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
    pub const fn jmp_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
    pub fn set_jmp_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Side-set data is asserted to pin OEs instead of pin values"]
    pub const fn side_pindir(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Side-set data is asserted to pin OEs instead of pin values"]
    pub fn set_side_pindir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "If 1, the delay MSB is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction."]
    pub const fn side_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the delay MSB is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction."]
    pub fn set_side_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "An instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear once the instruction completes."]
    pub const fn exec_stalled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "An instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear once the instruction completes."]
    pub fn set_exec_stalled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SmExecctrl {
    fn default() -> SmExecctrl {
        SmExecctrl(0)
    }
}
#[doc = "Control behaviour of the input/output shift registers for state machine 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmShiftctrl(pub u32);
impl SmShiftctrl {
    #[doc = "Push automatically when the input shift register is filled"]
    pub const fn autopush(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Push automatically when the input shift register is filled"]
    pub fn set_autopush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pull automatically when the output shift register is emptied"]
    pub const fn autopull(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pull automatically when the output shift register is emptied"]
    pub fn set_autopull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "1 = shift input shift register to right (data enters from left). 0 = to left."]
    pub const fn in_shiftdir(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "1 = shift input shift register to right (data enters from left). 0 = to left."]
    pub fn set_in_shiftdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "1 = shift out of output shift register to right. 0 = to left."]
    pub const fn out_shiftdir(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "1 = shift out of output shift register to right. 0 = to left."]
    pub fn set_out_shiftdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Number of bits shifted into RXSR before autopush or conditional push. Write 0 for value of 32."]
    pub const fn push_thresh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of bits shifted into RXSR before autopush or conditional push. Write 0 for value of 32."]
    pub fn set_push_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[doc = "Number of bits shifted out of TXSR before autopull or conditional pull. Write 0 for value of 32."]
    pub const fn pull_thresh(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of bits shifted out of TXSR before autopull or conditional pull. Write 0 for value of 32."]
    pub fn set_pull_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
    #[doc = "When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep. RX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    pub const fn fjoin_tx(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep. RX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    pub fn set_fjoin_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep. TX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    pub const fn fjoin_rx(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep. TX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    pub fn set_fjoin_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SmShiftctrl {
    fn default() -> SmShiftctrl {
        SmShiftctrl(0)
    }
}
#[doc = "Interrupt request register. Write 1 to clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq(pub u32);
impl Irq {
    pub const fn irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    pub fn set_irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Irq {
    fn default() -> Irq {
        Irq(0)
    }
}
