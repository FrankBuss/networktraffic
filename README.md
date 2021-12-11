# networktraffic

This is a simple network traffic logger, which logs the network traffic of one network device, and prints summary information for the last 30 days.

# how to compile and run the program

First install Rust, e.g. from https://rustup.rs. You can build the program with cargo build --release.

For creating the log, start it with the network interface and log it to a file:

```
./release/networktraffic -i eth0 > log.txt
```

This creates a log file, with the number of bytes sent and received for each minute, like this:

```
1639228696 0 0
1639228756 51083788 1276943
1639228816 50860637 1717584
1639228876 50256986 1263405
1639228936 52473745 1663039
1639228996 47932548 1089242
1639229056 39167617 1050084
1639229116 43437693 1076336
```

To show the network traffic for the last 30 days, call it with the log filename as an argument:

```
./release/networktraffic -f log.txt
```

Example output:
```
day: 1, sent: 0 B, received: 0 B
day: 2, sent: 0 B, received: 0 B
day: 3, sent: 0 B, received: 0 B
day: 4, sent: 0 B, received: 0 B
day: 5, sent: 0 B, received: 0 B
day: 6, sent: 0 B, received: 0 B
day: 7, sent: 0 B, received: 0 B
day: 8, sent: 0 B, received: 0 B
day: 9, sent: 0 B, received: 0 B
day: 10, sent: 0 B, received: 0 B
day: 11, sent: 0 B, received: 0 B
day: 12, sent: 0 B, received: 0 B
day: 13, sent: 0 B, received: 0 B
day: 14, sent: 0 B, received: 0 B
day: 15, sent: 0 B, received: 0 B
day: 16, sent: 0 B, received: 0 B
day: 17, sent: 0 B, received: 0 B
day: 18, sent: 0 B, received: 0 B
day: 19, sent: 0 B, received: 0 B
day: 20, sent: 0 B, received: 0 B
day: 21, sent: 0 B, received: 0 B
day: 22, sent: 0 B, received: 0 B
day: 23, sent: 0 B, received: 0 B
day: 24, sent: 0 B, received: 0 B
day: 25, sent: 0 B, received: 0 B
day: 26, sent: 0 B, received: 0 B
day: 27, sent: 0 B, received: 0 B
day: 28, sent: 0 B, received: 0 B
day: 29, sent: 0 B, received: 0 B
day: 30, sent: 319.7 MB, received: 8.7 MB
sum: sent: 319.7 MB, received: 8.7 MB
```
