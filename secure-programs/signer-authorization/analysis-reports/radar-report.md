# Radar Static Analysis Report

This report was generated on 25.09.2025 at 15:06. The results are provided for informational purposes only and should not replace thorough audits or expert evaluations. Users are responsible for conducting their own assessments and ensuring accuracy before making decisions.

## Alert Summary

| Alert       | Severity    | Certainty   | Locations   |
|-------------|-------------|-------------|-------------|
| [Account Reinitialization](#account-reinitialization) | Medium | Low | 1 |
| [Missing Owner Check](#missing-owner-check) | Low | Low | 1 |


### Account Reinitialization
**Severity:** Medium | **Certainty:** Low

When account initialization is not properly validated against reinitialization attempts, callers of the program may try to reinitialize an existing account to manipulate its data and state.

#### Locations
- /home/joe/solana-programs-analysis/secure-programs/signer-authorization/programs/signer-authorization/src/lib.rs:9:10-20
---

### Missing Owner Check
**Severity:** Low | **Certainty:** Low

The Account struct includes an owner field indicating the key associated with that account's owner. This field should be used to ensure a caller of an owner-only intended functionality, is in fact the owner.

#### Locations
- /home/joe/solana-programs-analysis/secure-programs/signer-authorization/programs/signer-authorization/src/lib.rs:27:3-9
---
