# MARU Kernel Architecture

> **Document status:** Design proposal  
> **Document version:** 0.1  
> **Project:** YEOUL Operating System  
> **Kernel:** MARU  
> **Target architectures:** x86_64 and AArch64 (ARM64)

## 1. Purpose and scope

MARU is the microkernel of YEOUL OS. It is designed as a modern, capability-oriented system with a small privileged core and independently evolving user-space services.

The architecture is not intended to reproduce the internal designs of Windows NT, traditional UNIX, or Linux. It adopts mechanisms that support strong isolation, explicit authority, memory safety, and service modularity.

This document defines the architectural direction and subsystem boundaries. It does not claim that the listed features are already implemented. Details requiring experiments or further decisions are marked as open design decisions.

### 1.1 Selected design decisions

| Area | Decision |
|---|---|
| Kernel architecture | Microkernel |
| Primary implementation language | Rust, no_std |
| Low-level implementation | Small, documented assembly layer |
| Security model | Capability-based access control |
| System services | User-space by default |
| Application interface | POSIX compatibility environment |
| Initial CPU targets | x86_64 and AArch64 |
| Executable format | ELF64 |
| Initial development environment | QEMU and cross-compilation |

### 1.2 Design priorities

1. **Isolation:** faults and authority should be contained within protection domains.
2. **Explicit authority:** access to kernel objects and devices requires capabilities.
3. **Memory safety:** use Rust's type and ownership systems wherever practical.
4. **Small privileged core:** keep policy and complex services outside the kernel.
5. **Concurrency:** design for multicore execution rather than treating SMP as an afterthought.
6. **Portability:** separate architecture-independent mechanisms from CPU-specific implementations.
7. **Verifiability:** make invariants testable and document unsafe boundaries.
8. **Compatibility:** support POSIX applications through user-space runtime components without making POSIX the kernel's internal object model.

## 2. System architecture

MARU separates privileged kernel mechanisms from user-space policy and services.

~~~text
+------------------------------------------------------+
|                 Applications                         |
|       POSIX applications / native applications       |
+-----------------------------+------------------------+
                              |
+-----------------------------v------------------------+
|             POSIX compatibility environment          |
|       libc, process runtime, fd and signal layers     |
+-----------------------------+------------------------+
                              |
+-----------------------------v------------------------+
|                 User-space services                  |
|  supervisor | process | filesystem | device | network|
+-----------------------------+------------------------+
                              |
                  IPC / shared memory /
                  capability transfer
                              |
+-----------------------------v------------------------+
|                    MARU Kernel                       |
|  threads & scheduler       address spaces            |
|  IPC & notifications       capability system         |
|  memory primitives         interrupt & timer core    |
|  architecture abstraction  syscall entry             |
+-----------------------------+------------------------+
                              |
+-----------------------------v------------------------+
|              Architecture-specific layer             |
|       x86_64                         AArch64           |
|  GDT/IDT, APIC, MMU             EL1/EL0, GIC, MMU      |
+-----------------------------+------------------------+
                              |
+-----------------------------v------------------------+
|                       Hardware                       |
+------------------------------------------------------+
~~~

### 2.1 Kernel responsibilities

The kernel provides mechanisms that require privilege or must be globally trusted:

- thread creation, execution state, and context switching;
- scheduling and CPU-local execution state;
- address-space and page-table primitives;
- physical memory allocation primitives;
- IPC endpoints, notifications, and shared-memory primitives;
- kernel-object lifetime management;
- capability validation and transfer;
- interrupt and timer delivery;
- controlled entry from user mode.

### 2.2 User-space responsibilities

The following are user-space services by default:

- filesystem implementations and namespace policy;
- device drivers and device-management policy;
- network protocols and network configuration;
- process lifecycle policy and service supervision;
- POSIX process, file-descriptor, and signal semantics;
- user identity, account policy, and service configuration.

A service may need narrowly scoped kernel capabilities for MMIO, interrupts, DMA buffers, or shared memory. User-space placement alone does not make a driver safe.

## 3. Architecture portability: x86_64 and AArch64

Both x86_64 and AArch64 are first-class architectural targets. Neither is treated as a secondary port.

