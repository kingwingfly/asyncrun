#![doc = include_str!("../README.md")]
#![deny(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    elided_lifetimes_in_paths
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use anyhow::Result;
use clap::Parser;
use std::env::current_dir;
use std::fmt::Display;
use std::io::Read;
use std::path::PathBuf;
use std::process::ExitStatus;
use tokio::fs::read_dir;
use tokio::process::Command;
use tokio_stream::wrappers::ReadDirStream;

#[derive(Parser, Debug)]
#[clap(name = "asyncrun", version, author, about)]
struct Cli {
    /// Filter by file/dir names all exist
    #[arg(short = 'e')]
    exist: Option<Vec<String>>,
    /// Filter by file/dir names all non-exist
    #[arg(short = 'E')]
    not_exist: Option<Vec<String>>,
    /// Number of async commands to run concurrently
    #[arg(short = 'n', default_value_t = 1 << 10)]
    async_num: usize,
    /// Path to search for directories
    #[arg(required = false, default_value = current_dir().unwrap().into_os_string())]
    path: PathBuf,
    /// Commands to run in each directory
    #[arg(required = true, last = true)]
    commands: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let filtered: Vec<_> = {
        use tokio_stream::StreamExt as _;
        let entries = ReadDirStream::new(read_dir(args.path).await?);
        entries
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().is_dir()
                    && args
                        .exist
                        .as_ref()
                        .map_or(true, |ns| ns.iter().all(|n| e.path().join(n).exists()))
                    && args
                        .not_exist
                        .as_ref()
                        .map_or(true, |ns| ns.iter().all(|n| !e.path().join(n).exists()))
            })
            .collect()
            .await
    };
    if filtered.is_empty() {
        eprintln!("All directories filtered.");
        return Ok(());
    }
    eprintln!("Filtered directories:");
    for e in filtered.iter() {
        eprintln!("{}", e.path().to_string_lossy())
    }
    let mut buf = [0; 1];
    eprintln!("Do you want to continue? [y/*]");
    std::io::stdin().read_exact(&mut buf).unwrap();
    if buf[0] != b'y' {
        return Ok(());
    }
    {
        use futures::StreamExt as _;
        let cmds = args.commands.join(" && ");
        let mut stream = futures::stream::iter(filtered.into_iter())
            .map(|e| {
                let mut cmd = Command::new("sh");
                cmd.args(["-c", &cmds]).current_dir(e.path());
                async move {
                    match cmd.output().await {
                        Ok(out) => Result::<Output>::Ok(Output {
                            path: e.path(),
                            status: out.status,
                            stdout: String::from_utf8(out.stdout).unwrap(),
                            stderr: String::from_utf8(out.stderr).unwrap(),
                        }),
                        Err(_) => Err(anyhow::anyhow!("Failed spawning child process.")),
                    }
                }
            })
            .buffer_unordered(args.async_num);
        while let Some(res) = stream.next().await {
            match res {
                Ok(out) => {
                    println!("{}", out);
                }
                Err(e) => {
                    eprintln!("{}", e)
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Output {
    path: PathBuf,
    status: ExitStatus,
    stdout: String,
    stderr: String,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Path:\n{}", self.path.to_string_lossy())?;
        writeln!(f, "StatusCode: {:?}", self.status.code())?;
        if !self.stdout.is_empty() {
            writeln!(f, "StdOut:\n{}", self.stdout.trim_end())?;
        }
        if !self.stderr.is_empty() {
            writeln!(f, "StdErr:\n{}", self.stderr.trim_end())?;
        }
        write!(f, "-------------------")?;
        Ok(())
    }
}
