# Chapter 5: Distributed Patterns

## Learning Objectives

After completing this chapter, you will be able to:

1. **Messaging Patterns**
   - Understand async communication patterns
   - Implement producer-consumer with channels
   - Build simple message queue
   - Handle backpressure

2. **Resilience Patterns**
   - Implement rate limiting
   - Build circuit breaker
   - Handle failures gracefully
   - Prevent cascade failures

## Chapter Structure

```
chapter_05_distributed/
├── README.md                    # This file
├── checkpoint.md                # Self-assessment
├── 01_messaging/
│   ├── theory.md               # Channels, queues, pub/sub
│   ├── lab_01_channel_patterns/ # Producer-consumer with channels
│   └── lab_02_simple_queue/    # In-memory message queue
└── 02_patterns/
    ├── theory.md               # Resilience patterns
    ├── lab_03_rate_limiter/    # Token bucket rate limiter
    └── lab_04_circuit_breaker/ # Circuit breaker pattern
```

## Prerequisites

- Completed Chapter 4 (caching concepts)
- Understanding of async/await
- Familiarity with concurrency primitives (Mutex, channels)

## Labs Overview

| Lab | Topic | Key Concepts |
|-----|-------|--------------|
| Lab 1 | Channel Patterns | mpsc, broadcast, fan-out/fan-in |
| Lab 2 | Simple Queue | Message persistence, acknowledgment |
| Lab 3 | Rate Limiter | Token bucket, sliding window |
| Lab 4 | Circuit Breaker | Failure detection, recovery |

## Why These Patterns Matter

### Messaging
```
Without messaging:
Service A -> Service B (tight coupling, blocking)

With messaging:
Service A -> Queue -> Service B (decoupled, async)
```

Benefits:
- Decoupling services
- Handling load spikes
- Enabling async processing

### Resilience
```
Without resilience:
One service fails -> Cascade failure -> Everything down

With resilience:
One service fails -> Circuit opens -> Graceful degradation
```

Benefits:
- Prevent cascade failures
- Protect resources
- Maintain availability

## Tools for Observation

```bash
# Monitor system resources
htop                            # CPU, memory
iostat -x 1                     # Disk I/O

# Network
netstat -an | grep ESTABLISHED  # Active connections

# Rust-specific
RUST_LOG=debug cargo run        # Enable logging
```

## Recommended Reading

- "Release It!" by Michael Nygard
- "Designing Data-Intensive Applications" by Martin Kleppmann
- Tokio documentation on channels
- AWS architecture patterns

## Time Estimate

- Theory reading: 2-3 hours
- Labs: 4-5 hours
- Total: 6-8 hours
