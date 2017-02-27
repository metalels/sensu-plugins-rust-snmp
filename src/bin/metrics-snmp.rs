extern crate snmp;
extern crate getopts;

use std::env;
use std::io::Write;
use std::time::{SystemTime, Duration};
use getopts::Options;
use snmp::{SyncSession,Value};

pub struct Target {
  name: String,
  oid: String,
  vtype: String
}

impl Target {
  pub fn new(name: &str, oid: &str, vtype: &str) -> Target {
    Target {
      name: name.to_string(),
      oid: oid.to_string(),
      vtype: vtype.to_string()
    }
  }
}

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} METRIC [options]\n\nRequires:\n    METRIC: <desc|ss|la|mem|dsk|if> or custom name with -O(oids)", program);
  print!("{}", opts.usage(&brief));
}

fn get_targets_from_nickname(nickname: &str) -> Result<Vec<Target>, String> {
  let mut targets: Vec<Target> = Vec::new();
  if nickname == "desc" {
    targets.push(Target::new("description", "1.3.6.1.2.1.1.1.0", "OctetString"));
  } else if nickname == "ss" {
    targets.push(Target::new("ssSwapIn", "1.3.6.1.4.1.2021.11.3.0", "Integer"));
    targets.push(Target::new("ssSwapOut", "1.3.6.1.4.1.2021.11.4.0", "Integer"));
    targets.push(Target::new("ssIOSent", "1.3.6.1.4.1.2021.11.5.0", "Integer"));
    targets.push(Target::new("ssIOReceive", "1.3.6.1.4.1.2021.11.6.0", "Integer"));
    targets.push(Target::new("ssSysInterrupts", "1.3.6.1.4.1.2021.11.7.0", "Integer"));
    targets.push(Target::new("ssSysContext", "1.3.6.1.4.1.2021.11.8.0", "Integer"));
    targets.push(Target::new("ssCpuUser", "1.3.6.1.4.1.2021.11.9.0", "Unknown"));
    targets.push(Target::new("ssCpuSystem", "1.3.6.1.4.1.2021.11.10.0", "Unknown"));
    targets.push(Target::new("ssCpuIdle", "1.3.6.1.4.1.2021.11.11.0", "Unknown"));
  } else if nickname == "la" {
    targets.push(Target::new("laLoad.1", "1.3.6.1.4.1.2021.10.1.3.1.0", "OctetString"));
    targets.push(Target::new("laLoad.2", "1.3.6.1.4.1.2021.10.1.3.2.0", "OctetString"));
    targets.push(Target::new("laLoad.3", "1.3.6.1.4.1.2021.10.1.3.3.0", "OctetString"));
  } else if nickname == "dsk" {
    targets.push(Target::new("dskPath", "1.3.6.1.4.1.2021.9.1.2.0", "OctetString"));
    targets.push(Target::new("dskDevice", "1.3.6.1.4.1.2021.9.1.3.0", "OctetString"));
    targets.push(Target::new("dskTotal", "1.3.6.1.4.1.2021.9.1.6.0", "Integer"));
    targets.push(Target::new("dskAvail", "1.3.6.1.4.1.2021.9.1.7.0", "Integer"));
    targets.push(Target::new("dskUsed", "1.3.6.1.4.1.2021.9.1.8.0", "Integer"));
    targets.push(Target::new("dskPercent", "1.3.6.1.4.1.2021.9.1.9.0", "Integer"));
    targets.push(Target::new("dskPercentNode", "1.3.6.1.4.1.2021.9.1.10.0", "Integer"));
  } else if nickname == "mem" {
    targets.push(Target::new("memTotalSwap", "1.3.6.1.4.1.2021.4.3.0", "Integer"));
    targets.push(Target::new("memAvailSwap", "1.3.6.1.4.1.2021.4.4.0", "Integer"));
    targets.push(Target::new("memTotalReal", "1.3.6.1.4.1.2021.4.5.0", "Integer"));
    targets.push(Target::new("memAvailReal", "1.3.6.1.4.1.2021.4.6.0", "Integer"));
    targets.push(Target::new("memTotalFree", "1.3.6.1.4.1.2021.4.11.0", "Integer"));
    targets.push(Target::new("memShared", "1.3.6.1.4.1.2021.4.13.0", "Integer"));
    targets.push(Target::new("memBuffer", "1.3.6.1.4.1.2021.4.14.0", "Integer"));
    targets.push(Target::new("memCached", "1.3.6.1.4.1.2021.4.15.0", "Integer"));
  } else if nickname == "if" {
    targets.push(Target::new("ifInOctets", "1.3.6.1.2.1.2.2.1.10.1", "Counter32"));
  }

  if targets.is_empty() {
    Err(["Not support metric-type", nickname].join(" "))
  } else {
    Ok(targets)
  }
}

fn create_targets(oids: &str) -> Result<Vec<Target>, String> {
  let mut targets: Vec<Target> = Vec::new();
  let vecs: Vec<&str> = oids.split(',').collect();

  for oid in &vecs {
    let veco: Vec<&str> = oid.split(':').collect();
    if veco.len() == 2 {
      targets.push(Target::new(veco[1], veco[0], "Unknown"));
    } else {
      targets.push(Target::new(veco[0], veco[0], "Unknown"));
    }
  }
  if targets.is_empty() {
    Err("Can not find oid".to_string())
  } else {
    Ok(targets)
  }
}

