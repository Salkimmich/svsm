// SPDX-License-Identifier: (GPL-2.0-or-later OR MIT)
//
// Copyright (c) 2023 SUSE LLC
//
// Author: Joerg Roedel <jroedel@suse.de>
//
// vim: ts=4 sw=4 et

// Address space definitions for SVSM virtual memory layout

/// Size helpers
pub const SIZE_1K: usize = 1024;
pub const SIZE_1M: usize = SIZE_1K * 1024;
pub const SIZE_1G: usize = SIZE_1M * 1024;

/// Pagesize definitions
pub const PAGE_SIZE: usize = SIZE_1K * 4;
pub const PAGE_SIZE_2M: usize = SIZE_1M * 2;

/// More size helpers
pub const SIZE_LEVEL3: usize = 1usize << ((9 * 3) + 12);
pub const SIZE_LEVEL2: usize = 1usize << ((9 * 2) + 12);
pub const SIZE_LEVEL1: usize = 1usize << ((9 * 1) + 12);

// Stack definitions
pub const STACK_PAGES: usize = 4;
pub const STACK_SIZE: usize = PAGE_SIZE * STACK_PAGES;
pub const STACK_GUARD_SIZE: usize = STACK_SIZE;
pub const STACK_TOTAL_SIZE: usize = STACK_SIZE + STACK_GUARD_SIZE;

const SIGN_BIT: usize = 47;

const fn sign_extend(addr: usize) -> usize {
    let mask = 1usize << SIGN_BIT;
    if (addr & mask) == mask {
        addr | 0xffff_0000_0000_0000
    } else {
        addr
    }
}

/// Level3 page-table index shared between all CPUs
pub const PGTABLE_LVL3_IDX_SHARED: usize = 511;

/// Base Address of shared memory region
pub const SVSM_SHARED_BASE: usize = sign_extend(PGTABLE_LVL3_IDX_SHARED << ((3 * 9) + 12));

/// Mapping range for shared stacks
pub const SVSM_SHARED_STACK_BASE: usize = SVSM_SHARED_BASE + (256 * SIZE_1G);
pub const SVSM_SHARED_STACK_END: usize = SVSM_SHARED_STACK_BASE + SIZE_1G;

/// PerCPU mappings level 3 index
pub const PGTABLE_LVL3_IDX_PERCPU: usize = 510;

/// Base Address of shared memory region
pub const SVSM_PERCPU_BASE: usize = sign_extend(PGTABLE_LVL3_IDX_PERCPU << ((3 * 9) + 12));

/// Region for PerCPU Stacks
pub const SVSM_PERCPU_STACKS_BASE: usize = SVSM_PERCPU_BASE + SIZE_LEVEL1;

/// Stack address of the per-cpu init task
pub const SVSM_STACKS_INIT_TASK: usize = SVSM_PERCPU_STACKS_BASE;

///  IST Stacks base address
pub const SVSM_STACKS_IST_BASE: usize = SVSM_STACKS_INIT_TASK + STACK_TOTAL_SIZE;

/// DoubleFault IST stack base address
pub const SVSM_STACK_IST_DF_BASE: usize = SVSM_STACKS_IST_BASE;
