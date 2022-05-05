use std::concat;

pub const KYA_SERVICE: &str = concat!("[Unit]\nDescription=Kya Gyazo Daemon\n\n",
"[Service]\nExecStart=kya-for-gyazo\n\n",
"[Install]\nWantedBy=default.target\n");

pub const KYA_SERVICE_FIRST_HALF: &str = concat!("[Unit]\nDescription=Kya Gyazo Daemon\n\n",
"[Service]\nExecStart=");

pub const KYA_SERVICE_SECOND_HALF: &str = concat!("\n\n", "[Install]\nWantedBy=default.target\n");