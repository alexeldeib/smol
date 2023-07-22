#[macro_use]
extern crate log;

use nix::errno::Errno;
use nix::mount::{mount as nix_mount, MsFlags};
use nix::sys::{
    stat::Mode,
    wait::{waitpid, WaitPidFlag, WaitStatus},
};
use nix::unistd::mkdir as nix_mkdir;
use nix::NixPath;

// struct MkdirInput<'a, T: ?Sized + NixPath> {
//     target: &'a T,
//     mode: Mode,
// }

// struct MountInput<'a, T: ?Sized + NixPath> {
//     source: Option<&'a T>,
//     target: &'a T,
//     fstype: Option<&'a T>,
//     flags: MsFlags,
//     data: Option<&'a T>,
// }

// const MOUNTS: &'static [MountInput<str>] = &[MountInput::<str> {
//     source: Some("/devtmpfs"),
//     target: "/dev",
//     fstype: Some("devtmpfs"),
//     flags: MsFlags::MS_NOSUID,
//     data: Some("mode=0755"),
// }];

#[derive(Debug, thiserror::Error)]
enum InitError {
    #[error("couldn't mount {} onto {}, because: {}", source, target, error)]
    Mount {
        source: String,
        target: String,
        #[source]
        error: nix::Error,
    },

    #[error("couldn't mkdir {}, because: {}", path, error)]
    Mkdir {
        path: String,
        #[source]
        error: nix::Error,
    },
}

