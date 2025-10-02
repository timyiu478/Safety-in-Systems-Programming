# A tool to inspect the open files of processes

## Usage

```bash
Usage: {} <name or pid of target>
```

## Example

```bash
tim@tim-virtual-machine ~/g/S/w/inspect-fds (main)> ./multi_pipe_test &
tim@tim-virtual-machine ~/g/S/w/inspect-fds (main)> cargo run multi_pipe_test
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/inspect-fds multi_pipe_test`
========== "./multi_pipe_test" (pid 703152, ppid 664779) ==========
0    (read/write)    cursor: 0    <terminal>
1    (read/write)    cursor: 0    <terminal>
2    (read/write)    cursor: 0    <terminal>
4    (write)         cursor: 0    <pipe #43317768>
5    (read)          cursor: 0    <pipe #43317769>
24   (read)          cursor: 143360 /usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf
25   (read)          cursor: 16384 /usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf
26   (read)          cursor: 24576 /usr/share/fonts/truetype/dejavu/DejaVuSansMono-Oblique.ttf
27   (read)          cursor: 28672 /usr/share/fonts/truetype/dejavu/DejaVuSansMono-BoldOblique.ttf
========== "./multi_pipe_test" (pid 703155, ppid 703152) ==========
0    (read)          cursor: 0    <pipe #43317768>
1    (write)         cursor: 0    <pipe #43317769>
2    (read/write)    cursor: 0    <terminal>
24   (read)          cursor: 143360 /usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf
25   (read)          cursor: 16384 /usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf
26   (read)          cursor: 24576 /usr/share/fonts/truetype/dejavu/DejaVuSansMono-Oblique.ttf
27   (read)          cursor: 28672 /usr/share/fonts/truetype/dejavu/DejaVuSansMono-BoldOblique.ttf
```

## Remarks

1. You need to run this tool on a Linux computer, because Mac and Windows don’t use the same API (`/proc/`) for providing information about processes. 
