//! BRAIN native functions
//!
//! Native functions for BRAIN

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

