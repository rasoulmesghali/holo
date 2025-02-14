//
// Copyright (c) The Holo Core Contributors
//
// See LICENSE for license details.
//

#![feature(lazy_cell)]
#![warn(rust_2018_idioms)]

pub mod serde;

use std::collections::HashMap;
use std::sync::{Arc, LazyLock as Lazy, OnceLock};

use maplit::hashmap;
use tracing::error;
use yang2::context::{
    Context, ContextFlags, EmbeddedModuleKey, EmbeddedModules,
};

// Global YANG context.
pub static YANG_CTX: OnceLock<Arc<Context>> = OnceLock::new();

// List of embedded YANG modules.
//
// All implemented or imported modules need to be specified here. Holo by
// default doesn't support loading YANG modules from the filesystem.
pub static YANG_EMBEDDED_MODULES: Lazy<EmbeddedModules> = Lazy::new(|| {
    hashmap! {
        // IETF modules
        EmbeddedModuleKey::new("iana-bfd-types", Some("2021-10-21"), None, None) =>
            include_str!("../modules/ietf/iana-bfd-types@2021-10-21.yang"),
        EmbeddedModuleKey::new("iana-bgp-community-types", Some("2023-07-05"), None, None) =>
            include_str!("../modules/ietf/iana-bgp-community-types@2023-07-05.yang"),
        EmbeddedModuleKey::new("iana-bgp-notification", Some("2023-07-05"), None, None) =>
            include_str!("../modules/ietf/iana-bgp-notification@2023-07-05.yang"),
        EmbeddedModuleKey::new("iana-bgp-rib-types", Some("2023-07-05"), None, None) =>
            include_str!("../modules/ietf/iana-bgp-rib-types@2023-07-05.yang"),
        EmbeddedModuleKey::new("iana-bgp-types", Some("2023-07-05"), None, None) =>
            include_str!("../modules/ietf/iana-bgp-types@2023-07-05.yang"),
        EmbeddedModuleKey::new("iana-if-type", Some("2017-01-19"), None, None) =>
            include_str!("../modules/ietf/iana-if-type@2017-01-19.yang"),
        EmbeddedModuleKey::new("iana-routing-types", Some("2018-10-29"), None, None) =>
            include_str!("../modules/ietf/iana-routing-types@2018-10-29.yang"),
        EmbeddedModuleKey::new("ietf-bfd-ip-mh", Some("2022-09-22"), None, None) =>
            include_str!("../modules/ietf/ietf-bfd-ip-mh@2022-09-22.yang"),
        EmbeddedModuleKey::new("ietf-bfd-ip-sh", Some("2022-09-22"), None, None) =>
            include_str!("../modules/ietf/ietf-bfd-ip-sh@2022-09-22.yang"),
        EmbeddedModuleKey::new("ietf-bfd-types", Some("2022-09-22"), None, None) =>
            include_str!("../modules/ietf/ietf-bfd-types@2022-09-22.yang"),
        EmbeddedModuleKey::new("ietf-bfd", Some("2022-09-22"), None, None) =>
            include_str!("../modules/ietf/ietf-bfd@2022-09-22.yang"),
        EmbeddedModuleKey::new("ietf-bgp", Some("2023-07-05"), None, None) =>
            include_str!("../modules/ietf/ietf-bgp@2023-07-05.yang"),
        EmbeddedModuleKey::new("ietf-bgp", Some("2023-07-05"), Some("ietf-bgp-capabilities"), Some("2023-07-05")) =>
            include_str!("../modules/ietf/ietf-bgp-capabilities@2023-07-05.yang"),
        EmbeddedModuleKey::new("ietf-bgp", Some("2023-07-05"), Some("ietf-bgp-common"), Some("2023-07-05")) =>
            include_str!("../modules/ietf/ietf-bgp-common@2023-07-05.yang"),
        EmbeddedModuleKey::new("ietf-bgp", Some("2023-07-05"), Some("ietf-bgp-common-multiprotocol"), Some("2023-07-05")) =>
            include_str!("../modules/ietf/ietf-bgp-common-multiprotocol@2023-07-05.yang"),
        EmbeddedModuleKey::new("ietf-bgp", Some("2023-07-05"), Some("ietf-bgp-common-structure"), Some("2023-07-05")) =>
            include_str!("../modules/ietf/ietf-bgp-common-structure@2023-07-05.yang"),
        EmbeddedModuleKey::new("ietf-bgp", Some("2023-07-05"), Some("ietf-bgp-neighbor"), Some("2023-07-05")) =>
            include_str!("../modules/ietf/ietf-bgp-neighbor@2023-07-05.yang"),
        EmbeddedModuleKey::new("ietf-bgp", Some("2023-07-05"), Some("ietf-bgp-rib"), Some("2023-07-05")) =>
            include_str!("../modules/ietf/ietf-bgp-rib@2023-07-05.yang"),
        EmbeddedModuleKey::new("ietf-bgp", Some("2023-07-05"), Some("ietf-bgp-rib-attributes"), Some("2023-07-05")) =>
            include_str!("../modules/ietf/ietf-bgp-rib-attributes@2023-07-05.yang"),
        EmbeddedModuleKey::new("ietf-bgp", Some("2023-07-05"), Some("ietf-bgp-rib-tables"), Some("2023-07-05")) =>
            include_str!("../modules/ietf/ietf-bgp-rib-tables@2023-07-05.yang"),
        EmbeddedModuleKey::new("ietf-bgp-policy", Some("2023-07-05"), None, None) =>
            include_str!("../modules/ietf/ietf-bgp-policy@2023-07-05.yang"),
        EmbeddedModuleKey::new("ietf-interfaces", Some("2018-02-20"), None, None) =>
            include_str!("../modules/ietf/ietf-interfaces@2018-02-20.yang"),
        EmbeddedModuleKey::new("ietf-ip", Some("2018-02-22"), None, None) =>
            include_str!("../modules/ietf/ietf-ip@2018-02-22.yang"),
        EmbeddedModuleKey::new("ietf-isis", Some("2022-10-19"), None, None) =>
            include_str!("../modules/ietf/ietf-isis@2022-10-19.yang"),
        EmbeddedModuleKey::new("ietf-key-chain", Some("2017-06-15"), None, None) =>
            include_str!("../modules/ietf/ietf-key-chain@2017-06-15.yang"),
        EmbeddedModuleKey::new("ietf-mpls-ldp", Some("2022-03-14"), None, None) =>
            include_str!("../modules/ietf/ietf-mpls-ldp@2022-03-14.yang"),
        EmbeddedModuleKey::new("ietf-netconf-acm", Some("2018-02-14"), None, None) =>
            include_str!("../modules/ietf/ietf-netconf-acm@2018-02-14.yang"),
        EmbeddedModuleKey::new("ietf-ospf", Some("2022-10-19"), None, None) =>
            include_str!("../modules/ietf/ietf-ospf@2022-10-19.yang"),
        EmbeddedModuleKey::new("ietf-ospf-sr", Some("2023-07-09"), None, None) =>
            include_str!("../modules/ietf/ietf-ospf-sr@2023-07-09.yang"),
        EmbeddedModuleKey::new("ietf-ospfv3-extended-lsa", Some("2023-08-13"), None, None) =>
            include_str!("../modules/ietf/ietf-ospfv3-extended-lsa@2023-08-13.yang"),
        EmbeddedModuleKey::new("ietf-rip", Some("2020-02-20"), None, None) =>
            include_str!("../modules/ietf/ietf-rip@2020-02-20.yang"),
        EmbeddedModuleKey::new("ietf-routing", Some("2018-03-13"), None, None) =>
            include_str!("../modules/ietf/ietf-routing@2018-03-13.yang"),
        EmbeddedModuleKey::new("ietf-routing-policy", Some("2021-10-11"), None, None) =>
            include_str!("../modules/ietf/ietf-routing-policy@2021-10-11.yang"),
        EmbeddedModuleKey::new("ietf-routing-types", Some("2017-12-04"), None, None) =>
            include_str!("../modules/ietf/ietf-routing-types@2017-12-04.yang"),
        EmbeddedModuleKey::new("ietf-segment-routing-common", Some("2021-05-26"), None, None) =>
            include_str!("../modules/ietf/ietf-segment-routing-common@2021-05-26.yang"),
        EmbeddedModuleKey::new("ietf-segment-routing-mpls", Some("2021-05-26"), None, None) =>
            include_str!("../modules/ietf/ietf-segment-routing-mpls@2021-05-26.yang"),
        EmbeddedModuleKey::new("ietf-segment-routing", Some("2021-05-26"), None, None) =>
            include_str!("../modules/ietf/ietf-segment-routing@2021-05-26.yang"),
        EmbeddedModuleKey::new("ietf-tcp", Some("2022-09-11"), None, None) =>
            include_str!("../modules/ietf/ietf-tcp@2022-09-11.yang"),
        EmbeddedModuleKey::new("ietf-tcp-common", Some("2023-04-17"), None, None) =>
            include_str!("../modules/ietf/ietf-tcp-common@2023-04-17.yang"),
        // IETF Holo augmentations
        EmbeddedModuleKey::new("holo-ospf", None, None, None) =>
            include_str!("../modules/augmentations/holo-ospf.yang"),
        EmbeddedModuleKey::new("holo-ospf-dev", None, None, None) =>
            include_str!("../modules/augmentations/holo-ospf-dev.yang"),
        // IETF Holo deviations
        EmbeddedModuleKey::new("ietf-mpls-ldp-holo-deviations", None, None, None) =>
            include_str!("../modules/deviations/ietf-mpls-ldp-holo-deviations.yang"),
        EmbeddedModuleKey::new("ietf-interfaces-holo-deviations", None, None, None) =>
            include_str!("../modules/deviations/ietf-interfaces-holo-deviations.yang"),
        EmbeddedModuleKey::new("ietf-ip-holo-deviations", None, None, None) =>
            include_str!("../modules/deviations/ietf-ip-holo-deviations.yang"),
        EmbeddedModuleKey::new("ietf-key-chain-holo-deviations", None, None, None) =>
            include_str!("../modules/deviations/ietf-key-chain-holo-deviations.yang"),
        EmbeddedModuleKey::new("ietf-ospf-holo-deviations", None, None, None) =>
            include_str!("../modules/deviations/ietf-ospf-holo-deviations.yang"),
        EmbeddedModuleKey::new("ietf-ospf-sr-holo-deviations", None, None, None) =>
            include_str!("../modules/deviations/ietf-ospf-sr-holo-deviations.yang"),
        EmbeddedModuleKey::new("ietf-ospfv3-extended-lsa-holo-deviations", None, None, None) =>
            include_str!("../modules/deviations/ietf-ospfv3-extended-lsa-holo-deviations.yang"),
        EmbeddedModuleKey::new("ietf-rip-holo-deviations", None, None, None) =>
            include_str!("../modules/deviations/ietf-rip-holo-deviations.yang"),
        EmbeddedModuleKey::new("ietf-routing-holo-deviations", None, None, None) =>
            include_str!("../modules/deviations/ietf-routing-holo-deviations.yang"),
        EmbeddedModuleKey::new("ietf-segment-routing-mpls-holo-deviations", None, None, None) =>
            include_str!("../modules/deviations/ietf-segment-routing-mpls-holo-deviations.yang"),
    }
});

