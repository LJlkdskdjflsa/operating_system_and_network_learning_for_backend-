# Chapter 6: Comprehensive Project

This chapter integrates everything you've learned - Rust, OS concepts, networking, databases, and distributed patterns - into a production-ready REST service with full observability.

## Learning Objectives

After completing this chapter, you will be able to:

- Build a complete REST API using Axum
- Integrate with databases using SQLx
- Implement structured logging with tracing
- Export Prometheus metrics
- Perform load testing and analyze results
- Identify and address performance bottlenecks

## Topics

### 1. REST Service (`01_rest_service/`)

Build a complete CRUD API with proper error handling and validation.

- **Theory**: REST principles, Axum architecture, request/response handling
- **Lab 1**: Axum CRUD API - Build a complete items API
- **Lab 2**: Database Integration - Add SQLite persistence with SQLx

### 2. Observability (`02_observability/`)

Add production-grade logging and metrics.

- **Theory**: Structured logging, distributed tracing, metrics types
- **Lab 3**: Tracing - Add structured logging with request spans
- **Lab 4**: Prometheus Metrics - Export HTTP metrics

### 3. Performance (`03_performance/`)

Test and optimize your service.

- **Theory**: Load testing, profiling, bottleneck analysis
- **Lab 5**: Load Testing - Benchmark and analyze your service

## Prerequisites

- Completed Chapters 1-5
- Familiarity with async Rust and Tokio
- Basic understanding of HTTP and databases

## Recommended Order

1. Complete REST Service labs first (foundation)
2. Add Observability (logging, metrics)
3. Performance testing last (requires working service)

## Time Estimate

- Theory reading: 2-3 hours
- Labs: 6-8 hours
- Total: 8-11 hours
