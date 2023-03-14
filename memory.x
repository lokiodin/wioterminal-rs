MEMORY
{
  /* Leave 16k for the default bootloader on the Wio Terminal */
  FLASH (rx)   : ORIGIN = 0x00000000 + 16K, LENGTH = 512K - 16K
  RAM   (rxw)  : ORIGIN = 0x20000000, LENGTH = 192K - 1K
  /* Can be removed if we use panic_halt instead of panic_persist ! */
  PANDUMP (rxw): ORIGIN = 0x2000FC00, LENGTH = 1K

}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
  /* Can be removed if we use panic_halt instead of panic_persist ! */
_panic_dump_start = ORIGIN(PANDUMP);
  /* Can be removed if we use panic_halt instead of panic_persist ! */
_panic_dump_end   = ORIGIN(PANDUMP) + LENGTH(PANDUMP);