use std::ops::Deref;
use std::sync::Arc;
use lazy_static::lazy_static;
use poise::Context;
use poise::serenity_prelude::Color;
use sysinfo::{ProcessorExt, ProcessRefreshKind, RefreshKind, System, SystemExt};
use time::OffsetDateTime;

lazy_static! {
	static ref PROCESSES: ProcessRefreshKind = {
		ProcessRefreshKind::new().with_cpu()
	};
	static ref SPECIFICS: RefreshKind = RefreshKind::new().with_cpu().with_memory().with_processes(*PROCESSES);
	pub static ref HOSTNAME: String = {
		let sys = System::new_with_specifics(*SPECIFICS);

		sys.host_name().unwrap_or("ERR: no sys name".to_owned())
	};
	pub static ref OS: String = {
		let sys = System::new_with_specifics(*SPECIFICS);

		sys.os_version().unwrap_or("ERR: no OS ver".to_owned())
	};
}

#[derive(Clone, Copy)]
pub struct Sys {
	pub last_checked: i64,
	pub cpu_usage: f32,
	pub avail_mem: u64,
}


impl Sys {
	pub fn new() -> Self {
		let sys = System::new_with_specifics(*SPECIFICS);

		Self {
			last_checked: OffsetDateTime::now_utc().unix_timestamp(),
			cpu_usage: sys.global_processor_info().cpu_usage(),
			avail_mem: sys.available_memory() / 1024,
		}
	}
	pub fn get_sysinfo(&mut self) -> Self {
		let now = OffsetDateTime::now_utc().unix_timestamp();
		if now - self.last_checked > 60 {
			let mut sys = System::new_with_specifics(*SPECIFICS);
			sys.refresh_specifics(*SPECIFICS);
			self.last_checked = now;
			self.cpu_usage = sys.global_processor_info().cpu_usage();
			self.avail_mem = sys.available_memory() / 1024;
		}
		*self
	}
}