fn time_now_unix() -> u64 {
  SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

fn print_value(agent_name: &str, nickname: &str, targetname:  &str, value: &str, strflg: bool) {
  if strflg {
    println!("{}.snmp.{}.{} {:?} {:?}", agent_name, targetname, nickname, value, time_now_unix());
  } else {
    println!("{}.snmp.{}.{} {} {:?}", agent_name, targetname, nickname, value, time_now_unix());
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let mut opts = Options::new();
  opts.optopt("n", "name", "set target agent name", "NAME");
  opts.optopt("h", "host", "set target host ip address", "ADDRESS");
  opts.optopt("p", "port", "set target host port", "PORT");
  opts.optopt("c", "community", "set target community name", "COMMUNITY");
  opts.optopt("o", "oids", "set target oid(s: use [,] to joins multi oids)", "OID[:OID_NAME],OID[:OID_NAME]...");
  opts.optflag("D", "debug", "print debug logs");
  opts.optflag("H", "help", "print this help menu");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => {
      let _ = writeln!(&mut std::io::stderr(), "{}", f);
      return
    }
  };

  if matches.opt_present("H") {
    print_usage(&program, opts);
    return;
  };

  if matches.free.len() < 1 {
    let _ = writeln!(&mut std::io::stderr(), "Metric name(METRIC) must be specified.");
    return;
  };

  let mut debug = false;

  if matches.opt_present("D") {
    debug = true;
  };

  let nickname = matches.free[0].clone();
  let targets;
  if matches.opt_present("o") {
    targets = match create_targets(&matches.opt_str("o").unwrap()) {
      Ok(m) => { m }
      Err(f) => {
        let _ = writeln!(&mut std::io::stderr(), "{}", f);
        return
      }
    };
  } else {
    targets = match get_targets_from_nickname(&nickname) {
      Ok(m) => { m }
      Err(f) => {
        let _ = writeln!(&mut std::io::stderr(), "{}", f);
        return
      }
    };
  }

  if debug {
    println!("target: {}", nickname);
  };

  let agent_host = match matches.opt_str("h") {
    Some(m) => { m }
    None => { "127.0.0.1".to_string() }
  };
  let agent_port = match matches.opt_str("p") {
    Some(m) => { m }
    None => { "161".to_string() }
  };
  let agent_name = match matches.opt_str("n") {
    Some(m) => { m }
    None => { agent_host.clone() }
  };
  if debug {
    println!("agent_name: {:?}", agent_name);
    println!("agent_host: {:?}", agent_host);
    println!("agent_port: {:?}", agent_port);
  };
  let agent_addr = [agent_host, agent_port].join(":");
  if debug {
    println!("agent_addr: {:?}", agent_addr);
  };
  let community_name = match matches.opt_str("c") {
    Some(m) => { m }
    None => { "public".to_string() }
  };
  let community = community_name.as_bytes();
  if debug {
    println!("community_name: {:?}", community_name);
    println!("community_bytes: {:?}", community);
  };

  let agent_name = &*agent_name;
  let agent_addr = &*agent_addr;
  let nickname = &*nickname;
  let timeout         = Duration::from_secs(2);
  let mut sess = SyncSession::new(agent_addr, community, Some(timeout), 0).unwrap();

  for target in targets {
    let vecs: Vec<&str> = target.oid.split('.').collect();
    let vecu: Vec<u32> = vecs.iter().map(|x| x.parse::<u32>().unwrap()).collect();
    if debug {
      println!("target oid vec: {:?}: {:?}", target.name, vecu);
    }
    let mut response;
    match sess.get(&vecu) {
      Ok(m) => { response = m }
      Err(f) => {
        let _ = writeln!(&mut std::io::stderr(), "{:?}", f);
        return
      }
    };

    if debug {
      println!("response: {:?}", response);
    }

    let result = response.varbinds.next();

    if target.vtype == "Counter32" {
      if let Some((_oid, Value::Counter32(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*value.to_string(), false);
      }
    } else if target.vtype == "Unsigned32" {
      if let Some((_oid, Value::Unsigned32(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*value.to_string(), false);
      }
    } else if target.vtype == "Counter64" {
      if let Some((_oid, Value::Counter64(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*value.to_string(), false);
      }
    } else if target.vtype == "Integer" {
      if let Some((_oid, Value::Integer(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*value.to_string(), false);
      }
    } else if target.vtype == "Opaque" {
      if let Some((_oid, Value::Opaque(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*String::from_utf8_lossy(value), true);
      }
    } else if target.vtype == "OctetString" {
      if let Some((_oid, Value::OctetString(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*String::from_utf8_lossy(value), true);
      }
    } else {// try all matching
      if let Some((_oid, Value::Counter32(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*value.to_string(), false);
      } else  if let Some((_oid, Value::Unsigned32(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*value.to_string(), false);
      } else if let Some((_oid, Value::Counter64(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*value.to_string(), false);
      } else if let Some((_oid, Value::Integer(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*value.to_string(), false);
      } else if let Some((_oid, Value::Opaque(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*String::from_utf8_lossy(value), true);
      } else if let Some((_oid, Value::OctetString(value))) = result {
        print_value(agent_name, &*target.name, nickname, &*String::from_utf8_lossy(value), true);
      }
    }
  }
}