// All modules currently implemented.
//
// The list includes modules that define YANG identities that can be
// instantiated.
pub static YANG_IMPLEMENTED_MODULES: Lazy<Vec<&'static str>> =
    Lazy::new(|| {
        vec![
            "iana-if-type",
            "ietf-bfd-ip-mh",
            "ietf-bfd-ip-sh",
            "ietf-bfd-types",
            "ietf-bfd",
            "ietf-routing-types",
            "ietf-interfaces",
            "ietf-ip",
            "ietf-key-chain",
            "ietf-routing",
            "ietf-segment-routing",
            "ietf-segment-routing-common",
            "ietf-segment-routing-mpls",
            "ietf-mpls-ldp",
            "ietf-ospf",
            "ietf-ospf-sr",
            "ietf-ospfv3-extended-lsa",
            "ietf-rip",
            "holo-ospf",
            "holo-ospf-dev",
        ]
    });

// All features currently supported.
pub static YANG_FEATURES: Lazy<HashMap<&'static str, Vec<&'static str>>> =
    Lazy::new(|| {
        hashmap! {
            "ietf-bfd-types" => vec![
                "client-base-cfg-parms",
                "single-minimum-interval",
            ],
            "ietf-key-chain" => vec![
                "hex-key-string",
                "independent-send-accept-lifetime",
            ],
            "ietf-ospf" => vec![
                "bfd",
                "explicit-router-id",
                "graceful-restart",
                "ietf-spf-delay",
                "key-chain",
                "max-ecmp",
                "mtu-ignore",
                "ospfv3-authentication-trailer",
                "stub-router",
            ],
            "ietf-rip" => vec![
                "explicit-neighbors",
                "global-statistics",
                "interface-statistics",
            ],
            "ietf-segment-routing-common" => vec![
                "sid-last-hop-behavior",
            ],
        }
    });

