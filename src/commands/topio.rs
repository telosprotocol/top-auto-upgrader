use std::{
    io::Write,
    process::{Command, Output},
};

use crate::error::AuError;

#[derive(Debug)]
pub struct TopioCommands {
    operator_user: String,
    exec_dir: String,
}

impl TopioCommands {
    pub fn new(user: &str, exec_dir: &str) -> Self {
        TopioCommands {
            operator_user: String::from(user),
            exec_dir: String::from(exec_dir),
        }
    }

    pub fn kill_topio(&self) -> Result<Output, AuError> {
        let cmd_str = String::from(
            r#"ps -ef | grep topio | grep -v grep | grep -v upgrader | awk '{print $2}' | xargs kill -9"#,
        );
        let c = Command::new("sudo")
            .args(&["-u", &self.operator_user])
            .args(&["sh", "-c"])
            .arg(cmd_str)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let r = c.wait_with_output()?;
        Ok(r)
    }

    pub fn wget_new_topio(&self, tag: String) -> Result<Output, AuError> {
        let cmd_str = format!(
            r#"cd {} && wget https://github.com/telosprotocol/TOP-chain/releases/download/v{}/topio-{}-release.tar.gz -O topio-{}-release.tar.gz > /dev/null 2>&1 && tar zxvf topio-{}-release.tar.gz > /dev/null 2>&1 "#,
            &self.exec_dir, &tag, &tag, &tag, &tag
        );
        let c = Command::new("sudo")
            .args(&["-u", &self.operator_user])
            .args(&["sh", "-c"])
            .arg(cmd_str)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let r = c.wait_with_output()?;
        Ok(r)
    }

    pub fn install_new_topio(&self, tag: String) -> Result<Output, AuError> {
        let cmd_str = format!(
            r#"cd {} && cd topio-{}-release && sudo bash install.sh && . /etc/profile && bash set_topio.sh && . ~/.bashrc && ulimit -n 65535 && topio -v"#,
            &self.exec_dir, &tag
        );
        _ = Command::new("sudo")
            .args(&["-u", &self.operator_user])
            .args(&["sh", "-c"])
            .arg(cmd_str)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?
            .wait_with_output();
        self.check_version()
    }

    pub fn check_version(&self) -> Result<Output, AuError> {
        let cmd_str = format!(r#"cd {} && topio -v"#, &self.exec_dir);
        let c = Command::new("sudo")
            .args(&["-u", &self.operator_user])
            .args(&["sh", "-c"])
            .arg(cmd_str)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let r = c.wait_with_output()?;
        Ok(r)
    }

    pub fn set_miner_key(&self, mining_pub_key: String, pswd: String) -> Result<Output, AuError> {
        let cmd_str = format!(
            r#"cd {} && topio mining setMinerKey {}"#,
            &self.exec_dir, &mining_pub_key
        );
        let mut command = Command::new("sudo")
            .args(&["-u", &self.operator_user])
            .args(&["sh", "-c"])
            .arg(cmd_str)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let mut stdin = command.stdin.take().expect("Failed to use stdin");

        std::thread::spawn(move || {
            stdin
                .write_all(pswd.as_bytes())
                .expect("Failed to write to stdin");
        });

        let output = command.wait_with_output()?;

        Ok(output)
    }

    pub fn start_topio(&self) -> Result<Output, AuError> {
        let cmd_str = format!(r#"cd {} && topio node startNode"#, &self.exec_dir);
        let c = Command::new("sudo")
            .args(&["-u", &self.operator_user])
            .args(&["sh", "-c"])
            .arg(cmd_str)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let r = c.wait_with_output()?;
        Ok(r)
    }

    pub fn check_is_joined(&self) -> Result<Output, AuError> {
        let cmd_str = format!(r#"cd {} && topio node isJoined"#, &self.exec_dir);
        let c = Command::new("sudo")
            .args(&["-u", &self.operator_user])
            .args(&["sh", "-c"])
            .arg(cmd_str)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let r = c.wait_with_output()?;
        Ok(r)
    }
}

#[cfg(test)]
mod test {
    use crate::commands::TopioCommands;

    #[test]
    #[ignore]
    fn test_topio_cmd() {
        let c = TopioCommands::new("top", "/tmp/test_topio_au");

        // let r = c.set_miner_key(String::from("BKQLB1qlWXqmfltrMuP0u2h8hfq+Wk8JnbzQbP5EG0xqgWUw97wDF7VnsQOlQ0WVvd/Kv1a6ijFKkf8SPwDSWa4="),String::from("1234"));
        // println!("set key result:{:?}", r);

        let r = c.kill_topio();
        println!("kill result:{:?}", r);

        let r = c.wget_new_topio(String::from("1.7.1"));
        println!("wget result:{:?}", r);

        let r = c.install_new_topio(String::from("1.7.1"));
        println!("install result:{:?}", r);

        let r = c.check_version();
        println!("version result:{:?}", r);

        let r = c.set_miner_key(String::from("BKQLB1qlWXqmfltrMuP0u2h8hfq+Wk8JnbzQbP5EG0xqgWUw97wDF7VnsQOlQ0WVvd/Kv1a6ijFKkf8SPwDSWa4="),String::from("1234"));
        println!("set key result:{:?}", r);

        let r = c.start_topio();
        println!("start result:{:?}", r);

        let r = c.check_is_joined();
        println!("check start result:{:?}", r);
    }
}