The kernel is divided into architecture-independent mechanisms and architecture-specific implementations. Common code must not directly depend on x86 registers, AArch64 exception levels, a particular interrupt controller, or a particular page-table format.

### 3.1 Supported target definitions

| Architecture | Rust target triple | Initial execution model |
|---|---|---|
| x86_64 | x86_64-unknown-none | Kernel at ring 0, applications at ring 3 |
| AArch64 | aarch64-unknown-none | Kernel at EL1, applications at EL0 |

The target triples identify the intended bare-metal compilation environments. The actual boot protocol, linker layout, CPU feature baseline, and supported machine profiles must be documented and tested independently.

### 3.2 Architecture-independent code

The following components belong in common kernel code:

- scheduler policy and run-queue abstractions;
- thread lifecycle and generic thread state;
- capability tables and rights validation;
- IPC protocol and endpoint semantics;
- generic address-space API;
- memory-object abstractions;
- timer abstractions;
- syscall semantics;
- kernel object lifetime rules.

Common code may call architecture interfaces, but must not embed architecture-specific register layouts or instruction sequences.

### 3.3 Architecture-specific code

Each architecture implements the same internal contracts for:

- boot and early CPU initialization;
- privilege-level transitions;
- exception and interrupt entry/return;
- context save and restore;
- page-table creation and updates;
- TLB invalidation and address-space switching;
- CPU-local storage;
- inter-processor interrupts;
- timer programming;
- cache maintenance and memory barriers where required;
- low-level atomic and synchronization support.

A proposed source layout is:

~~~text
kernel/
  core/
    thread/
    scheduler/
    memory/
    ipc/
    capability/
    object/
    syscall/
  arch/
    mod.rs
    x86_64/
      boot/
      cpu/
      exception/
      interrupt/
      memory/
      context/
      timer/
    aarch64/
      boot/
      cpu/
      exception/
      interrupt/
      memory/
      context/
      timer/
  platform/
    pc/
    virt_aarch64/
~~~

This is a logical layout proposal, not a requirement to create every directory immediately.

### 3.4 x86_64 implementation considerations

The x86_64 port must define and test:

- boot handoff and early CPU setup;
- GDT and IDT configuration;
- ring 0 / ring 3 transitions;
- syscall entry and return mechanism;
- exception and page-fault handling;
- page-table management and TLB invalidation;
- local APIC and supported platform interrupt routing;
- a reliable timer source;
- inter-processor interrupts and per-CPU state;
- memory ordering and atomic operations.

The initial PC platform profile should be explicit. Features such as APIC mode, timer source, and required CPU extensions must not be assumed silently.

### 3.5 AArch64 implementation considerations

The AArch64 port must define and test:

- boot entry and handoff contract;
- exception-level configuration and transition to EL1;
- EL0 / EL1 user-kernel transitions;
- exception vector table and exception return;
- translation-table setup and address-space switching;
- TLB maintenance operations;
- generic timer access;
- GIC interrupt-controller integration for the selected platform;
- inter-processor interrupts;
- CPU feature discovery and cache-coherency assumptions;
- memory barriers and cache maintenance where required;
- secondary CPU startup, using a documented platform mechanism such as PSCI when available.

The initial AArch64 virtual platform should be specified separately from physical-board support. QEMU's virt machine is a suitable initial integration target, but the exact firmware, interrupt-controller version, and boot path must be fixed in the platform test configuration.

### 3.6 Platform versus architecture

Architecture and platform are separate concepts.

- **Architecture** describes CPU execution, privilege, exceptions, registers, and translation.
- **Platform** describes the machine-level boot environment, interrupt controller, timers, buses, firmware interfaces, and devices.

For example, an AArch64 QEMU virt machine and a physical ARM server share the AArch64 architecture but may have different platform integration requirements.

Drivers and platform discovery should therefore not be placed indiscriminately in the generic AArch64 CPU layer.

### 3.7 Portability acceptance criteria

A subsystem is not considered architecture-independent merely because it compiles on both targets. It must preserve the same documented semantics.

At minimum, CI should:

