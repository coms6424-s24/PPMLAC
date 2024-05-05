/* Copyright lowRISC contributors (OpenTitan project). */
/* Licensed under the Apache License, Version 2.0, see LICENSE for details. */
/* SPDX-License-Identifier: Apache-2.0 */
/*
  Overflow call stack and jump at end of loop
*/
  loopi 8, 1
    addi x1, x0, 0

  loopi 1, 1
    jal x1, done

done:
  ecall
