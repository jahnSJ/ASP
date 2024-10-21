MEMORY
{
  CCMRAM    (xrw)    : ORIGIN = 0x10000000,   LENGTH = 64K
  RAM    (xrw)    : ORIGIN = 0x20000000,   LENGTH = 192K
  FLASH    (rx)    : ORIGIN = 0x8000000,   LENGTH = 2048K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
