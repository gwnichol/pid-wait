use syscalls::{syscall, SyscallNo, syscall_args};
use std::{os::unix::io::RawFd, process::Command, error::Error};
use std::boxed::Box;
use std::fmt;
use std::str;
use std::io::Write;
use nix::{unistd, poll::{PollFd, PollFlags, ppoll}, sys::signal::SigSet};

#[derive(Debug)]
pub struct WaitPidError {
    text: String,
}

impl WaitPidError {
    pub fn new(description: String) -> WaitPidError
    {
        WaitPidError {
            text: description
        }
    }

    pub fn from(description: &'static str) -> WaitPidError
    {
        WaitPidError::new(String::from(description))
    }
}

impl fmt::Display for WaitPidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Error for WaitPidError {}

/// Get the pid of a command based off search term
/// It will prompt the user if the search has multiple matches
pub fn get_pid_for_cmd(search: &str) -> Result<unistd::Pid, Box<dyn Error>> {
    let out = Command::new("ps").arg("a").arg("-o").arg("pid=,command=").output()?;
    let stdout = String::from_utf8(out.stdout)?;
    let mut matches : Vec<(u64,&str)> = Vec::new();
    let mypid = unistd::getpid().as_raw() as u64;
    for line in stdout.lines() {
        let split = line.trim().split_once(' ').unwrap_or(("",""));
        if split.1.contains(search){
            let pid = split.0.parse::<u64>()?;
            if pid != mypid {
                matches.push((pid, split.1));
            }
        }
    }

    if matches.len() == 0 {
        Err(Box::new(WaitPidError::from("There were no matches for search")))
    } else if matches.len() == 1 {
        Ok(unistd::Pid::from_raw(matches[0].0 as i32))
    } else {
        let mut index : usize = 0;
        for option in &matches {
            println!("{}: ({}) {}", index, option.0, option.1);
            index = index + 1;
        }

        let mut line : String = String::new();
        print!("Select a match: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut line)?;
        let choice : usize = line.trim().parse()?;
        if choice >= matches.len() {
            Err(Box::new(WaitPidError::new(format!("Invalid choice: {}", choice))))
        } else {
            Ok(unistd::Pid::from_raw(matches[choice].0 as i32))
        }
    }
}

/// Open and return a fd that references a given process by pid
/// This wraps around the SYS_pidfd_open systecall on Linux
/// The returned fd can be polled to determine when the process closes.
pub fn fd_from_pid(pid: unistd::Pid) -> Result<RawFd, Box<dyn Error>> {
    if pid <= unistd::Pid::from_raw(0) {
        return Err(Box::new(WaitPidError::new(format!("Improper pid given: {}", pid))));
    }
    // This unsafe call _should_ be safe since the syscall does not use any pointers. Just need to make sure the fd is disposed of
    match unsafe { syscall(SyscallNo::SYS_pidfd_open, &syscall_args!(pid.as_raw() as u64)) } {
        Ok(fd) => {
            if fd < 0 {
                Err(Box::new(WaitPidError::from("Invalid fd treated as Ok")))
            } else {
                Ok(fd as RawFd)
            }
        },
        Err(err) => {
            Err(Box::new(WaitPidError::new(format!("SYS_pidfd_open call returned error: {}", err))))
        }
    }
}

/// Wait for a pid to finish execution then return
/// This will open a FD from the pid then poll it to wait for a return
pub fn wait_for_pid(pid: unistd::Pid) -> Result<(), Box<dyn Error>> {
    let pidfd = fd_from_pid(pid)?;

    println!("Pid is good: {}", pidfd);

    let pid_poll : PollFd = PollFd::new(pidfd, PollFlags::POLLIN);

    let ret : Result<(), Box<dyn Error>> = match ppoll(&mut[pid_poll], None, SigSet::empty()) {
        Ok(_) => {
            Ok(())
        },
        Err(_) => {
            Err(Box::new(WaitPidError::from("Error listening for fd")))
        }
    };

    unistd::close(pidfd)?;

    ret
}