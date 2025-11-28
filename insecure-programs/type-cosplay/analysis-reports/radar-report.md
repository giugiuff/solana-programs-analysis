# Radar Static Analysis Report

This report was generated on 26.09.2025 at 08:03. The results are provided for informational purposes only and should not replace thorough audits or expert evaluations. Users are responsible for conducting their own assessments and ensuring accuracy before making decisions.

## Alert Summary

| Alert       | Severity    | Certainty   | Locations   |
|-------------|-------------|-------------|-------------|
| [Missing Owner Check](#missing-owner-check) | Low | Low | 1 |
| [Type Cosplay](#type-cosplay) | Low | Low | 1 |


### Missing Owner Check
**Severity:** Low | **Certainty:** Low

The Account struct includes an owner field indicating the key associated with that account's owner. This field should be used to ensure a caller of an owner-only intended functionality, is in fact the owner.

#### Locations
- /home/joe/solana-programs-analysis/insecure-programs/type-cosplay/programs/type-cosplay/src/lib.rs:26:3-9
---

### Type Cosplay
**Severity:** Low | **Certainty:** Low

When two account types can be deserialized with the exact same values, a malicious user could substitute between the account types, leading to unexpected execution and possible authorization bypass depending on how the data is used. Using try_from_slice does not check for the necessary discriminator.

#### Locations
- /home/joe/solana-programs-analysis/insecure-programs/type-cosplay/programs/type-cosplay/src/lib.rs:10:26-40
---
