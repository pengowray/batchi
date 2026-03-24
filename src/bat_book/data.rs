use super::types::{BookEntryDef, BatBookManifest, BatBookRegion, Commonness};
use super::species;

// ══════════════════════════════════════════════════════════════════════════════
// Global book — family-level entries
// ══════════════════════════════════════════════════════════════════════════════

const GLOBAL_BOOK: &[BookEntryDef] = &[
    BookEntryDef { species: &species::RHINOLOPHIDAE, commonness: None, description: None, name: None },
    BookEntryDef { species: &species::HIPPOSIDERIDAE, commonness: None, description: None, name: None },
    BookEntryDef { species: &species::VESPERTILIONIDAE, commonness: None, description: None, name: None },
    BookEntryDef { species: &species::MOLOSSIDAE, commonness: None, description: None, name: None },
    BookEntryDef { species: &species::EMBALLONURIDAE, commonness: None, description: None, name: None },
    BookEntryDef { species: &species::PHYLLOSTOMIDAE, commonness: None, description: None, name: None },
    BookEntryDef { species: &species::MORMOOPIDAE, commonness: None, description: None, name: None },
    BookEntryDef { species: &species::MINIOPTERIDAE, commonness: None, description: None, name: None },
    BookEntryDef { species: &species::NYCTERIDAE, commonness: None, description: None, name: None },
    BookEntryDef { species: &species::MEGADERMATIDAE, commonness: None, description: None, name: None },
    // Non-echolocating (will be sorted to end by get_manifest)
    BookEntryDef { species: &species::PTEROPODIDAE, commonness: None, description: None, name: None },
];

// ══════════════════════════════════════════════════════════════════════════════
// VIC, Australia — species-level entries sorted by commonness
// ══════════════════════════════════════════════════════════════════════════════

