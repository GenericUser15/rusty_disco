/* Memory layout for the F303VCT6 */
MEMORY
{
  FLASH(rx) : ORIGIN = 0x08000000, LENGTH = 256K
  RAM(rwx) : ORIGIN = 0x20000000, LENGTH = 40K
}

/* Entry point */
ENTRY(Reset);
EXTERN(__RESET_VECTOR);

PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

SECTIONS
{
	/* Vector table must always be first */
	.vector_table ORIGIN(FLASH) : ALIGN(4)
	{
		__STACK_START = .;
		LONG(_stack_start);

		KEEP(*(.vector_table.reset_vector));
		__reset_vector = ABSOLUTE(.);
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

