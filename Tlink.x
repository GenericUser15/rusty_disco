/* Memory layout for the F303VCT6 */
MEMORY
{
  FLASH(rx) : ORIGIN = 0x08000000, LENGTH = 256K
  RAM(rwx) : ORIGIN = 0x20000000, LENGTH = 40K
}

/* Entry point */
ENTRY(Reset);
EXTERN(__RESET_VECTOR);
EXTERN(__EXCEPTIONS);
EXTERN(__INTERRUPTS);

PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));
PROVIDE(NMI = DefaultExceptionHandler);
PROVIDE(HardFault = DefaultExceptionHandler);
PROVIDE(MemManage = DefaultExceptionHandler);
PROVIDE(BusFault = DefaultExceptionHandler);
PROVIDE(UsageFault = DefaultExceptionHandler);
PROVIDE(SVCall = DefaultExceptionHandler);
PROVIDE(PendSV = DefaultExceptionHandler);
PROVIDE(SysTick = DefaultExceptionHandler);
PROVIDE(WWDG = DefaultInterruptHandler);
PROVIDE(PVD = DefaultInterruptHandler);
PROVIDE(TAMPER_STAMP = DefaultInterruptHandler);
PROVIDE(RTC_WKUP = DefaultInterruptHandler);
PROVIDE(FLASH = DefaultInterruptHandler);
PROVIDE(RCC = DefaultInterruptHandler);
PROVIDE(EXTI0 = DefaultInterruptHandler);
PROVIDE(EXTI1 = DefaultInterruptHandler);
PROVIDE(EXTI2_TS = DefaultInterruptHandler);
PROVIDE(EXTI3 = DefaultInterruptHandler);
PROVIDE(EXTI4 = DefaultInterruptHandler);
PROVIDE(DMA1_CHANNEL1 = DefaultInterruptHandler);
PROVIDE(DMA1_CHANNEL2 = DefaultInterruptHandler);
PROVIDE(DMA1_CHANNEL3 = DefaultInterruptHandler);
PROVIDE(DMA1_CHANNEL4 = DefaultInterruptHandler);
PROVIDE(DMA1_CHANNEL5 = DefaultInterruptHandler);
PROVIDE(DMA1_CHANNEL6 = DefaultInterruptHandler);
PROVIDE(DMA1_CHANNEL7 = DefaultInterruptHandler);
PROVIDE(ADC1_2 = DefaultInterruptHandler);
PROVIDE(USB_HP_CAN_TX = DefaultInterruptHandler);
PROVIDE(USB_LP_CAN_RX0 = DefaultInterruptHandler);
PROVIDE(CAN_RX1 = DefaultInterruptHandler);
PROVIDE(CAN_SCE = DefaultInterruptHandler);
PROVIDE(EXTI9_5 = DefaultInterruptHandler);
PROVIDE(TIM1_BRK_TIM15 = DefaultInterruptHandler);
PROVIDE(TIM1_UP_TIM16 = DefaultInterruptHandler);
PROVIDE(TIM1_RTG_COM_TIM17 = DefaultInterruptHandler);
PROVIDE(TIM1_CC = DefaultInterruptHandler);
PROVIDE(TIM2 = DefaultInterruptHandler);
PROVIDE(TIM3 = DefaultInterruptHandler);
PROVIDE(TIM4 = DefaultInterruptHandler);
PROVIDE(I2C1_EV = DefaultInterruptHandler);
PROVIDE(I2C1_ER = DefaultInterruptHandler);
PROVIDE(I2C2_EV = DefaultInterruptHandler);
PROVIDE(I2C2_ER = DefaultInterruptHandler);
PROVIDE(SPI1 = DefaultInterruptHandler);
PROVIDE(SPI2 = DefaultInterruptHandler);
PROVIDE(USART1 = DefaultInterruptHandler);
PROVIDE(USART2 = DefaultInterruptHandler);
PROVIDE(USART3 = DefaultInterruptHandler);
PROVIDE(EXTI15_10 = DefaultInterruptHandler);
PROVIDE(RTC_Alarm = DefaultInterruptHandler);
PROVIDE(USBWakeUp = DefaultInterruptHandler);
PROVIDE(TIM8_BRK = DefaultInterruptHandler);
PROVIDE(TIM8_UP = DefaultInterruptHandler);
PROVIDE(TIM8_TRIG_COM = DefaultInterruptHandler);
PROVIDE(TIM8_CC = DefaultInterruptHandler);
PROVIDE(ADC3 = DefaultInterruptHandler);
PROVIDE(FMC = DefaultInterruptHandler);
PROVIDE(SPI3 = DefaultInterruptHandler);
PROVIDE(UART4 = DefaultInterruptHandler);
PROVIDE(UART5 = DefaultInterruptHandler);
PROVIDE(TIM6_DAC = DefaultInterruptHandler);
PROVIDE(TIM7 = DefaultInterruptHandler);
PROVIDE(DMA2_CHANNEL1 = DefaultInterruptHandler);
PROVIDE(DMA2_CHANNEL2 = DefaultInterruptHandler);
PROVIDE(DMA2_CHANNEL3 = DefaultInterruptHandler);
PROVIDE(DMA2_CHANNEL4 = DefaultInterruptHandler);
PROVIDE(DMA2_CHANNEL5 = DefaultInterruptHandler);
PROVIDE(ADC4 = DefaultInterruptHandler);
PROVIDE(COMP1_2_3 = DefaultInterruptHandler);
PROVIDE(COMP4_5_6 = DefaultInterruptHandler);
PROVIDE(COMP7 = DefaultInterruptHandler);
PROVIDE(I2C3_EV = DefaultInterruptHandler);
PROVIDE(I2C3_ER = DefaultInterruptHandler);
PROVIDE(USB_HP = DefaultInterruptHandler);
PROVIDE(USB_LP = DefaultInterruptHandler);
PROVIDE(USB_WakeUp_RMP = DefaultInterruptHandler);
PROVIDE(TIM20_BRK = DefaultInterruptHandler);
PROVIDE(TIM20_UP = DefaultInterruptHandler);
PROVIDE(TIM20_TRG_COM = DefaultInterruptHandler);
PROVIDE(TIM20_CC = DefaultInterruptHandler);
PROVIDE(FPU = DefaultInterruptHandler);
PROVIDE(SPI4 = DefaultInterruptHandler);

