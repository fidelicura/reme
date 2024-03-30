# Overview
Event reminder for Linux with convenient configuration!

>By default, resulting binary is optimized for size. No really need in performance optimizations.

# Example
Here you can see example config of `reme`:
```toml
[[event]] # write for every event to differ between
time.post = "2023-03-14T00:00:00" # for now, only datetime according to ISO 8601 (no date only) is supported
time.warn = "30 minutes" # show notification every "N X" where N is value and X is seconds/minutes/hours/days/weeks/months/years
priority = "low" # notification priority (just in case you want some notification to be exteremely important)
message.main = "welcome!" # header of notification
message.additional = "welcoming text" # body of notification

[[event]] # example of shortened toml format that also works
time.post = "2024-08-27T00:00:00"
priority = "critical"
message = { main = "second new message" }
```
