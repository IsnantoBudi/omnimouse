# E2E Test Infra: Riot MQ-10 Performance & Native Win32 Optimization

## Test Philosophy
- Requirement-driven, opaque-box, and deterministic verification.
- Validates performance, CPU usage, zero external child process spawning, timing precision, and static build cleanliness.

## Feature Inventory
| # | Feature | Source (requirement) | Tier 1 | Tier 2 | Tier 3 |
|---|---------|---------------------|:------:|:------:|:------:|
| 1 | Zero PowerShell Child Process | ORIGINAL_REQUEST §R1 | Pass | Pass | Pass |
| 2 | Device Monitoring CPU < 0.1% | ORIGINAL_REQUEST §R1 | Pass | Pass | Pass |
| 3 | Hardware Status Accuracy (USB/BT/Both/None) | ORIGINAL_REQUEST §R1 | Pass | Pass | Pass |
| 4 | Rapid-Click 100 CPS Throughput (10ms) | ORIGINAL_REQUEST §R2 | Pass | Pass | Pass |
| 5 | Mouse Hook Responsiveness (No Stutter) | ORIGINAL_REQUEST §R2 | Pass | Pass | Pass |
| 6 | No Unsynchronized Data Races (GUI Timer) | ORIGINAL_REQUEST §R3 | Pass | Pass | Pass |
| 7 | Zero Build Errors and Zero Warnings | ORIGINAL_REQUEST §Criteria | Pass | Pass | Pass |

## Test Architecture
- **Static Verification**:
  - `cargo check --message-format=short`: Must exit with code 0 and emit 0 errors and 0 warnings.
  - `cargo test`: Must execute all unit tests with 0 failures and 0 warnings.
  - Source tree scan: Zero occurrences of `Command::new("powershell")` or `powershell` invocations in `src/`.
- **Runtime Verification**:
  - Subprocess monitor: When `riot-mq10.exe` is running, query `Get-Process powershell` to confirm 0 instances spawned.
  - CPU load monitor: Query `Get-Process riot-mq10` to verify CPU < 0.1% over a 30-second window.
  - Click timing validation: Verify click intervals in `autoclick.rs` unit tests and simulated burst timing tests.
