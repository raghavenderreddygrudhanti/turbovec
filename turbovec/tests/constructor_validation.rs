//! Tests that constructor validation returns proper errors instead of panicking.
//! Covers TurboQuantIndex and IdMapIndex for invalid bit_width and dim values.

use turbovec::{ConstructError, IdMapIndex, TurboQuantIndex};

// ─── TurboQuantIndex::new ────────────────────────────────────────────────────

#[test]
fn new_rejects_bit_width_zero() {
    assert!(matches!(
        TurboQuantIndex::new(128, 0),
        Err(ConstructError::InvalidBitWidth(0))
    ));
}

#[test]
fn new_rejects_bit_width_one() {
    assert!(matches!(
        TurboQuantIndex::new(128, 1),
        Err(ConstructError::InvalidBitWidth(1))
    ));
}

#[test]
fn new_rejects_bit_width_five() {
    assert!(matches!(
        TurboQuantIndex::new(128, 5),
        Err(ConstructError::InvalidBitWidth(5))
    ));
}

#[test]
fn new_rejects_bit_width_eight() {
    assert!(matches!(
        TurboQuantIndex::new(128, 8),
        Err(ConstructError::InvalidBitWidth(8))
    ));
}

#[test]
fn new_rejects_bit_width_hundred() {
    assert!(matches!(
        TurboQuantIndex::new(128, 100),
        Err(ConstructError::InvalidBitWidth(100))
    ));
}

#[test]
fn new_rejects_dim_zero() {
    assert!(matches!(
        TurboQuantIndex::new(0, 4),
        Err(ConstructError::InvalidDim(0))
    ));
}

#[test]
fn new_rejects_dim_one() {
    assert!(matches!(
        TurboQuantIndex::new(1, 4),
        Err(ConstructError::InvalidDim(1))
    ));
}

#[test]
fn new_rejects_dim_four() {
    assert!(matches!(
        TurboQuantIndex::new(4, 4),
        Err(ConstructError::InvalidDim(4))
    ));
}

#[test]
fn new_rejects_dim_seven() {
    assert!(matches!(
        TurboQuantIndex::new(7, 4),
        Err(ConstructError::InvalidDim(7))
    ));
}

#[test]
fn new_rejects_dim_nine() {
    assert!(matches!(
        TurboQuantIndex::new(9, 4),
        Err(ConstructError::InvalidDim(9))
    ));
}

#[test]
fn new_accepts_valid_bit_widths() {
    assert!(TurboQuantIndex::new(128, 2).is_ok());
    assert!(TurboQuantIndex::new(128, 3).is_ok());
    assert!(TurboQuantIndex::new(128, 4).is_ok());
}

#[test]
fn new_accepts_valid_dims() {
    assert!(TurboQuantIndex::new(8, 4).is_ok());
    assert!(TurboQuantIndex::new(16, 4).is_ok());
    assert!(TurboQuantIndex::new(128, 4).is_ok());
    assert!(TurboQuantIndex::new(1536, 4).is_ok());
    assert!(TurboQuantIndex::new(3072, 4).is_ok());
}

// ─── TurboQuantIndex::new_lazy ───────────────────────────────────────────────

#[test]
fn new_lazy_rejects_bit_width_zero() {
    assert!(matches!(
        TurboQuantIndex::new_lazy(0),
        Err(ConstructError::InvalidBitWidth(0))
    ));
}

#[test]
fn new_lazy_rejects_bit_width_five() {
    assert!(matches!(
        TurboQuantIndex::new_lazy(5),
        Err(ConstructError::InvalidBitWidth(5))
    ));
}

#[test]
fn new_lazy_accepts_valid_bit_widths() {
    assert!(TurboQuantIndex::new_lazy(2).is_ok());
    assert!(TurboQuantIndex::new_lazy(3).is_ok());
    assert!(TurboQuantIndex::new_lazy(4).is_ok());
}

// ─── IdMapIndex::new ─────────────────────────────────────────────────────────

#[test]
fn id_map_new_rejects_invalid_bit_width() {
    assert!(matches!(
        IdMapIndex::new(128, 0),
        Err(ConstructError::InvalidBitWidth(0))
    ));
    assert!(matches!(
        IdMapIndex::new(128, 5),
        Err(ConstructError::InvalidBitWidth(5))
    ));
}

#[test]
fn id_map_new_rejects_invalid_dim() {
    assert!(matches!(
        IdMapIndex::new(0, 4),
        Err(ConstructError::InvalidDim(0))
    ));
    assert!(matches!(
        IdMapIndex::new(7, 4),
        Err(ConstructError::InvalidDim(7))
    ));
}

#[test]
fn id_map_new_accepts_valid_params() {
    assert!(IdMapIndex::new(128, 2).is_ok());
    assert!(IdMapIndex::new(128, 3).is_ok());
    assert!(IdMapIndex::new(128, 4).is_ok());
}

// ─── IdMapIndex::new_lazy ────────────────────────────────────────────────────

#[test]
fn id_map_new_lazy_rejects_invalid_bit_width() {
    assert!(matches!(
        IdMapIndex::new_lazy(0),
        Err(ConstructError::InvalidBitWidth(0))
    ));
    assert!(matches!(
        IdMapIndex::new_lazy(1),
        Err(ConstructError::InvalidBitWidth(1))
    ));
}

#[test]
fn id_map_new_lazy_accepts_valid_bit_widths() {
    assert!(IdMapIndex::new_lazy(2).is_ok());
    assert!(IdMapIndex::new_lazy(3).is_ok());
    assert!(IdMapIndex::new_lazy(4).is_ok());
}
