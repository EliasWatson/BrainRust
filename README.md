# BrainRust

A [BrainF\*ck](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in Rust.

# Benchmarks

| Version | Mandlebrot         |
| ------- | ------------------ |
| 1.0.0   | 70.377 s ± 1.522 s |
| 1.1.0   | 12.239 s ± 0.076 s |
| 1.3.0   | 6.3440 s ± 0.149 s |
| 1.3.2   | 6.1790 s ± 0.067 s |
| 1.3.3   | 6.8400 s ± 0.069 s |
| 1.4.0   | 6.8590 s ± 0.071 s |
| 1.5.1   | 6.3900 s ± 0.058 s |

## Environment

Benchmarks ran with [hyperfine](https://github.com/sharkdp/hyperfine):

```shell
hyperfine -w 1 -r 10 './target/release/brainrust program_name.bf'
```

System specs:
- Manjaro (Linux Kernel 6.0)
- Ryzen 7 3800X
- Dual-Channel 16GB DDR4-3600
- Sabrent Rocket 4.0 1TB NVMe SSD