SECTIONS
{
	/* Vector table must always be first */
	.vector_table ORIGIN(FLASH) : ALIGN(4)
	{
		__STACK_START = .;
		LONG(_stack_start);

		KEEP(*(.vector_table.reset_vector));
		__reset_vector = ABSOLUTE(.);

		KEEP(*(.vector_table.exceptions));
		__eexceptions = ABSOLUTE(.);

		KEEP(*(.vector_table.interrupts));
		__einterupts = ABSOLUTE(.);
	} > FLASH

	.text :
	{
		/* Merge text sections of all input files. */
		/* e.g., merge: led.o, startup.o etc */
		*(.text .text.*);
		__etext = ABSOLUTE(.);
	} > FLASH

	.rodata :
	{
		. = ALIGN(4);
		*(.rodata .rodata.*);
		. = ALIGN(4);
		__erodata = ABSOLUTE(.);
	} > FLASH

	.data : AT(__erodata)
	{
		. = ALIGN(4);
		__sdata = ABSOLUTE(.);

		*(.data . data.*);

		.ALIGN(4);
		__edata = ABSOLUTE(.);
	} > RAM

	.bss :
	{
		. = ALIGN(4);
		__sbss = ABSOLUTE(.);

		*(.bss .bss.*);
		
		. = ALIGN(4);
		__ebss = .;
	} > RAM

	__sidata = LOADADDR(.data);
	
	/DISCARD/ :
	{
		*(.ARM.exidx .ARM.exidx.*);
	}
}

