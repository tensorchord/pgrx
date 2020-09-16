// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx_utils::{exit_with_error, get_pgbin_dir, get_pgdata_dir, handle_result};
use std::process::Stdio;

pub(crate) fn status_postgres(major_version: u16, fail_on_error: bool) -> bool {
    let datadir = get_pgdata_dir(major_version);
    let bindir = get_pgbin_dir(major_version);

    if !datadir.exists() {
        // Postgres couldn't possibly be running if there's no data directory
        // and even if it were, we'd have no way of knowing
        return false;
    }

    let mut command = std::process::Command::new(format!("{}/pg_ctl", bindir.display()));
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("status")
        .arg("-D")
        .arg(datadir.display().to_string());
    let command_str = format!("{:?}", command);

    if fail_on_error {
        let output = handle_result!(
            format!("failed to get postgres' status: {}", command_str),
            command.output()
        );

        let code = output.status.code().unwrap();
        let is_running = code == 0; // running
        let is_stopped = code == 3; // not running

        if !is_running && !is_stopped {
            exit_with_error!(
                "problem running pg_ctl: {}\n\n{}",
                command_str,
                String::from_utf8(output.stderr).unwrap()
            )
        }

        // a status code of zero means it's running
        is_running
    } else {
        match command.status() {
            // is it running?
            Ok(status) => status.code().unwrap_or(3) == 0,

            // assume it's not
            Err(_) => false,
        }
    }
}