fn main() -> Result<(), InitError> {
    debug!("Hello, world!");

    // let chmod_0755: Mode =
    //     Mode::S_IRWXU | Mode::S_IRGRP | Mode::S_IXGRP | Mode::S_IROTH | Mode::S_IXOTH;

    // let chmod_0555: Mode = Mode::S_IRUSR
    //     | Mode::S_IXUSR
    //     | Mode::S_IRGRP
    //     | Mode::S_IXGRP
    //     | Mode::S_IROTH
    //     | Mode::S_IXOTH;

    // let chmod_1777: Mode = Mode::S_IRWXU | Mode::S_IRWXG | Mode::S_IRWXO | Mode::S_ISVTX;

    // let common_mnt_flags: MsFlags = MsFlags::MS_NODEV | MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID;

    // let common_cgroup_mnt_flags =
    //     MsFlags::MS_NODEV | MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_RELATIME;

    // debug!("Mounting /dev (first time)");
    // mkdir("/dev", chmod_0755)?;

    // debug!("Mounting newroot");
    // mkdir("/newroot", chmod_0755)?;

    // // avoid remounting /dev
    // // otherwise have to find the root device again, which we won't know
    // // because config injected on VM boot will be lost on switch_root
    // // mount(Some("/dev"), "/newroot/dev", None, MsFlags::MS_MOVE, None)?;

    // debug!("Mounting /dev/pts");
    // mkdir("/dev/pts", chmod_0755)?;
    // mount(
    //     Some("devpts"),
    //     "/dev/pts",
    //     Some("devpts"),
    //     MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NOATIME,
    //     Some("mode=0620,gid=5,ptmxmode=666"),
    // )?;

    // debug!("Mounting /dev/mqueue");
    // mkdir("/dev/mqueue", chmod_0755)?;
    // mount(
    //     Some("mqueue"),
    //     "/dev/mqueue",
    //     Some("mqueue"),
    //     common_mnt_flags,
    //     None,
    // )?;

    // debug!("Mounting /dev/shm");
    // mkdir("/dev/shm", chmod_1777)?;
    // mount(
    //     Some("shm"),
    //     "/dev/shm",
    //     Some("tmpfs"),
    //     MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
    //     None,
    // )?;

    // debug!("Mounting /dev/hugepages");
    // mkdir("/dev/hugepages", chmod_0755)?;
    // mount(
    //     Some("hugetlbfs"),
    //     "/dev/hugepages",
    //     Some("hugetlbfs"),
    //     MsFlags::MS_RELATIME,
    //     Some("pagesize=2M"),
    // )?;

    // debug!("Mounting /proc");
    // mkdir("/proc", chmod_0555)?;
    // mount(Some("proc"), "/proc", Some("proc"), common_mnt_flags, None)?;
    // mount(
    //     Some("binfmt_misc"),
    //     "/proc/sys/fs/binfmt_misc",
    //     Some("binfmt_misc"),
    //     common_mnt_flags | MsFlags::MS_RELATIME,
    //     None,
    // )?;

    // debug!("Mounting /sys");
    // mkdir("/sys", chmod_0555)?;
    // mount(Some("sys"), "/sys", Some("sysfs"), common_mnt_flags, None)?;

    // debug!("Mounting /run");
    // mkdir("/run", chmod_0755)?;
    // mount(
    //     Some("run"),
    //     "/run",
    //     Some("tmpfs"),
    //     MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
    //     Some("mode=0755"),
    // )?;
    // mkdir("/run/lock", Mode::all())?;

    // debug!("Mounting /sys/fs/cgroup/unified");
    // mkdir("/sys/fs/cgroup/unified", chmod_0555)?;
    // mount(
    //     Some("cgroup2"),
    //     "/sys/fs/cgroup/unified",
    //     Some("cgroup2"),
    //     common_mnt_flags | MsFlags::MS_RELATIME,
    //     Some("nsdelegate"),
    // )?;

    // debug!("Mounting /sys/fs/cgroup/net_cls,net_prio");
    // mkdir("/sys/fs/cgroup/net_cls,net_prio", chmod_0555)?;
    // mount(
    //     Some("cgroup"),
    //     "/sys/fs/cgroup/net_cls,net_prio",
    //     Some("cgroup"),
    //     common_cgroup_mnt_flags,
    //     Some("net_cls,net_prio"),
    // )?;

    // debug!("Mounting /sys/fs/cgroup/hugetlb");
    // mkdir("/sys/fs/cgroup/hugetlb", chmod_0555)?;
    // mount(
    //     Some("cgroup"),
    //     "/sys/fs/cgroup/hugetlb",
    //     Some("cgroup"),
    //     common_cgroup_mnt_flags,
    //     Some("hugetlb"),
    // )?;

    // debug!("Mounting /sys/fs/cgroup/pids");
    // mkdir("/sys/fs/cgroup/pids", chmod_0555)?;
    // mount(
    //     Some("cgroup"),
    //     "/sys/fs/cgroup/pids",
    //     Some("cgroup"),
    //     common_cgroup_mnt_flags,
    //     Some("pids"),
    // )?;

    // debug!("Mounting /sys/fs/cgroup/freezer");
    // mkdir("/sys/fs/cgroup/freezer", chmod_0555)?;
    // mount(
    //     Some("cgroup"),
    //     "/sys/fs/cgroup/freezer",
    //     Some("cgroup"),
    //     common_cgroup_mnt_flags,
    //     Some("freezer"),
    // )?;

    // debug!("Mounting /sys/fs/cgroup/cpu,cpuacct");
    // mkdir("/sys/fs/cgroup/cpu,cpuacct", chmod_0555)?;
    // mount(
    //     Some("cgroup"),
    //     "/sys/fs/cgroup/cpu,cpuacct",
    //     Some("cgroup"),
    //     common_cgroup_mnt_flags,
    //     Some("cpu,cpuacct"),
    // )?;

    // debug!("Mounting /sys/fs/cgroup/devices");
    // mkdir("/sys/fs/cgroup/devices", chmod_0555)?;
    // mount(
    //     Some("cgroup"),
    //     "/sys/fs/cgroup/devices",
    //     Some("cgroup"),
    //     common_cgroup_mnt_flags,
    //     Some("devices"),
    // )?;

    // debug!("Mounting /sys/fs/cgroup/blkio");
    // mkdir("/sys/fs/cgroup/blkio", chmod_0555)?;
    // mount(
    //     Some("cgroup"),
    //     "/sys/fs/cgroup/blkio",
    //     Some("cgroup"),
    //     common_cgroup_mnt_flags,
    //     Some("blkio"),
    // )?;

    // debug!("Mounting cgroup/memory");
    // mkdir("/sys/fs/cgroup/memory", chmod_0555)?;
    // mount(
    //     Some("cgroup"),
    //     "/sys/fs/cgroup/memory",
    //     Some("cgroup"),
    //     common_cgroup_mnt_flags,
    //     Some("memory"),
    // )?;

    // debug!("Mounting /sys/fs/cgroup/perf_event");
    // mkdir("/sys/fs/cgroup/perf_event", chmod_0555)?;
    // mount(
    //     Some("cgroup"),
    //     "/sys/fs/cgroup/perf_event",
    //     Some("cgroup"),
    //     common_cgroup_mnt_flags,
    //     Some("perf_event"),
    // )?;

    // debug!("Mounting /sys/fs/cgroup/cpuset");
    // mkdir("/sys/fs/cgroup/cpuset", chmod_0555)?;
    // mount(
    //     Some("cgroup"),
    //     "/sys/fs/cgroup/cpuset",
    //     Some("cgroup"),
    //     common_cgroup_mnt_flags,
    //     Some("cpuset"),
    // )?;

    // mkdir("/root", Mode::S_IRWXU)?;

    loop {
        reap_zombies();
    }

    // if let Err(error) = MOUNTS
    //     .iter()
    //     .map(|m| mount(m.source, m.target, m.fstype, m.flags, m.data))
    //     .collect::<Result<Vec<()>, InitError>>()
    // {
    //     eprintln!("Error: {}", error);
    // }
}

