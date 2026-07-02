/* Linker script cho STM32F411 (Cortex-M4F).
 * Được dùng thay cho link.x của cortex-m-rt.
 * Bố cục vùng nhớ (FLASH/RAM) lấy từ memory.x.
 */

INCLUDE memory.x

/* Điểm vào của chương trình sau reset. */
ENTRY(Reset);

/* Giữ lại các bảng vector kể cả khi không được tham chiếu trực tiếp. */
EXTERN(__RESET_VECTOR);
EXTERN(__EXCEPTIONS);
EXTERN(__INTERRUPTS);

/* Đỉnh stack = cuối RAM (stack lớn dần xuống). */
PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

SECTIONS
{
  /* Bảng vector đặt ngay đầu FLASH. */
  .vector_table ORIGIN(FLASH) :
  {
    __vector_table = .;

    /* Word đầu tiên: giá trị nạp vào MSP khi reset. */
    LONG(_stack_start & 0xFFFFFFF8);

    /* Word thứ hai: địa chỉ Reset handler. */
    KEEP(*(.vector_table.reset_vector));

    /* 14 exception (vị trí 2..15). */
    KEEP(*(.vector_table.exceptions));

    /* Các vector ngắt ngoại vi. */
    KEEP(*(.vector_table.interrupts));
  } > FLASH

  /* Mã lệnh và dữ liệu chỉ đọc. */
  .text :
  {
    *(.text .text.*);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > FLASH

  .ARM.extab :
  {
    *(.ARM.extab* .gnu.linkonce.armextab.*);
  } > FLASH

  /* Dữ liệu khởi tạo: nạp trong FLASH, chạy trong RAM. */
  .data : ALIGN(4)
  {
    . = ALIGN(4);
    _sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > RAM AT > FLASH

  /* Địa chỉ ảnh khởi tạo của .data trong FLASH. */
  _sidata = LOADADDR(.data);

  /* Dữ liệu chưa khởi tạo (khởi tạo 0 trong Reset handler). */
  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    _sbss = .;
    *(.bss .bss.*);
    *(COMMON);
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  /* Vùng RAM không khởi tạo. */
  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    *(.uninit .uninit.*);
    . = ALIGN(4);
  } > RAM

  /* Bỏ các section không cần cho firmware bare-metal. */
  /DISCARD/ :
  {
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}