const VIC_AUSTRALIA_BOOK: &[BookEntryDef] = &[
    // ── Very Common ──────────────────────────────────────────────
    BookEntryDef {
        species: &species::CHALINOLOBUS_GOULDII,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Widespread and abundant across Victoria. Roosts in tree hollows, buildings, and bat boxes. Alternating call frequencies are distinctive."),
        name: None,
    },
    BookEntryDef {
        species: &species::CHALINOLOBUS_MORIO,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Common across southern Australia. Small, dark bat roosting in tree hollows and buildings. Higher frequency calls than Gould's Wattled Bat."),
        name: None,
    },
    BookEntryDef {
        species: &species::NYCTOPHILUS_GEOFFROYI,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Australia's most widespread bat. Very quiet, broadband FM calls; often difficult to detect acoustically. Gleaning insectivore with large ears."),
        name: None,
    },
    BookEntryDef {
        species: &species::AUSTRONOMUS_AUSTRALIS,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Australia's largest insectivorous bat. Loud, low-frequency calls audible to some humans. Fast, high-flying open-air forager."),
        name: None,
    },
    BookEntryDef {
        species: &species::VESPADELUS_VULTURNUS,
        commonness: Some(Commonness::VeryCommon),
        description: Some("One of Australia's smallest bats (~4 g). Common in forests and urban areas throughout Victoria. High-frequency calls."),
        name: None,
    },
    // ── Common ───────────────────────────────────────────────────
    BookEntryDef {
        species: &species::VESPADELUS_REGULUS,
        commonness: Some(Commonness::Common),
        description: Some("Small forest bat found across southern Australia. Roosts in tree hollows. Call frequency overlaps with Little Forest Bat."),
        name: None,
    },
    BookEntryDef {
        species: &species::NYCTOPHILUS_GOULDI,
        commonness: Some(Commonness::Common),
        description: Some("Common in forests of eastern Australia. Very quiet calls, similar to Lesser Long-eared Bat. Distinguished by larger size and habitat preference."),
        name: None,
    },
    BookEntryDef {
        species: &species::VESPADELUS_DARLINGTONI,
        commonness: Some(Commonness::Common),
        description: Some("Largest Vespadelus species. Found in wet and dry forests of south-eastern Australia including Tasmania."),
        name: None,
    },
    BookEntryDef {
        species: &species::MINIOPTERUS_ORIANAE_OCEANENSIS,
        commonness: Some(Commonness::Common),
        description: Some("Cave-roosting bat found along eastern Australia. Fast, agile flier. Maternity cave near Bairnsdale. Vulnerable in Victoria."),
        name: None,
    },
    BookEntryDef {
        species: &species::OZIMOPS_PLANICEPS,
        commonness: Some(Commonness::Common),
        description: Some("Small free-tailed bat of south-eastern Australia. Roosts in tree hollows and buildings. Rapid, direct flight."),
        name: None,
    },
    BookEntryDef {
        species: &species::OZIMOPS_RIDEI,
        commonness: Some(Commonness::Common),
        description: Some("Widespread across eastern Australian coasts. Similar to Southern Free-tailed Bat but slightly higher frequency calls."),
        name: None,
    },
    // ── Uncommon ─────────────────────────────────────────────────
    BookEntryDef {
        species: &species::FALSISTRELLUS_TASMANIENSIS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Large vesper bat of south-eastern forests. Roosts in tree hollows. Vulnerable (IUCN). Distinctive mid-range frequency calls."),
        name: None,
    },
    BookEntryDef {
        species: &species::SCOTOREPENS_ORION,
        commonness: Some(Commonness::Uncommon),
        description: Some("Robust bat of south-eastern coastal forests. Narrow frequency range distinctive. Roosts in tree hollows."),
        name: None,
    },
    BookEntryDef {
        species: &species::SCOTOREPENS_BALSTONI,
        commonness: Some(Commonness::Uncommon),
        description: Some("Widespread across inland Australia. Found in drier regions of northern and western Victoria. Similar frequency to Gould's Wattled Bat."),
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_MACROPUS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Australia's only fishing bat. Trawls water surfaces with large feet. Found near rivers, lakes, and dams. Very quiet calls."),
        name: None,
    },
    BookEntryDef {
        species: &species::SACCOLAIMUS_FLAVIVENTRIS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Large, fast-flying bat with glossy black fur and yellow belly. Migratory; visits Victoria seasonally. High-altitude forager."),
        name: None,
    },
    BookEntryDef {
        species: &species::RHINOLOPHUS_MEGAPHYLLUS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Constant-frequency echolocation using distinctive horseshoe-shaped noseleaf. Cave-roosting. Found in forests of eastern and southern Victoria."),
        name: None,
    },
    // ── Rare ─────────────────────────────────────────────────────
    BookEntryDef {
        species: &species::NYCTOPHILUS_MAJOR,
        commonness: Some(Commonness::Rare),
        description: Some("Formerly N. timoriensis. Rare in Victoria, restricted to drier woodlands. Very quiet gleaning calls. Vulnerable."),
        name: None,
    },
    BookEntryDef {
        species: &species::VESPADELUS_BAVERSTOCKI,
        commonness: Some(Commonness::Rare),
        description: Some("Small bat of inland Australia. In Victoria, restricted to the semi-arid northwest (Mallee region)."),
        name: None,
    },
    BookEntryDef {
        species: &species::SCOTEANAX_RUEPPELLII,
        commonness: Some(Commonness::Rare),
        description: Some("Large, robust bat of eastern coastal forests. Rare in Victoria, mainly in far east Gippsland. Aggressive predator of large insects and small vertebrates."),
        name: None,
    },
    // ── Endangered ───────────────────────────────────────────────
    BookEntryDef {
        species: &species::MINIOPTERUS_ORIANAE_BASSANII,
        commonness: Some(Commonness::Endangered),
        description: Some("Critically Endangered (EPBC Act). Dependent on a single maternity cave near Warrnambool. Population <50 individuals. Southwest Victoria only."),
        name: None,
    },
    BookEntryDef {
        species: &species::NYCTOPHILUS_CORBENI,
        commonness: Some(Commonness::Endangered),
        description: Some("Vulnerable (EPBC Act). Extremely rare in Victoria; restricted to northwest Mallee (Hattah-Kulkyne, Gunbower). Possibly <50 individuals in VIC."),
        name: None,
    },
    // ── Non-echolocating (sorted to end by get_manifest) ────────
    BookEntryDef {
        species: &species::PTEROPUS_POLIOCEPHALUS,
        commonness: Some(Commonness::Rare),
        description: Some("Australia's largest bat (wingspan ~1 m). Does not echolocate. Camps in colonies along waterways. Vulnerable (EPBC Act). Pollinator and seed disperser."),
        name: None,
    },
    BookEntryDef {
        species: &species::PTEROPUS_SCAPULATUS,
        commonness: Some(Commonness::Vagrant),
        description: Some("Seasonal visitor to northern Victoria. Does not echolocate. Nomadic, following eucalypt flowering. Occasionally camps at Swan Hill and Numurkah."),
        name: None,
    },
];

