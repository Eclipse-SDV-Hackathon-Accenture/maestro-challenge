use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref COMMANDS: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("cpu-usage", "awk '/cpu / {CPU=($2+$4)*100/($2+$4+$5)} END {printf(\"%.1f\", CPU)}' /proc/stat"), // CPU usage in %
            ("cpu-usage-user", "awk '/cpu / {CPU=($2)*100/($2+$4+$5)} END {printf(\"%.1f\", CPU)}' /proc/stat"), // CPU usage user in %
            ("cpu-usage-system", "awk '/cpu / {CPU=($4)*100/($2+$4+$5)} END {printf(\"%.1f\", CPU)}' /proc/stat"), // CPU usage system in %
            ("memory-usage", "awk '/MemTotal/ {TOT=$2} /MemFree/ {FREE=$2} END {printf(\"%.1f\", FREE/TOT * 100)}' /proc/meminfo"), // memory usage in %
            ("cores", "awk '/siblings/ {printf(\"%u\", $3); exit}' /proc/cpuinfo"), // amount of cores
            ("memory-total", "awk '/MemTotal/ {printf(\"%u\", $2/1024)}' /proc/meminfo"), // total memory in KiB
        ])
    };
}
