use std::{fmt, ops};

use winapi::{
    shared::{
        minwindef::{
            DWORD,
            LPVOID
        },
        basetsd::SIZE_T
    },
    um::{
        winnt::{PVOID, HANDLE, MEMORY_BASIC_INFORMATION as MEMINFO, MEMORY_BASIC_INFORMATION32},
        sysinfoapi::{LPSYSTEM_INFO, SYSTEM_INFO, GetSystemInfo},
        processthreadsapi,
        memoryapi,
    },
};

struct SysInfo(pub SYSTEM_INFO);

impl ops::Deref for SysInfo {
    type Target = SYSTEM_INFO;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for SysInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SYSTEM_INFO")
            .field("dwPageSize", &self.dwPageSize)
            .field("lpMinimumApplicationAddress", &self.lpMinimumApplicationAddress)
            .field("lpMaximumApplicationAddress", &self.lpMaximumApplicationAddress)
            .field("dwActiveProcessorMask", &self.dwActiveProcessorMask)
            .field("dwNumberOfProcessors", &self.dwNumberOfProcessors)
            .field("dwProcessorType", &self.dwProcessorType)
            .field("dwAllocationGranularity", &self.dwAllocationGranularity)
            .field("wProcessorLevel", &self.wProcessorLevel)
            .field("wProcessorRevision", &self.wProcessorRevision)
            .finish()
    }
}

struct MemInfo(pub MEMINFO);

impl ops::Deref for MemInfo {
    type Target = MEMINFO;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for MemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION")
            .field("BaseAddress", &self.BaseAddress)
            .field("AllocationBase", &self.AllocationBase)
            .field("AllocationProtect", &self.AllocationProtect)
            .field("RegionSize", &self.RegionSize)
            .field("State", &self.State)
            .field("Protect", &self.Protect)
            .field("Type", &self.Type)
            .finish()
    }
}

fn main() {
    let this_pid: DWORD;
    let this_proc: HANDLE;
    let min_addr: LPVOID;
    let max_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEMINFO;

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMINFO>();
    
    // Ensures all memory is initialized.
    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    };

    // System calls.
    unsafe {
        this_pid = processthreadsapi::GetCurrentProcessId();
        this_proc = processthreadsapi::GetCurrentProcess();
        GetSystemInfo(&mut proc_info as LPSYSTEM_INFO);
    };
    let proc_info = SysInfo(proc_info);
    
    // Rename for convenience.
    min_addr = proc_info.lpMinimumApplicationAddress;
    max_addr = proc_info.lpMaximumApplicationAddress;

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_addr, max_addr);
    
    // Scan through the address space.
    loop {
        let rc: SIZE_T = unsafe {
            // Provides information about a specific segment of the running program's memory
            // address space, starting at base_addr.
            memoryapi::VirtualQueryEx(this_proc, base_addr, 
                                      &mut mem_info, MEMINFO_SIZE as SIZE_T)
        };
        let mem_info = MemInfo(mem_info);
        if rc == 0 {
            break;
        }
        println!("{:#?}", mem_info);
        base_addr = (base_addr as usize + mem_info.RegionSize as usize) as PVOID;
    }
}