1. compile common kernel code for x86_64 and AArch64;
2. compile each architecture-specific implementation with its target toolchain;
3. boot the x86_64 QEMU profile;
4. boot the AArch64 QEMU virt profile;
5. run equivalent tests for memory, IPC, capability checks, and user-mode entry on both;
6. report architecture-specific test coverage separately.

Physical hardware support is a later milestone and must not be implied by QEMU success.

## 4. Kernel core subsystems

### 4.1 Threads and scheduling

The kernel manages schedulable threads. A thread contains the execution context required to resume execution, scheduling state, and a reference to its address space.

A process is a higher-level resource and lifecycle abstraction. Process policy is primarily managed by the user-space process service.

Initial scheduler proposal:

- preemptive scheduling;
- per-CPU run queues;
- priority-based round-robin within a priority level;
- idle thread per CPU;
- timer-driven preemption;
- explicit CPU affinity;
- inter-processor rescheduling notifications.

The first implementation should prioritize correctness of context switching, interrupt interaction, and synchronization. Load balancing and NUMA-aware policies can follow.

### 4.2 Address spaces and memory

The kernel provides:

- physical-page allocation primitives;
- address-space objects;
- page mapping and unmapping;
- page permission enforcement;
- address-space switching;
- shared-memory objects;
- TLB maintenance.

Memory policy such as application heap allocation, resource quotas, and service-specific reclamation strategies belongs outside the kernel where possible.

The design must distinguish physical memory ownership, virtual mappings, and capabilities to memory objects. User processes must never receive unrestricted access to kernel memory.

### 4.3 IPC

IPC is a fundamental kernel mechanism and a major performance boundary.

Initial primitives:

| Primitive | Purpose |
|---|---|
| Synchronous message | Request/reply communication |
| Notification | Lightweight event delivery |
| Shared memory | Efficient bulk data transfer |
| Capability transfer | Explicit delegation of object authority |
| Interrupt notification | Delivery of device events to authorized services |

Message payload transfer and capability transfer are separate operations. The kernel validates transferred capabilities and applies the receiver's rights constraints.

Large payloads should use shared memory or memory-object transfer rather than requiring repeated copies through the kernel.

The exact IPC model—synchronous, asynchronous, or hybrid—remains an open design decision.

### 4.4 Capability system

A capability is an unforgeable authority to access a kernel object with a bounded set of rights.

The user-visible representation is an opaque handle. It must not expose kernel pointers or permit a process to manufacture authority by guessing identifiers.

Initial object classes:

- thread;
- address space;
- memory object;
- IPC endpoint;
- notification;
- interrupt object;
- timer object;
- capability space.

Rights are object-specific. A generic rights vocabulary may include read, write, map, execute, transfer, duplicate, and control, but each object type defines which operations are meaningful.

Required invariants:

- handles are scoped to a capability space;
- invalid or stale handles fail safely;
- rights cannot be escalated by transfer;
- capability transfer is explicit;
- object lifetime is independent of user-controlled handle values;
- revocation semantics are defined before capability delegation becomes widely used.

The initial revocation strategy is not yet fixed. Generation counters and hierarchical revocation are candidates that require a prototype and threat-model review.

### 4.5 Interrupts and timers

The kernel owns privileged interrupt entry and dispatch. Device-specific policy and most interrupt handling run in user space.

The low-level interrupt path should do minimal work: identify and acknowledge the event as required by the platform, update kernel state, and notify the authorized recipient. Blocking service work must not run in a hard-interrupt context.

Timer interfaces must provide monotonic time and scheduling deadlines without exposing architecture-specific timer registers to common code.

## 5. Object and resource model

Kernel resources are represented by typed kernel objects. Each object has explicit creation, reference, authority, and destruction rules.

The kernel must not expose internal object addresses to user space. User-space handles are indices or opaque tokens resolved through a protected capability space.

Rust ownership and borrowing should be used to encode internal lifetime rules. Where shared mutable state is necessary, synchronization and lock-order invariants must be documented.

Object destruction must account for outstanding references, in-flight IPC, active mappings, and CPU-local references.

## 6. POSIX compatibility environment

POSIX is an application compatibility environment, not the kernel's internal object model.

The compatibility environment is implemented in user space through a C library, runtime components, and system services. It translates POSIX operations into MARU IPC and kernel mechanisms.

