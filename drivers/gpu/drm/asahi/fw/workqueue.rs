// SPDX-License-Identifier: GPL-2.0-only OR MIT
#![allow(missing_docs)]
#![allow(dead_code)]

//! GPU work queues

use super::event;
use super::types::*;
use crate::event::EventValue;
use crate::trivial_gpustruct;

#[derive(Debug)]
#[repr(u32)]
pub(crate) enum CommandType {
    RunVertex = 0,
    RunFragment = 1,
    RunBlitter = 2,
    RunCompute = 3,
    Barrier = 4,
    InitBuffer = 6,
}

pub(crate) trait Command: GpuStruct {}

pub(crate) mod raw {
    use super::*;

    #[derive(Debug)]
    #[repr(C)]
    pub(crate) struct Barrier {
        pub(crate) tag: CommandType,
        pub(crate) wait_stamp: GpuWeakPointer<FwStamp>,
        pub(crate) wait_value: EventValue,
        pub(crate) wait_slot: u32,
        pub(crate) stamp_self: EventValue,
        pub(crate) uuid: u32,
        pub(crate) unk: u32,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub(crate) struct GpuContextData {
        pub(crate) unk_0: u8,
        pub(crate) unk_1: u8,
        unk_2: Array<0x2, u8>,
        pub(crate) unk_4: u8,
        pub(crate) unk_5: u8,
        unk_6: Array<0x18, u8>,
        pub(crate) unk_1e: u8,
        pub(crate) unk_1f: u8,
        unk_20: Array<0x3, u8>,
        pub(crate) unk_23: u8,
        unk_24: Array<0x1c, u8>,
    }

    impl Default for GpuContextData {
        fn default() -> Self {
            Self {
                unk_0: 0xff,
                unk_1: 0xff,
                unk_2: Default::default(),
                unk_4: 0,
                unk_5: 1,
                unk_6: Default::default(),
                unk_1e: 0xff,
                unk_1f: 0,
                unk_20: Default::default(),
                unk_23: 2,
                unk_24: Default::default(),
            }
        }
    }

    #[derive(Debug, Default)]
    #[repr(C)]
    pub(crate) struct RingState {
        pub(crate) gpu_doneptr: AtomicU32,
        __pad0: Pad<0xc>,
        pub(crate) unk_10: AtomicU32,
        __pad1: Pad<0xc>,
        pub(crate) unk_20: AtomicU32,
        __pad2: Pad<0xc>,
        pub(crate) gpu_rptr: AtomicU32,
        __pad3: Pad<0xc>,
        pub(crate) cpu_wptr: AtomicU32,
        __pad4: Pad<0xc>,
        pub(crate) rb_size: u32,
        __pad5: Pad<0xc>,
        // This isn't part of the structure, but it's here as a
        // debugging hack so we can inspect what ring position
        // the driver considered complete and freeable.
        pub(crate) cpu_freeptr: AtomicU32,
        __pad6: Pad<0xc>,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub(crate) struct Priority(u32, u32, U64, u32, u32, u32);

    pub(crate) const PRIORITY: [Priority; 4] = [
        Priority(0, 0, U64(0xffff_ffff_ffff_0000), 1, 0, 1),
        Priority(1, 1, U64(0xffff_ffff_0000_0000), 0, 0, 0),
        Priority(2, 2, U64(0xffff_0000_0000_0000), 0, 0, 2),
        Priority(3, 3, U64(0x0000_0000_0000_0000), 0, 0, 3),
    ];

    impl Default for Priority {
        fn default() -> Priority {
            PRIORITY[2]
        }
    }

    #[derive(Debug)]
    #[repr(C)]
    pub(crate) struct QueueInfo<'a> {
        pub(crate) state: GpuPointer<'a, super::RingState>,
        pub(crate) ring: GpuPointer<'a, &'a [u64]>,
        pub(crate) notifier_list: GpuWeakPointer<event::NotifierList>,
        pub(crate) gpu_buf: GpuPointer<'a, &'a [u8]>,
        pub(crate) gpu_rptr1: AtomicU32,
        pub(crate) gpu_rptr2: AtomicU32,
        pub(crate) gpu_rptr3: AtomicU32,
        pub(crate) event_id: AtomicI32,
        pub(crate) priority: Priority,
        pub(crate) unk_4c: i32,
        pub(crate) uuid: u32,
        pub(crate) unk_54: i32,
        pub(crate) unk_58: U64,
        pub(crate) busy: AtomicU32,
        pub(crate) __pad: Pad<0x20>,
        pub(crate) unk_84_state: AtomicU32,
        pub(crate) unk_88: u32,
        pub(crate) unk_8c: u32,
        pub(crate) unk_90: u32,
        pub(crate) unk_94: u32,
        pub(crate) pending: AtomicU32,
        pub(crate) unk_9c: u32,
        pub(crate) gpu_context: GpuWeakPointer<super::GpuContextData>,
        pub(crate) unk_a8: U64,
    }
}

trivial_gpustruct!(Barrier);
trivial_gpustruct!(GpuContextData);
trivial_gpustruct!(RingState);

impl Command for Barrier {}

#[derive(Debug)]
pub(crate) struct QueueInfo {
    pub(crate) state: GpuObject<RingState>,
    pub(crate) ring: GpuArray<u64>,
    pub(crate) gpu_buf: GpuArray<u8>,
}

impl GpuStruct for QueueInfo {
    type Raw<'a> = raw::QueueInfo<'a>;
}