//
// YANG conversion traits.
//

pub trait ToYang {
    // Return YANG textual representation of the value.
    fn to_yang(&self) -> String;
}

pub trait ToYangBits {
    // Return vector representing YANG bit set.
    fn to_yang_bits(&self) -> Vec<&'static str>;
}

pub trait TryFromYang: Sized {
    // Construct value from YANG identity or enum value.
    fn try_from_yang(identity: &str) -> Option<Self>;
}

//
// YANG path type.
//
// Instances of this structure are created automatically at build-time, and
// their use should be preferred over regular strings for extra type safety.
//
#[derive(Clone, Copy, Debug)]
pub struct YangPath(&'static str);

// ===== impl YangPath =====

impl YangPath {
    pub const fn new(path: &'static str) -> YangPath {
        YangPath(path)
    }

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl std::fmt::Display for YangPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ===== global functions =====

// Creates empty YANG context.
pub fn new_context() -> Context {
    let mut ctx = Context::new(
        ContextFlags::NO_YANGLIBRARY | ContextFlags::DISABLE_SEARCHDIRS,
    )
    .expect("Failed to create YANG context");
    ctx.set_embedded_modules(&YANG_EMBEDDED_MODULES);
    ctx
}

// Loads a YANG module.
pub fn load_module(ctx: &mut Context, name: &str) {
    let features = YANG_FEATURES
        .get(name)
        .map(|features| features.as_slice())
        .unwrap_or_else(|| &[]);
    if let Err(error) = ctx.load_module(name, None, features) {
        error!(%error, "failed to load YANG module");
        std::process::exit(1);
    }
}

// Loads a YANG deviations module.
pub fn load_deviations(ctx: &mut Context, name: &str) {
    let name = format!("{}-holo-deviations", name);
    // Ignore errors since the deviation module might not exist.
    let _ = ctx.load_module(&name, None, &[]);
}
