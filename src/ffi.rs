use ::std::os::raw::{
    c_char,
    c_int,
    c_long,
    c_longlong,
    c_schar,
    c_short,
    c_uchar,
    c_ulong,
    c_ushort,
    c_uint,
    c_void
};


#[repr(C)]
pub struct iovec {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct vmctx {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct seg_desc {
	pub base: uint64_t,
	pub limit: uint32_t,
	pub access: uint32_t,
}

#[repr(C)]
pub struct vm_guest_paging {
    pub cr3: uint64_t,
    pub cpl: i32,
    pub cpu_mode: vm_cpu_mode,
    pub paging_mode: vm_paging_mode,
}

// XXX translate this beast
#[repr(C)]
pub struct vm_exit {
    pub _address: u8,
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

// XXX MIKE verify
#[repr(C)]
pub struct cpuset_t {
    pub _bits: [ulong_t; 4usize],
}

#[repr(C)]
pub enum x2apic_state {
    X2APIC_DISABLED,
    X2APIC_ENABLED,
    X2APIC_STATE_LAST,
}

#[repr(C)]
pub enum vm_cpu_mode {
	CPU_MODE_REAL,
	CPU_MODE_PROTECTED,
	CPU_MODE_COMPATIBILITY,         /* IA-32E mode (CS.L = 0) */
	CPU_MODE_64BIT,                 /* IA-32E mode (CS.L = 1) */
}

#[repr(C)]
pub enum vm_paging_mode {
	PAGING_MODE_FLAT,
	PAGING_MODE_32,
	PAGING_MODE_PAE,
	PAGING_MODE_64,
}

#[repr(C)]
pub enum vm_mmap_style {
	 VM_MMAP_NONE,           /* no mapping */
	 VM_MMAP_ALL,            /* fully and statically mapped */
	 VM_MMAP_SPARSE,         /* mappings created on-demand */
}

#[repr(C)]
pub enum vm_suspend_how {
	VM_SUSPEND_NONE,
	VM_SUSPEND_RESET,
	VM_SUSPEND_POWEROFF,
	VM_SUSPEND_HALT,
	VM_SUSPEND_TRIPLEFAULT,
	VM_SUSPEND_LAST
}

#[repr(C)]
pub enum vm_cap_type {
	VM_CAP_HALT_EXIT,
	VM_CAP_MTRAP_EXIT,
	VM_CAP_PAUSE_EXIT,
	VM_CAP_UNRESTRICTED_GUEST,
	VM_CAP_ENABLE_INVPCID,
	VM_CAP_MAX
}

#[repr(C)]
pub enum vm_intr_trigger {
	EDGE_TRIGGER,
	LEVEL_TRIGGER
}


pub type __int8_t = c_schar;
pub type __uint8_t = c_uchar;
pub type __int16_t = c_short;
pub type __uint16_t = c_ushort;
pub type __int32_t = c_int;
pub type __uint32_t = c_uint;
pub type __int64_t = c_long;
pub type __uint64_t = c_ulong;
pub type __register_t = __int64_t;
pub type __vm_offset_t = __uint64_t;
pub type __vm_paddr_t = __uint64_t;
pub type __vm_ooffset_t = __int64_t;
pub type __vm_size_t = __uint64_t;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = __register_t;
pub type sbintime_t = __int64_t;
pub type vm_memattr_t = c_char;
pub type vm_offset_t = __vm_offset_t;
pub type vm_ooffset_t = __vm_ooffset_t;
pub type vm_paddr_t = __vm_paddr_t;
pub type vm_pindex_t = __uint64_t;
pub type vm_size_t = __vm_size_t;
pub type ulong_t = c_ulong;
pub type time_t = c_long;
pub type suseconds_t = c_long;
pub type uint32_t = u32;
pub type uint64_t = u64;


#[link(name = "libvmm")]
extern "C" {
    /// Get the length and name of the memory segment identified by 'segid'.
    /// Note that system memory segments are identified with a nul name.
    ///
    /// Returns 0 on success and non-zero otherwise.
    pub fn vm_get_memseg(ctx: *const vmctx, ident: c_int, lenp: *mut usize, name: *mut c_char,
         namesiz: usize) -> c_int;

    /// Iterate over the guest address space. This function finds an address range
    /// that starts at an address >= *gpa.
    ///
    /// Returns 0 if the next address range was found and non-zero otherwise.
    pub fn vm_mmap_getnext(ctx: *const vmctx, gpa: *mut vm_paddr_t, segid: *mut i32,
        segoff: *mut vm_offset_t, len: *mut usize, prot: *mut i32, flags: *mut i32) -> c_void;

    /// Create a device memory segment identified by 'segid'.
    ///
    /// Returns a pointer to the memory segment on success and MAP_FAILED otherwise.
    pub fn vm_create_devmem(ctx: *const vmctx, segid: i32, name: *const c_char, len: usize)
        -> *const c_void;

    /// Map the memory segment identified by 'segid' into the guest address space
    /// at [gpa,gpa+len) with protection 'prot'.
    pub fn vm_mmap_memseg( ctx: *mut vmctx, gpa: vm_paddr_t, segid: c_int, segoff: vm_ooffset_t,
        len: usize, prot: c_int) -> c_int;

    pub fn vm_create(name: *const c_char) -> c_int;

    pub fn vm_get_device_fd(ctx: *mut vmctx) -> c_int;

    pub fn vm_open(name: *const c_char) -> *mut vmctx;

    pub fn vm_destroy(ctx: *mut vmctx);

    pub fn vm_parse_memsize( optarg: *const c_char, memsize: *mut usize,) -> c_int;

    pub fn vm_setup_memory(ctx: *mut vmctx, len: usize, s: vm_mmap_style) -> c_int;

    pub fn vm_map_gpa( ctx: *mut vmctx, gaddr: vm_paddr_t, len: usize,) -> *mut c_void;

    pub fn vm_get_gpa_pmap( arg1: *mut vmctx, gpa: uint64_t, pte: *mut uint64_t, num: *mut c_int)
		-> c_int;

    pub fn vm_gla2gpa( arg1: *mut vmctx, vcpuid: c_int, paging: *mut vm_guest_paging,
		gla: uint64_t, prot: c_int, gpa: *mut uint64_t, fault: *mut c_int) -> c_int;

    pub fn vm_gla2gpa_nofault( arg1: *mut vmctx, vcpuid: c_int, paging: *mut vm_guest_paging,
        gla: uint64_t, prot: c_int, gpa: *mut uint64_t, fault: *mut c_int) -> c_int;

    pub fn vm_get_lowmem_limit(ctx: *mut vmctx) -> u32;

    pub fn vm_set_lowmem_limit(ctx: *mut vmctx, limit: u32);

    pub fn vm_set_memflags(ctx: *mut vmctx, flags: c_int);

    pub fn vm_get_memflags(ctx: *mut vmctx) -> c_int;

    pub fn vm_get_lowmem_size(ctx: *mut vmctx) -> usize;

    pub fn vm_get_highmem_size(ctx: *mut vmctx) -> usize;

    pub fn vm_set_desc( ctx: *mut vmctx, vcpu: c_int, reg: c_int, base: uint64_t, limit: u32,
		access: u32) -> c_int;

    pub fn vm_get_desc( ctx: *mut vmctx, vcpu: c_int, reg: c_int, base: *mut uint64_t,
		limit: *mut u32, access: *mut u32) -> c_int;

    pub fn vm_get_seg_desc( ctx: *mut vmctx, vcpu: c_int, reg: c_int, seg_desc: *mut seg_desc)
		-> c_int;

    pub fn vm_set_register( ctx: *mut vmctx, vcpu: c_int, reg: c_int, val: uint64_t) -> c_int;

    pub fn vm_get_register( ctx: *mut vmctx, vcpu: c_int, reg: c_int, retval: *mut uint64_t)
		-> c_int;

    pub fn vm_set_register_set( ctx: *mut vmctx, vcpu: c_int, count: c_uint, regnums: *const c_int,
		regvals: *mut uint64_t) -> c_int;

    pub fn vm_get_register_set( ctx: *mut vmctx, vcpu: c_int, count: c_uint, regnums: *const c_int,
		regvals: *mut uint64_t) -> c_int;

    pub fn vm_run( ctx: *mut vmctx, vcpu: c_int, ret_vmexit: *mut vm_exit) -> c_int;

    pub fn vm_suspend(ctx: *mut vmctx, how: vm_suspend_how) -> c_int;

    pub fn vm_reinit(ctx: *mut vmctx) -> c_int;

    pub fn vm_apicid2vcpu(ctx: *mut vmctx, apicid: c_int) -> c_int;

    pub fn vm_lapic_irq( ctx: *mut vmctx, vcpu: c_int, vector: c_int) -> c_int;

    pub fn vm_lapic_local_irq( ctx: *mut vmctx, vcpu: c_int, vector: c_int) -> c_int;

    pub fn vm_lapic_msi(ctx: *mut vmctx, addr: uint64_t, msg: uint64_t) -> c_int;

    pub fn vm_ioapic_assert_irq( ctx: *mut vmctx, irq: c_int) -> c_int;

    pub fn vm_ioapic_deassert_irq( ctx: *mut vmctx, irq: c_int) -> c_int;

    pub fn vm_ioapic_pulse_irq( ctx: *mut vmctx, irq: c_int) -> c_int;

    pub fn vm_ioapic_pincount( ctx: *mut vmctx, pincount: *mut c_int) -> c_int;

    pub fn vm_isa_assert_irq( ctx: *mut vmctx, atpic_irq: c_int, ioapic_irq: c_int) -> c_int;

    pub fn vm_isa_deassert_irq( ctx: *mut vmctx, atpic_irq: c_int, ioapic_irq: c_int) -> c_int;

    pub fn vm_isa_pulse_irq( ctx: *mut vmctx, atpic_irq: c_int, ioapic_irq: c_int) -> c_int;

    pub fn vm_isa_set_irq_trigger( ctx: *mut vmctx, atpic_irq: c_int, trigger: vm_intr_trigger)
		-> c_int;

    pub fn vm_inject_nmi(ctx: *mut vmctx, vcpu: c_int) -> c_int;

    pub fn vm_capability_name2type(capname: *const c_char) -> c_int;

    pub fn vm_capability_type2name(type_: c_int) -> *const c_char;

    pub fn vm_get_capability( ctx: *mut vmctx, vcpu: c_int, cap: vm_cap_type, retval: *mut c_int)
		-> c_int;

    pub fn vm_set_capability( ctx: *mut vmctx, vcpu: c_int, cap: vm_cap_type, val: c_int) -> c_int;

    pub fn vm_assign_pptdev(ctx: *const vmctx, pptfd: c_int) -> c_int;

    pub fn vm_unassign_pptdev(ctx: *const vmctx, pptfd: c_int) -> c_int;

    // XXX MIKE ppt methods here
    /*
    int	vm_map_pptdev_mmio(struct vmctx *ctx, int pptfd, vm_paddr_t gpa,
        size_t len, vm_paddr_t hpa);
    int	vm_setup_pptdev_msi(struct vmctx *ctx, int vcpu, int pptfd,
        uint64_t addr, uint64_t msg, int numvec);
    int	vm_setup_pptdev_msix(struct vmctx *ctx, int vcpu, int pptfd,
        int idx, uint64_t addr, uint64_t msg, uint32_t vector_control);
    int	vm_get_pptdev_limits(struct vmctx *ctx, int pptfd, int *msi_limit,
        int *msix_limit);
    */

    pub fn vm_get_intinfo( ctx: *mut vmctx, vcpu: c_int, i1: *mut uint64_t, i2: *mut uint64_t)
		-> c_int;

    pub fn vm_set_intinfo( ctx: *mut vmctx, vcpu: c_int, exit_intinfo: uint64_t) -> c_int;

    /// Return a pointer to the statistics buffer. Note that this is not MT-safe.
    pub fn vm_get_stats( ctx: *mut vmctx, vcpu: c_int, ret_tv: *mut timeval,
		ret_entries: *mut c_int) -> *mut uint64_t;

    pub fn vm_get_stat_desc( ctx: *mut vmctx, index: c_int) -> *const c_char;

    pub fn vm_get_x2apic_state( ctx: *mut vmctx, vcpu: c_int, s: *mut x2apic_state) -> c_int;

    pub fn vm_set_x2apic_state( ctx: *mut vmctx, vcpu: c_int, s: x2apic_state) -> c_int;

    pub fn vm_get_hpet_capabilities( ctx: *mut vmctx, capabilities: *mut u32) -> c_int;

    /// Translate the GLA range [gla,gla+len) into GPA segments in 'iov'.
    /// The 'iovcnt' should be big enough to accommodate all GPA segments.
    ///
    /// retval	fault		Interpretation
    ///   0		  0		Success
    ///   0		  1		An exception was injected into the guest
    /// EFAULT	 N/A		Error
    pub fn vm_copy_setup( ctx: *mut vmctx, vcpu: c_int, pg: *mut vm_guest_paging, gla: uint64_t,
		len: usize, prot: c_int, iov: *mut iovec, iovcnt: c_int, fault: *mut c_int,) -> c_int;

    pub fn vm_copyin( ctx: *mut vmctx, vcpu: c_int, guest_iov: *mut iovec, host_dst: *mut c_void,
		len: usize);

    pub fn vm_copyout( ctx: *mut vmctx, vcpu: c_int, host_src: *const c_void,
		guest_iov: *mut iovec, len: usize);

    pub fn vm_copy_teardown( ctx: *mut vmctx, vcpu: c_int, iov: *mut iovec, iovcnt: c_int);

    /// RTC
    pub fn vm_rtc_write( ctx: *mut vmctx, offset: c_int, value: u8,) -> c_int;

    pub fn vm_rtc_read( ctx: *mut vmctx, offset: c_int, retval: *mut u8,) -> c_int;

    pub fn vm_rtc_settime(ctx: *mut vmctx, secs: time_t) -> c_int;

    pub fn vm_rtc_gettime(ctx: *mut vmctx, secs: *mut time_t) -> c_int;

    /// Reset vcpu register state
    pub fn vcpu_reset(ctx: *mut vmctx, vcpu: c_int) -> c_int;

    pub fn vm_active_cpus(ctx: *mut vmctx, cpus: *mut cpuset_t) -> c_int;

    pub fn vm_suspended_cpus(ctx: *mut vmctx, cpus: *mut cpuset_t) -> c_int;

    pub fn vm_debug_cpus(ctx: *mut vmctx, cpus: *mut cpuset_t) -> c_int;

    pub fn vm_activate_cpu(ctx: *mut vmctx, vcpu: c_int) -> c_int;

    pub fn vm_suspend_cpu(ctx: *mut vmctx, vcpu: c_int) -> c_int;

    pub fn vm_resume_cpu(ctx: *mut vmctx, vcpu: c_int) -> c_int;

    /// CPU topology
    pub fn vm_set_topology( ctx: *mut vmctx, sockets: u16, cores: u16, threads: u16, maxcpus: u16)
		-> c_int;

    pub fn vm_get_topology( ctx: *mut vmctx, sockets: *mut u16, cores: *mut u16, threads: *mut u16,
		 maxcpus: *mut u16) -> c_int;
}