### 6.1 File descriptors and capabilities

A POSIX file descriptor is a small integer managed by the POSIX runtime. Internally, the runtime associates it with an authorized service endpoint or object capability.

For example:

~~~text
POSIX application
    read(fd, buffer, length)
             |
             v
POSIX libc / runtime
    fd table -> authorized object reference
             |
             v
Filesystem service (IPC)
             |
             v
MARU kernel mechanisms
~~~

The integer file descriptor is not itself a kernel capability and must not grant authority outside the owning runtime's descriptor table.

### 6.2 Process operations

The process service implements POSIX process semantics over kernel threads, address spaces, and capabilities.

Native process creation and executable loading should be established before attempting full fork semantics. A later fork implementation may use copy-on-write memory, but must also correctly handle descriptor state, threads, signals, and capability inheritance.

### 6.3 Staged compatibility

POSIX support must be described by a tested conformance profile rather than a broad claim of complete compatibility.

Suggested progression:

1. basic file operations and standard streams;
2. process creation, execution, exit, and wait;
3. memory mapping interfaces;
4. POSIX threads;
5. signals;
6. pipes and sockets;
7. broader POSIX interfaces and conformance testing.

The exact POSIX edition and feature profile remain to be selected. Linux system-call ABI compatibility is not implied by POSIX support and is not an initial requirement.

## 7. User-space services

System services run as isolated user-space tasks and communicate through documented IPC protocols.

| Service role | Responsibility |
|---|---|
| System supervisor | Start, monitor, and restart services |
| Process service | Process lifecycle and POSIX process policy |
| Filesystem service | Filesystem implementation and namespace policy |
| Device manager | Device discovery and service coordination |
| Device drivers | Device-specific operations |
| Network service | Network protocols and configuration |
| POSIX runtime | POSIX API and descriptor semantics |

These are functional roles, not finalized product or component names.

### 7.1 Service supervision and recovery

The supervisor starts essential services, distributes initial capabilities, monitors health, and applies restart policies.

A service restart must consider state recovery, outstanding requests, capability reissuance, and dependent services. Restarting a process alone does not guarantee recovery of its previous state.

### 7.2 User-space drivers

Drivers receive only the authority required for their device:

- narrowly scoped MMIO mappings;
- authorized interrupt notifications;
- explicitly managed DMA buffers;
- IPC endpoints for device clients.

DMA isolation depends on platform facilities such as an IOMMU or equivalent. Where such facilities are absent, the system must document the reduced isolation guarantee rather than assume DMA is safe.

## 8. Rust implementation rules

The kernel is Rust-first and uses no_std. Assembly is restricted to architecture-specific entry, exit, context-switch, and other unavoidable low-level operations.

C may be used where needed for the POSIX C library or carefully bounded interoperability.

Implementation rules:

1. keep unsafe blocks small and document their safety invariants;
2. isolate architecture-specific unsafe code behind narrow interfaces;
3. avoid unbounded allocation in interrupt and other latency-sensitive contexts;
4. document lock ordering and interrupt-safety requirements;
5. avoid global mutable state where practical;
6. define stable, explicit data layouts at ABI boundaries;
7. validate all user-provided pointers, lengths, handles, and message metadata;
8. do not treat Rust's type system as a substitute for synchronization or hardware isolation.

Rust prevents many classes of memory errors when used correctly, but does not automatically prevent deadlocks, data races hidden behind unsafe code, incorrect page-table permissions, DMA attacks, or flawed authority delegation.

## 9. Boot and initialization

Bootloader choice is an implementation decision. The kernel should consume a versioned boot-information interface rather than depend throughout its codebase on one bootloader's data structures.

Proposed sequence:

1. bootloader transfers control and supplies boot information;
2. architecture layer establishes the required CPU and exception state;
3. kernel validates memory and platform information;
4. physical memory and page-table facilities are initialized;
5. scheduler, threads, IPC, and capability mechanisms are initialized;
6. the first user-space task is created;
7. the system supervisor starts essential services;
8. the system exposes its initial command-line environment.

The x86_64 and AArch64 boot paths may differ substantially, but must produce the same documented kernel boot contract.

## 10. Fault isolation and recovery

