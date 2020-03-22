#!/usr/bin/env bash

bindgen hexchat-plugin.h -o binding.rs \
--whitelist-type "hexchat.*" --whitelist-var "HEXCHAT.*" \
--blacklist-type time_t \
--raw-line "#![allow(dead_code)]" \
--raw-line "#![allow(non_camel_case_types)]" \
--raw-line "#![allow(non_snake_case)]" \
--raw-line "#![allow(missing_docs)]" \
--raw-line "use libc::time_t;" \
