initSidebarItems({"enum":[["IC_10BITADDR_MASTER_A","Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing"],["IC_10BITADDR_SLAVE_A","When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register."],["IC_RESTART_EN_A","Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register."],["IC_SLAVE_DISABLE_A","This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled."],["MASTER_MODE_A","This bit controls whether the DW_apb_i2c master is enabled."],["RX_FIFO_FULL_HLD_CTRL_A","This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter."],["SPEED_A","These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode."],["STOP_DET_IFADDRESSED_A","In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0"],["TX_EMPTY_CTRL_A","This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register."]],"struct":[["IC_10BITADDR_MASTER_W","Write proxy for field `IC_10BITADDR_MASTER`"],["IC_10BITADDR_SLAVE_W","Write proxy for field `IC_10BITADDR_SLAVE`"],["IC_RESTART_EN_W","Write proxy for field `IC_RESTART_EN`"],["IC_SLAVE_DISABLE_W","Write proxy for field `IC_SLAVE_DISABLE`"],["MASTER_MODE_W","Write proxy for field `MASTER_MODE`"],["RX_FIFO_FULL_HLD_CTRL_W","Write proxy for field `RX_FIFO_FULL_HLD_CTRL`"],["SPEED_W","Write proxy for field `SPEED`"],["STOP_DET_IFADDRESSED_W","Write proxy for field `STOP_DET_IFADDRESSED`"],["TX_EMPTY_CTRL_W","Write proxy for field `TX_EMPTY_CTRL`"]],"type":[["IC_10BITADDR_MASTER_R","Reader of field `IC_10BITADDR_MASTER`"],["IC_10BITADDR_SLAVE_R","Reader of field `IC_10BITADDR_SLAVE`"],["IC_RESTART_EN_R","Reader of field `IC_RESTART_EN`"],["IC_SLAVE_DISABLE_R","Reader of field `IC_SLAVE_DISABLE`"],["MASTER_MODE_R","Reader of field `MASTER_MODE`"],["R","Reader of register IC_CON"],["RX_FIFO_FULL_HLD_CTRL_R","Reader of field `RX_FIFO_FULL_HLD_CTRL`"],["SPEED_R","Reader of field `SPEED`"],["STOP_DET_IFADDRESSED_R","Reader of field `STOP_DET_IFADDRESSED`"],["STOP_DET_IF_MASTER_ACTIVE_R","Reader of field `STOP_DET_IF_MASTER_ACTIVE`"],["TX_EMPTY_CTRL_R","Reader of field `TX_EMPTY_CTRL`"],["W","Writer for register IC_CON"]]});