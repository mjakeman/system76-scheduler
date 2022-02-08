// Copyright 2021-2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command;
use std::process::Stdio;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut xprop = Command::new("xprop")
        .arg("-spy")
        .arg("-root")
        .arg("_NET_ACTIVE_WINDOW")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()?;

    let mut output = BufReader::new(xprop.stdout.take().unwrap()).lines();

    while let Ok(Some(line)) = output.next_line().await {
        if let Some(window_id) = line.split(' ').rev().next() {
            if window_id.starts_with("0x") && window_id != "0x0" {
                if let Ok(pid) = fetch_pid(window_id).await {
                    set_foreground_process(pid).await;
                }
            }
        }
    }

    Ok(())
}

async fn fetch_pid(window_id: &str) -> anyhow::Result<u32> {

}