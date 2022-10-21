# BrainRust

A [BrainF\*ck](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in Rust.

# Benchmarks

| Version | Mandlebrot         |
| ------- | ------------------ |
| 0.0.0   | 68.754 s ± 0.531 s |

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