fn reap_zombies() {
    loop {
        match waitpid(None, Some(WaitPidFlag::WNOHANG)) {
            Ok(status) => match status {
                WaitStatus::Exited(child_pid, exit_code) => {
                    warn!(
                        "Reaped child process with pid: {}, exit code: {}",
                        child_pid, exit_code
                    )
                }
                WaitStatus::Signaled(child_pid, signal, core_dumped) => {
                    warn!(
                        "Reaped child process with pid: {} and signal: {}, core dumped? {}",
                        child_pid, signal, core_dumped
                    )
                }
                WaitStatus::Stopped(child_pid, signal) => {
                    debug!(
                        "waitpid Stopped: surprising (pid: {}, signal: {})",
                        child_pid, signal
                    );
                }
                WaitStatus::PtraceEvent(child_pid, signal, event) => {
                    debug!(
                        "waitpid PtraceEvent: interesting (pid: {}, signal: {}, event: {})",
                        child_pid, signal, event
                    );
                }
                WaitStatus::PtraceSyscall(child_pid) => {
                    debug!("waitpid PtraceSyscall: unfathomable (pid: {})", child_pid);
                }
                WaitStatus::Continued(child_pid) => {
                    debug!("waitpid Continue: not supposed to! (pid: {})", child_pid);
                }
                WaitStatus::StillAlive => {
                    trace!("no more children to reap");
                    break;
                }
            },
            Err(e) => match e {
                Errno::ECHILD => {
                    debug!("no child to wait");
                    continue;
                }
                Errno::EINTR => {
                    debug!("got EINTR waiting for pids, continuing...");
                    continue;
                }
                _ => {
                    debug!("error calling waitpid: {}", e);
                    continue;
                }
            },
        }
    }
}

fn mount<T: ?Sized + NixPath>(
    source: Option<&T>,
    target: &T,
    fstype: Option<&T>,
    flags: MsFlags,
    data: Option<&T>,
) -> Result<(), InitError> {
    nix_mount(source, target, fstype, flags, data).map_err(|error| InitError::Mount {
        source: source
            .map(|p| {
                p.with_nix_path(|cs| {
                    cs.to_owned()
                        .into_string()
                        .ok().unwrap_or_default()
                })
                .unwrap_or_else(|_| String::new())
            })
            .unwrap_or_else(String::new),
        target: target
            .with_nix_path(|cs| {
                cs.to_owned()
                    .into_string()
                    .ok().unwrap_or_default()
            })
            .unwrap_or_else(|_| String::new()),
        error,
    })
}

fn mkdir<T: ?Sized + NixPath>(path: &T, mode: Mode) -> Result<(), InitError> {
    nix_mkdir(path, mode).map_err(|error| InitError::Mkdir {
        path: path
            .with_nix_path(|cs| {
                cs.to_owned()
                    .into_string()
                    .ok().unwrap_or_default()
            })
            .unwrap_or_else(|_| String::new()),
        error,
    })
}