// ══════════════════════════════════════════════════════════════════════════════
// Europe — species-level entries sorted by commonness
// ══════════════════════════════════════════════════════════════════════════════
//
// Sources:
// - Dietz, Helversen & Nill (2009): Bats of Britain, Europe and Northwest Africa
// - Russ (2012): British Bat Calls: A Guide to Species Identification
// - Barataud (2015): Acoustic Ecology of European Bats

const EUROPE_BOOK: &[BookEntryDef] = &[
    // ── Very Common ──────────────────────────────────────────────
    BookEntryDef {
        species: &species::PIPISTRELLUS_PIPISTRELLUS,
        commonness: Some(Commonness::VeryCommon),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::PIPISTRELLUS_PYGMAEUS,
        commonness: Some(Commonness::VeryCommon),
        description: None,
        name: None,
    },
    // ── Common ───────────────────────────────────────────────────
    BookEntryDef {
        species: &species::PIPISTRELLUS_NATHUSII,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::PIPISTRELLUS_KUHLII,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_DAUBENTONII,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_NATTERERI,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_MYSTACINUS,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_BRANDTII,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_MYOTIS,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::NYCTALUS_NOCTULA,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::NYCTALUS_LEISLERI,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::EPTESICUS_SEROTINUS,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::EPTESICUS_NILSSONII,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::PLECOTUS_AURITUS,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::RHINOLOPHUS_FERRUMEQUINUM,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::RHINOLOPHUS_HIPPOSIDEROS,
        commonness: Some(Commonness::Common),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::MINIOPTERUS_SCHREIBERSII,
        commonness: Some(Commonness::Common),
        description: Some("Fast, agile cave-dweller found across southern Europe. Long, narrow wings. Formerly one species; now split into several. Sensitive to cave disturbance."),
        name: None,
    },
    // ── Uncommon ─────────────────────────────────────────────────
    BookEntryDef {
        species: &species::BARBASTELLA_BARBASTELLUS,
        commonness: Some(Commonness::Uncommon),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::PLECOTUS_AUSTRIACUS,
        commonness: Some(Commonness::Uncommon),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::VESPERTILIO_MURINUS,
        commonness: Some(Commonness::Uncommon),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_BECHSTEINII,
        commonness: Some(Commonness::Uncommon),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_DASYCNEME,
        commonness: Some(Commonness::Uncommon),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::TADARIDA_TENIOTIS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Europe's only free-tailed bat. Loud, low-frequency calls audible to humans. Fast, high-altitude forager. Restricted to the Mediterranean; roosts in cliff crevices and tall buildings."),
        name: None,
    },
    // ── Rare ─────────────────────────────────────────────────────
    BookEntryDef {
        species: &species::RHINOLOPHUS_EURYALE,
        commonness: Some(Commonness::Rare),
        description: None,
        name: None,
    },
    BookEntryDef {
        species: &species::NYCTALUS_LASIOPTERUS,
        commonness: Some(Commonness::Rare),
        description: None,
        name: None,
    },
];

/// Get the bat book manifest for a given region.
///
/// Non-echolocating species are always sorted to the end (stable sort preserves
/// relative order within each group).
pub fn get_manifest(region: BatBookRegion) -> BatBookManifest {
    let book: &[BookEntryDef] = match region {
        BatBookRegion::VicAustralia => VIC_AUSTRALIA_BOOK,
        BatBookRegion::Europe => EUROPE_BOOK,
        _ => GLOBAL_BOOK,
    };
    let mut entries: Vec<_> = book.iter().map(|e| e.materialize()).collect();
    // Stable sort: echolocating first, non-echolocating last
    entries.sort_by_key(|e| if e.echolocates { 0u8 } else { 1 });
    BatBookManifest {
        region: region.short_label().to_string(),
        entries,
    }
}
