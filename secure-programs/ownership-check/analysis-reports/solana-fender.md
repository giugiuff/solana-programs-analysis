# Solana Fender Analysis: ownership-check

## Summary

Total findings: 2

| Severity | Count |
| --- | --- |
| Medium | 2 |

## Detailed Findings

### File: src/lib.rs

#### Medium Severity Issue

**Severity**: Medium

**Location**: Line 39

**Description**: Struct 'SecureOwnershipv1' has multiple Account fields without constraints to prevent duplicate accounts

**Recommendation**: This is a medium severity issue that should be reviewed. Check for duplicate mutable accounts to prevent unintended data modification.

#### Medium Severity Issue

**Severity**: Medium

**Location**: Line 53

**Description**: Struct 'SecureOwnershipv2' has multiple Account fields without constraints to prevent duplicate accounts

**Recommendation**: This is a medium severity issue that should be reviewed. Check for duplicate mutable accounts to prevent unintended data modification.