A user-space service failure should not automatically imply kernel failure.

| Failure | Intended response |
|---|---|
| Application exits | Reclaim its resources |
| Filesystem service fails | Isolate it and attempt supervised recovery |
| Network service fails | Keep unrelated services running |
| Driver fails | Isolate the affected device path |
| Kernel invariant is violated | Enter a controlled fatal-error path and preserve diagnostics |

Recovery depends on service state and dependencies. The supervisor must not blindly restart a service when doing so could corrupt persistent state or duplicate an operation.

## 11. Testing and quality requirements

Testing must cover both architectural targets and the boundaries between protection domains.

| Test category | Required coverage |
|---|---|
| Unit tests | Core data structures and algorithms |
| Architecture tests | Exceptions, context switching, page tables, timers |
| Boot tests | x86_64 and AArch64 QEMU profiles |
| Memory tests | Mapping permissions and address-space isolation |
| Capability tests | Invalid handles, rights checks, transfer |
| IPC tests | Correctness, ordering, and data transfer |
| SMP tests | Concurrent execution and synchronization |
| Service tests | Failure isolation and recovery |
| ABI tests | Native ABI and POSIX runtime behavior |
| Performance tests | IPC, syscalls, context switches, mappings |

### 11.1 Minimum cross-architecture CI

The initial CI pipeline should:

- build the kernel for x86_64;
- build the kernel for AArch64;
- run the x86_64 QEMU boot test;
- run the AArch64 QEMU virt boot test;
- run equivalent smoke tests for user-mode entry and IPC on both targets as those features become available;
- publish logs and distinguish compile-only checks from runtime tests.

A successful cross-compilation is not evidence that the kernel boots or behaves correctly on that architecture.

### 11.2 Performance metrics

Track at least:

- IPC round-trip latency;
- syscall latency;
- context-switch latency;
- thread creation latency;
- address-space switch cost;
- memory-map operation cost;
- capability lookup cost;
- shared-memory throughput.

Benchmarks must record the QEMU machine profile, CPU model, configuration, and build mode so results can be compared meaningfully.

## 12. Development milestones

| Milestone | Scope |
|---|---|
| M0 — Foundation | Rust no_std, boot, serial output, exceptions, memory map |
| M1 — Core mechanisms | Physical memory, address spaces, threads, context switching, timer |
| M2 — Isolation | User mode, capability spaces, IPC, notifications |
| M3 — First user space | ELF loading, initial task, supervisor |
| M4 — Basic services | Process service, RAM filesystem, console and device services |
| M5 — POSIX environment | libc, descriptors, process API, threads, signals |
| M6 — Expansion | SMP hardening, drivers, networking, recovery, optimization |

Both x86_64 and AArch64 should be brought through the foundation milestones. Architecture-specific features may be staged, but the common interfaces must be designed for both from the beginning.

## 13. Open design decisions

The following items are deliberately not fixed by this document:

- synchronous, asynchronous, or hybrid IPC semantics;
- exact syscall ABI and numbering;
- capability revocation and delegation model;
- scheduler policy beyond the initial prototype;
- physical memory allocator strategy;
- exact POSIX edition and conformance profile;
- boot protocol and firmware assumptions for each platform;
- initial GIC version and AArch64 platform profile;
- device-driver framework and DMA isolation requirements;
- persistent filesystem and service recovery protocol.

Each decision should be recorded separately with its context, alternatives, rationale, and consequences. Changes to these decisions should not silently alter the documented ABI.

## 14. Architectural summary

MARU is a Rust-first, capability-based microkernel for YEOUL OS. It provides a deliberately small set of privileged mechanisms and places complex system policy in isolated user-space services.

Its initial CPU targets are **x86_64 and AArch64 (ARM64)**. Architecture-independent kernel mechanisms are shared, while boot, exception handling, context switching, memory translation, timers, and interrupt-controller integration are implemented behind architecture-specific interfaces.

POSIX compatibility is provided above the kernel through user-space runtime components. It does not replace MARU's capability model or require the kernel to adopt a traditional UNIX internal architecture.

The defining design principle is:

> **Keep the privileged core small. Make authority explicit. Isolate services. Share mechanisms across architectures without hiding their differences.**
