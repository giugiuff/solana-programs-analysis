# Radar Static Analysis Report

This report was generated on 26.09.2025 at 08:03. The results are provided for informational purposes only and should not replace thorough audits or expert evaluations. Users are responsible for conducting their own assessments and ensuring accuracy before making decisions.

## Alert Summary

| Alert       | Severity    | Certainty   | Locations   |
|-------------|-------------|-------------|-------------|
| [Missing Owner Check](#missing-owner-check) | Low | Low | 1 |
| [Duplicate Mutable Accounts](#duplicate-mutable-accounts) | Medium | Medium | 2 |


### Missing Owner Check
**Severity:** Low | **Certainty:** Low

The Account struct includes an owner field indicating the key associated with that account's owner. This field should be used to ensure a caller of an owner-only intended functionality, is in fact the owner.


### Duplicate Mutable Accounts
**Severity:** Medium | **Certainty:** Medium

When there are two or more accounts with mutable data, a check must be in place to ensure mutation of each account is differentiated properly, to avoid unintended data modification of other accounts.
