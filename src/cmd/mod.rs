#[cfg(all(feature = "apply", feature = "feature_capable"))]
pub mod apply;
#[cfg(feature = "datapusher_plus")]
pub mod applydp;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod behead;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod cat;
pub mod count;
pub mod datefmt;
pub mod dedup;
pub mod describegpt;
pub mod diff;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod enumerate;
pub mod excel;
pub mod exclude;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod explode;
pub mod extdedup;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod extsort;
#[cfg(all(feature = "fetch", feature = "feature_capable"))]
pub mod fetch;
#[cfg(all(feature = "fetch", feature = "feature_capable"))]
pub mod fetchpost;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod fill;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod fixlengths;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod flatten;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod fmt;
#[cfg(all(feature = "foreach", target_family = "unix", not(feature = "lite")))]
pub mod foreach;
pub mod frequency;
#[cfg(all(feature = "geocode", feature = "feature_capable"))]
pub mod geocode;
pub mod headers;
pub mod index;
pub mod input;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod join;
#[cfg(feature = "polars")]
pub mod joinp;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod jsonl;
#[cfg(feature = "luau")]
pub mod luau;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod partition;
pub mod pseudo;
#[cfg(all(feature = "python", feature = "feature_capable"))]
pub mod python;
pub mod rename;
pub mod replace;
pub mod reverse;
pub mod safenames;
pub mod sample;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod schema;
pub mod search;
pub mod searchset;
pub mod select;
pub mod slice;
pub mod snappy;
pub mod sniff;
pub mod sort;
pub mod sortcheck;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod split;
#[cfg(feature = "polars")]
pub mod sqlp;
pub mod stats;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod table;
#[cfg(all(feature = "to", feature = "feature_capable"))]
pub mod to;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod tojsonl;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub mod transpose;
pub mod validate;
