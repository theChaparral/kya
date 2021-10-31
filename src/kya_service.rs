use std::concat;

pub const KYA_SERVICE: &str = concat!("[Unit]\nDescription=Kya Gyazo Daemon\n\n",
"[Service]\nExecStart=kya-for-gyazo\n\n",
"[Install]\nWantedBy=default.target\n");
