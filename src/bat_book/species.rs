//! Bat species catalog — all species and families defined once, reused across books.
//!
//! Each constant represents a base species or family. Regional bat books
//! reference these via `BookEntryDef` and can override description/name.

use super::types::BatSpecies;

// ══════════════════════════════════════════════════════════════════════════════
// Family-level entries (used by the Global book)
// ══════════════════════════════════════════════════════════════════════════════
//
// Sources:
// - Jones & Barlow (2004) JEB: Scaling of echolocation call parameters
// - Jung et al. (2014) PMC: Molossidae call design
// - Shi et al. (2024) PMC: Correlated evolution body size & echolocation
// - Jones & Rayner (1989) Springer: Horseshoe bat foraging ecology
// - Collen (2012) BioOne: Rhinolophidae & Hipposideridae comparative ecology

pub const RHINOLOPHIDAE: BatSpecies = BatSpecies {
    id: "rhinolophidae",
    name: "Horseshoe bats",
    scientific_name: "",
    family: "Rhinolophidae",
    call_type: "CF",
    freq_lo_hz: 30_000.0,
    freq_hi_hz: 120_000.0,
    description: "Constant-frequency calls; species range ~30 kHz (large) to ~112 kHz (lesser horseshoe)",
    echolocates: true,
};

pub const HIPPOSIDERIDAE: BatSpecies = BatSpecies {
    id: "hipposideridae",
    name: "Roundleaf bats",
    scientific_name: "",
    family: "Hipposideridae",
    call_type: "CF",
    freq_lo_hz: 60_000.0,
    freq_hi_hz: 160_000.0,
    description: "CF calls; Cleotis percivalis reaches 212 kHz, the highest known bat frequency",
    echolocates: true,
};

pub const VESPERTILIONIDAE: BatSpecies = BatSpecies {
    id: "vespertilionidae",
    name: "Vesper bats",
    scientific_name: "",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 15_000.0,
    freq_hi_hz: 120_000.0,
    description: "Broadest family; FM sweeps; most species 20\u{2013}60 kHz peak",
    echolocates: true,
};

pub const MOLOSSIDAE: BatSpecies = BatSpecies {
    id: "molossidae",
    name: "Free-tailed bats",
    scientific_name: "",
    family: "Molossidae",
    call_type: "QCF",
    freq_lo_hz: 10_000.0,
    freq_hi_hz: 45_000.0,
    description: "Narrowband, long-duration QCF calls; 16\u{2013}44 kHz peak typical",
    echolocates: true,
};

pub const EMBALLONURIDAE: BatSpecies = BatSpecies {
    id: "emballonuridae",
    name: "Sheath-tailed bats",
    scientific_name: "",
    family: "Emballonuridae",
    call_type: "QCF",
    freq_lo_hz: 20_000.0,
    freq_hi_hz: 55_000.0,
    description: "Quasi-constant-frequency calls; some species sweep 40\u{2013}100 kHz",
    echolocates: true,
};

pub const PHYLLOSTOMIDAE: BatSpecies = BatSpecies {
    id: "phyllostomidae",
    name: "Leaf-nosed bats",
    scientific_name: "",
    family: "Phyllostomidae",
    call_type: "FM",
    freq_lo_hz: 40_000.0,
    freq_hi_hz: 120_000.0,
    description: "Low-intensity \"whispering\" bats; multi-harmonic FM calls",
    echolocates: true,
};

pub const MORMOOPIDAE: BatSpecies = BatSpecies {
    id: "mormoopidae",
    name: "Ghost-faced bats",
    scientific_name: "",
    family: "Mormoopidae",
    call_type: "CF-FM",
    freq_lo_hz: 45_000.0,
    freq_hi_hz: 65_000.0,
    description: "P. parnellii CF at ~63 kHz with FM sweep to ~54 kHz",
    echolocates: true,
};

pub const MINIOPTERIDAE: BatSpecies = BatSpecies {
    id: "miniopteridae",
    name: "Bent-winged bats",
    scientific_name: "",
    family: "Miniopteridae",
    call_type: "FM",
    freq_lo_hz: 45_000.0,
    freq_hi_hz: 85_000.0,
    description: "FM-dominated calls; formerly classified within Vespertilionidae",
    echolocates: true,
};

pub const NYCTERIDAE: BatSpecies = BatSpecies {
    id: "nycteridae",
    name: "Slit-faced bats",
    scientific_name: "",
    family: "Nycteridae",
    call_type: "FM",
    freq_lo_hz: 30_000.0,
    freq_hi_hz: 80_000.0,
    description: "Low-intensity, multi-harmonic FM calls emitted through nostrils",
    echolocates: true,
};

pub const MEGADERMATIDAE: BatSpecies = BatSpecies {
    id: "megadermatidae",
    name: "False vampires",
    scientific_name: "",
    family: "Megadermatidae",
    call_type: "FM",
    freq_lo_hz: 20_000.0,
    freq_hi_hz: 110_000.0,
    description: "Low-intensity broadband FM; large carnivorous bats",
    echolocates: true,
};

pub const PTEROPODIDAE: BatSpecies = BatSpecies {
    id: "pteropodidae",
    name: "Fruit bats",
    scientific_name: "",
    family: "Pteropodidae",
    call_type: "clicks",
    freq_lo_hz: 10_000.0,
    freq_hi_hz: 100_000.0,
    description: "Most don't echolocate; Rousettus uses tongue clicks for cave navigation",
    echolocates: false,
};

// ══════════════════════════════════════════════════════════════════════════════
// Species: Victoria, Australia
// ══════════════════════════════════════════════════════════════════════════════
//
// Sources:
// - Batica: Microbat Call Identification Assistant (Bayside, VIC)
// - SWIFFT: Insectivorous bats of Victoria
// - Milne (2002): Key to the Bat Calls of the Top End of the NT
// - Wikipedia: List of bats of Australia
// - Museums Victoria, Atlas of Living Australia

pub const CHALINOLOBUS_GOULDII: BatSpecies = BatSpecies {
    id: "chalinolobus_gouldii",
    name: "Gould's Wattled Bat",
    scientific_name: "Chalinolobus gouldii",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 25_000.0,
    freq_hi_hz: 34_000.0,
    description: "Widespread and abundant across Australia. Roosts in tree hollows, buildings, and bat boxes. Alternating call frequencies are distinctive.",
    echolocates: true,
};

pub const CHALINOLOBUS_MORIO: BatSpecies = BatSpecies {
    id: "chalinolobus_morio",
    name: "Chocolate Wattled Bat",
    scientific_name: "Chalinolobus morio",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 48_000.0,
    freq_hi_hz: 53_000.0,
    description: "Common across southern Australia. Small, dark bat roosting in tree hollows and buildings. Higher frequency calls than Gould's Wattled Bat.",
    echolocates: true,
};

pub const NYCTOPHILUS_GEOFFROYI: BatSpecies = BatSpecies {
    id: "nyctophilus_geoffroyi",
    name: "Lesser Long-eared Bat",
    scientific_name: "Nyctophilus geoffroyi",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 35_000.0,
    freq_hi_hz: 80_000.0,
    description: "Australia's most widespread bat. Very quiet, broadband FM calls; often difficult to detect acoustically. Gleaning insectivore with large ears.",
    echolocates: true,
};

pub const AUSTRONOMUS_AUSTRALIS: BatSpecies = BatSpecies {
    id: "austronomus_australis",
    name: "White-striped Free-tailed Bat",
    scientific_name: "Austronomus australis",
    family: "Molossidae",
    call_type: "QCF",
    freq_lo_hz: 10_000.0,
    freq_hi_hz: 15_000.0,
    description: "Australia's largest insectivorous bat. Loud, low-frequency calls audible to some humans. Fast, high-flying open-air forager.",
    echolocates: true,
};

pub const VESPADELUS_VULTURNUS: BatSpecies = BatSpecies {
    id: "vespadelus_vulturnus",
    name: "Little Forest Bat",
    scientific_name: "Vespadelus vulturnus",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 45_000.0,
    freq_hi_hz: 53_000.0,
    description: "One of Australia's smallest bats (~4 g). Common in forests and urban areas. High-frequency calls.",
    echolocates: true,
};

pub const VESPADELUS_REGULUS: BatSpecies = BatSpecies {
    id: "vespadelus_regulus",
    name: "Southern Forest Bat",
    scientific_name: "Vespadelus regulus",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 40_000.0,
    freq_hi_hz: 55_000.0,
    description: "Small forest bat found across southern Australia. Roosts in tree hollows. Call frequency overlaps with Little Forest Bat.",
    echolocates: true,
};

pub const NYCTOPHILUS_GOULDI: BatSpecies = BatSpecies {
    id: "nyctophilus_gouldi",
    name: "Gould's Long-eared Bat",
    scientific_name: "Nyctophilus gouldi",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 35_000.0,
    freq_hi_hz: 80_000.0,
    description: "Common in forests of eastern Australia. Very quiet calls, similar to Lesser Long-eared Bat. Distinguished by larger size and habitat preference.",
    echolocates: true,
};

pub const VESPADELUS_DARLINGTONI: BatSpecies = BatSpecies {
    id: "vespadelus_darlingtoni",
    name: "Large Forest Bat",
    scientific_name: "Vespadelus darlingtoni",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 38_000.0,
    freq_hi_hz: 46_000.0,
    description: "Largest Vespadelus species. Found in wet and dry forests of south-eastern Australia including Tasmania.",
    echolocates: true,
};

pub const MINIOPTERUS_ORIANAE_OCEANENSIS: BatSpecies = BatSpecies {
    id: "miniopterus_orianae_oceanensis",
    name: "Eastern Bent-wing Bat",
    scientific_name: "Miniopterus orianae oceanensis",
    family: "Miniopteridae",
    call_type: "FM",
    freq_lo_hz: 43_000.0,
    freq_hi_hz: 48_000.0,
    description: "Cave-roosting bat found along eastern Australia. Fast, agile flier.",
    echolocates: true,
};

pub const OZIMOPS_PLANICEPS: BatSpecies = BatSpecies {
    id: "ozimops_planiceps",
    name: "Southern Free-tailed Bat",
    scientific_name: "Ozimops planiceps",
    family: "Molossidae",
    call_type: "QCF",
    freq_lo_hz: 25_000.0,
    freq_hi_hz: 29_000.0,
    description: "Small free-tailed bat of south-eastern Australia. Roosts in tree hollows and buildings. Rapid, direct flight.",
    echolocates: true,
};

pub const OZIMOPS_RIDEI: BatSpecies = BatSpecies {
    id: "ozimops_ridei",
    name: "Ride's Free-tailed Bat",
    scientific_name: "Ozimops ridei",
    family: "Molossidae",
    call_type: "QCF",
    freq_lo_hz: 30_000.0,
    freq_hi_hz: 35_000.0,
    description: "Widespread across eastern Australian coasts. Similar to Southern Free-tailed Bat but slightly higher frequency calls.",
    echolocates: true,
};

pub const FALSISTRELLUS_TASMANIENSIS: BatSpecies = BatSpecies {
    id: "falsistrellus_tasmaniensis",
    name: "Eastern Falsistrelle",
    scientific_name: "Falsistrellus tasmaniensis",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 34_000.0,
    freq_hi_hz: 39_000.0,
    description: "Large vesper bat of south-eastern forests. Roosts in tree hollows. Vulnerable (IUCN). Distinctive mid-range frequency calls.",
    echolocates: true,
};

pub const SCOTOREPENS_ORION: BatSpecies = BatSpecies {
    id: "scotorepens_orion",
    name: "Eastern Broad-nosed Bat",
    scientific_name: "Scotorepens orion",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 34_500.0,
    freq_hi_hz: 37_500.0,
    description: "Robust bat of south-eastern coastal forests. Narrow frequency range distinctive. Roosts in tree hollows.",
    echolocates: true,
};

pub const SCOTOREPENS_BALSTONI: BatSpecies = BatSpecies {
    id: "scotorepens_balstoni",
    name: "Inland Broad-nosed Bat",
    scientific_name: "Scotorepens balstoni",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 28_000.0,
    freq_hi_hz: 34_000.0,
    description: "Widespread across inland Australia. Found in drier regions. Similar frequency to Gould's Wattled Bat.",
    echolocates: true,
};

pub const MYOTIS_MACROPUS: BatSpecies = BatSpecies {
    id: "myotis_macropus",
    name: "Large-footed Myotis",
    scientific_name: "Myotis macropus",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 35_000.0,
    freq_hi_hz: 80_000.0,
    description: "Australia's only fishing bat. Trawls water surfaces with large feet. Found near rivers, lakes, and dams. Very quiet calls.",
    echolocates: true,
};

pub const SACCOLAIMUS_FLAVIVENTRIS: BatSpecies = BatSpecies {
    id: "saccolaimus_flaviventris",
    name: "Yellow-bellied Sheathtail Bat",
    scientific_name: "Saccolaimus flaviventris",
    family: "Emballonuridae",
    call_type: "QCF",
    freq_lo_hz: 17_500.0,
    freq_hi_hz: 22_500.0,
    description: "Large, fast-flying bat with glossy black fur and yellow belly. Migratory. High-altitude forager.",
    echolocates: true,
};

pub const RHINOLOPHUS_MEGAPHYLLUS: BatSpecies = BatSpecies {
    id: "rhinolophus_megaphyllus",
    name: "Eastern Horseshoe Bat",
    scientific_name: "Rhinolophus megaphyllus",
    family: "Rhinolophidae",
    call_type: "CF",
    freq_lo_hz: 67_000.0,
    freq_hi_hz: 71_000.0,
    description: "Constant-frequency echolocation using distinctive horseshoe-shaped noseleaf. Cave-roosting. Found in forests of eastern Australia.",
    echolocates: true,
};

pub const PTEROPUS_POLIOCEPHALUS: BatSpecies = BatSpecies {
    id: "pteropus_poliocephalus",
    name: "Grey-headed Flying-fox",
    scientific_name: "Pteropus poliocephalus",
    family: "Pteropodidae",
    call_type: "none",
    freq_lo_hz: 0.0,
    freq_hi_hz: 0.0,
    description: "Australia's largest bat (wingspan ~1 m). Does not echolocate. Camps in colonies along waterways. Vulnerable (EPBC Act). Pollinator and seed disperser.",
    echolocates: false,
};

pub const NYCTOPHILUS_MAJOR: BatSpecies = BatSpecies {
    id: "nyctophilus_major",
    name: "Greater Long-eared Bat",
    scientific_name: "Nyctophilus major",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 35_000.0,
    freq_hi_hz: 65_000.0,
    description: "Formerly N. timoriensis. Restricted to drier woodlands. Very quiet gleaning calls. Vulnerable.",
    echolocates: true,
};

pub const VESPADELUS_BAVERSTOCKI: BatSpecies = BatSpecies {
    id: "vespadelus_baverstocki",
    name: "Inland Forest Bat",
    scientific_name: "Vespadelus baverstocki",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 45_000.0,
    freq_hi_hz: 50_000.0,
    description: "Small bat of inland Australia. Restricted to semi-arid regions.",
    echolocates: true,
};

pub const SCOTEANAX_RUEPPELLII: BatSpecies = BatSpecies {
    id: "scoteanax_rueppellii",
    name: "Greater Broad-nosed Bat",
    scientific_name: "Scoteanax rueppellii",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 30_000.0,
    freq_hi_hz: 38_000.0,
    description: "Large, robust bat of eastern coastal forests. Aggressive predator of large insects and small vertebrates.",
    echolocates: true,
};

pub const MINIOPTERUS_ORIANAE_BASSANII: BatSpecies = BatSpecies {
    id: "miniopterus_orianae_bassanii",
    name: "Southern Bent-wing Bat",
    scientific_name: "Miniopterus orianae bassanii",
    family: "Miniopteridae",
    call_type: "FM",
    freq_lo_hz: 43_000.0,
    freq_hi_hz: 48_000.0,
    description: "Critically Endangered (EPBC Act). Dependent on a single maternity cave. Population critically low.",
    echolocates: true,
};

pub const NYCTOPHILUS_CORBENI: BatSpecies = BatSpecies {
    id: "nyctophilus_corbeni",
    name: "South-eastern Long-eared Bat",
    scientific_name: "Nyctophilus corbeni",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 35_000.0,
    freq_hi_hz: 65_000.0,
    description: "Vulnerable (EPBC Act). Extremely rare; restricted to northwest woodlands.",
    echolocates: true,
};

pub const PTEROPUS_SCAPULATUS: BatSpecies = BatSpecies {
    id: "pteropus_scapulatus",
    name: "Little Red Flying-fox",
    scientific_name: "Pteropus scapulatus",
    family: "Pteropodidae",
    call_type: "none",
    freq_lo_hz: 0.0,
    freq_hi_hz: 0.0,
    description: "Does not echolocate. Nomadic, following eucalypt flowering. Seasonal visitor to southern regions.",
    echolocates: false,
};

// ══════════════════════════════════════════════════════════════════════════════
// Species: Europe
// ══════════════════════════════════════════════════════════════════════════════
//
// Sources:
// - Dietz, Helversen & Nill (2009): Bats of Britain, Europe and Northwest Africa
// - Russ (2012): British Bat Calls: A Guide to Species Identification
// - Barataud (2015): Acoustic Ecology of European Bats
// - Middleton, Froud & French (2014): Social Calls of the Bats of Britain and Ireland

pub const PIPISTRELLUS_PIPISTRELLUS: BatSpecies = BatSpecies {
    id: "pipistrellus_pipistrellus",
    name: "Common Pipistrelle",
    scientific_name: "Pipistrellus pipistrellus",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 42_000.0,
    freq_hi_hz: 51_000.0,
    description: "Europe's most abundant bat. Characteristic frequency ~45 kHz separates it from soprano pipistrelle. Roosts in buildings; forages along edges and over water.",
    echolocates: true,
};

pub const PIPISTRELLUS_PYGMAEUS: BatSpecies = BatSpecies {
    id: "pipistrellus_pygmaeus",
    name: "Soprano Pipistrelle",
    scientific_name: "Pipistrellus pygmaeus",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 51_000.0,
    freq_hi_hz: 60_000.0,
    description: "Cryptic species split from common pipistrelle in 1999. Characteristic frequency ~55 kHz. Strongly associated with riparian habitats.",
    echolocates: true,
};

pub const PIPISTRELLUS_NATHUSII: BatSpecies = BatSpecies {
    id: "pipistrellus_nathusii",
    name: "Nathusius' Pipistrelle",
    scientific_name: "Pipistrellus nathusii",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 35_000.0,
    freq_hi_hz: 42_000.0,
    description: "Long-distance migrant; travels up to 2,000 km. Characteristic frequency ~38 kHz. Favours wetlands and riparian woodland.",
    echolocates: true,
};

pub const PIPISTRELLUS_KUHLII: BatSpecies = BatSpecies {
    id: "pipistrellus_kuhlii",
    name: "Kuhl's Pipistrelle",
    scientific_name: "Pipistrellus kuhlii",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 35_000.0,
    freq_hi_hz: 45_000.0,
    description: "Expanding northward across Europe. Characteristic frequency ~40 kHz. Common around buildings and street lights in Mediterranean regions.",
    echolocates: true,
};

pub const MYOTIS_DAUBENTONII: BatSpecies = BatSpecies {
    id: "myotis_daubentonii",
    name: "Daubenton's Bat",
    scientific_name: "Myotis daubentonii",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 32_000.0,
    freq_hi_hz: 85_000.0,
    description: "Forages low over calm water, trawling insects from the surface. Steep FM sweeps. Often seen along canals and rivers at dusk.",
    echolocates: true,
};

pub const MYOTIS_NATTERERI: BatSpecies = BatSpecies {
    id: "myotis_nattereri",
    name: "Natterer's Bat",
    scientific_name: "Myotis nattereri",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 23_000.0,
    freq_hi_hz: 115_000.0,
    description: "Very broadband FM calls with prominent harmonics. Gleaning specialist, picks prey from foliage and walls. Roosts in old buildings and tree holes.",
    echolocates: true,
};

pub const MYOTIS_MYSTACINUS: BatSpecies = BatSpecies {
    id: "myotis_mystacinus",
    name: "Whiskered Bat",
    scientific_name: "Myotis mystacinus",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 32_000.0,
    freq_hi_hz: 80_000.0,
    description: "Small Myotis often found in villages and woodland edges. Very similar in call and appearance to Brandt's bat; confirmed by handling or genetics.",
    echolocates: true,
};

pub const MYOTIS_BRANDTII: BatSpecies = BatSpecies {
    id: "myotis_brandtii",
    name: "Brandt's Bat",
    scientific_name: "Myotis brandtii",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 28_000.0,
    freq_hi_hz: 80_000.0,
    description: "Closely related to whiskered bat; prefers mature woodland. Slightly lower frequency calls. Identified reliably only by dentition or genetics.",
    echolocates: true,
};

pub const MYOTIS_MYOTIS: BatSpecies = BatSpecies {
    id: "myotis_myotis",
    name: "Greater Mouse-eared Bat",
    scientific_name: "Myotis myotis",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 25_000.0,
    freq_hi_hz: 80_000.0,
    description: "One of Europe's largest vespertilionids. Ground-gleaning specialist hunting beetles in short grass and forest floors. Large nursery colonies in roofs and caves.",
    echolocates: true,
};

pub const MYOTIS_BECHSTEINII: BatSpecies = BatSpecies {
    id: "myotis_bechsteinii",
    name: "Bechstein's Bat",
    scientific_name: "Myotis bechsteinii",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 30_000.0,
    freq_hi_hz: 100_000.0,
    description: "Rare woodland specialist with very quiet, broadband calls. Indicator species for old-growth forest. Roosts in tree holes; rarely found in buildings.",
    echolocates: true,
};

pub const MYOTIS_DASYCNEME: BatSpecies = BatSpecies {
    id: "myotis_dasycneme",
    name: "Pond Bat",
    scientific_name: "Myotis dasycneme",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 30_000.0,
    freq_hi_hz: 65_000.0,
    description: "Larger relative of Daubenton's bat. Trawls over broad lakes and slow rivers. Vulnerable across most of its range in NW Europe.",
    echolocates: true,
};

pub const NYCTALUS_NOCTULA: BatSpecies = BatSpecies {
    id: "nyctalus_noctula",
    name: "Common Noctule",
    scientific_name: "Nyctalus noctula",
    family: "Vespertilionidae",
    call_type: "QCF",
    freq_lo_hz: 18_000.0,
    freq_hi_hz: 25_000.0,
    description: "Large, fast-flying bat. Loud, narrowband calls audible on bat detectors at distance. Roosts in tree holes; one of the first species to emerge at dusk.",
    echolocates: true,
};

pub const NYCTALUS_LEISLERI: BatSpecies = BatSpecies {
    id: "nyctalus_leisleri",
    name: "Leisler's Bat",
    scientific_name: "Nyctalus leisleri",
    family: "Vespertilionidae",
    call_type: "QCF",
    freq_lo_hz: 22_000.0,
    freq_hi_hz: 30_000.0,
    description: "Smaller noctule with slightly higher frequency calls. Fast open-air forager. Migratory in parts of its range. Common in Ireland.",
    echolocates: true,
};

pub const NYCTALUS_LASIOPTERUS: BatSpecies = BatSpecies {
    id: "nyctalus_lasiopterus",
    name: "Greater Noctule",
    scientific_name: "Nyctalus lasiopterus",
    family: "Vespertilionidae",
    call_type: "QCF",
    freq_lo_hz: 14_000.0,
    freq_hi_hz: 20_000.0,
    description: "Europe's largest bat. Occasionally catches small birds in flight during nocturnal migration. Very low-frequency calls. Rare across its range.",
    echolocates: true,
};

pub const EPTESICUS_SEROTINUS: BatSpecies = BatSpecies {
    id: "eptesicus_serotinus",
    name: "Serotine",
    scientific_name: "Eptesicus serotinus",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 22_000.0,
    freq_hi_hz: 55_000.0,
    description: "Large bat with broad FM sweeps. One of the last to emerge, often foraging along tree lines and around street lights. Roosts almost exclusively in buildings.",
    echolocates: true,
};

pub const EPTESICUS_NILSSONII: BatSpecies = BatSpecies {
    id: "eptesicus_nilssonii",
    name: "Northern Bat",
    scientific_name: "Eptesicus nilssonii",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 25_000.0,
    freq_hi_hz: 45_000.0,
    description: "The world's northernmost bat, found above the Arctic Circle. Tolerates cold climates. Common in Scandinavia and mountain regions of central Europe.",
    echolocates: true,
};

pub const PLECOTUS_AURITUS: BatSpecies = BatSpecies {
    id: "plecotus_auritus",
    name: "Brown Long-eared Bat",
    scientific_name: "Plecotus auritus",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 25_000.0,
    freq_hi_hz: 85_000.0,
    description: "Iconic enormous ears. Very quiet, broadband calls; often called a \"whispering bat\". Gleaning specialist in woodland. Roosts in buildings and tree holes.",
    echolocates: true,
};

pub const PLECOTUS_AUSTRIACUS: BatSpecies = BatSpecies {
    id: "plecotus_austriacus",
    name: "Grey Long-eared Bat",
    scientific_name: "Plecotus austriacus",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 22_000.0,
    freq_hi_hz: 50_000.0,
    description: "Prefers warmer lowland areas. Quieter and slightly lower frequency than brown long-eared. Difficult to distinguish visually; confirmed by tragus shape.",
    echolocates: true,
};

pub const BARBASTELLA_BARBASTELLUS: BatSpecies = BatSpecies {
    id: "barbastella_barbastellus",
    name: "Barbastelle",
    scientific_name: "Barbastella barbastellus",
    family: "Vespertilionidae",
    call_type: "FM",
    freq_lo_hz: 30_000.0,
    freq_hi_hz: 45_000.0,
    description: "Distinctive flat face with upturned nose. Alternating call frequencies (~32 and ~34 kHz). Specialist moth-hunter; forest dependent. Rare across most of its range.",
    echolocates: true,
};

pub const VESPERTILIO_MURINUS: BatSpecies = BatSpecies {
    id: "vespertilio_murinus",
    name: "Parti-coloured Bat",
    scientific_name: "Vespertilio murinus",
    family: "Vespertilionidae",
    call_type: "QCF",
    freq_lo_hz: 22_000.0,
    freq_hi_hz: 30_000.0,
    description: "Striking frosted fur. Alternating call frequencies. Migratory; roosts in high-rise buildings. Males produce audible courtship calls from roost entrances.",
    echolocates: true,
};

pub const RHINOLOPHUS_FERRUMEQUINUM: BatSpecies = BatSpecies {
    id: "rhinolophus_ferrumequinum",
    name: "Greater Horseshoe Bat",
    scientific_name: "Rhinolophus ferrumequinum",
    family: "Rhinolophidae",
    call_type: "CF",
    freq_lo_hz: 78_000.0,
    freq_hi_hz: 84_000.0,
    description: "Europe's largest horseshoe bat. Constant-frequency call at ~83 kHz. Hunts large beetles and moths in flight. Roosts in caves, mines, and old buildings.",
    echolocates: true,
};

pub const RHINOLOPHUS_HIPPOSIDEROS: BatSpecies = BatSpecies {
    id: "rhinolophus_hipposideros",
    name: "Lesser Horseshoe Bat",
    scientific_name: "Rhinolophus hipposideros",
    family: "Rhinolophidae",
    call_type: "CF",
    freq_lo_hz: 105_000.0,
    freq_hi_hz: 115_000.0,
    description: "One of Europe's smallest bats (~5 g). CF call at ~110 kHz. Forages close to vegetation in sheltered valleys. Very sensitive to disturbance at roost sites.",
    echolocates: true,
};

pub const RHINOLOPHUS_EURYALE: BatSpecies = BatSpecies {
    id: "rhinolophus_euryale",
    name: "Mediterranean Horseshoe Bat",
    scientific_name: "Rhinolophus euryale",
    family: "Rhinolophidae",
    call_type: "CF",
    freq_lo_hz: 100_000.0,
    freq_hi_hz: 108_000.0,
    description: "Medium-sized horseshoe bat. CF call at ~104 kHz. Restricted to Mediterranean and warm-temperate zones. Cave-dwelling; large colony roosts.",
    echolocates: true,
};

pub const TADARIDA_TENIOTIS: BatSpecies = BatSpecies {
    id: "tadarida_teniotis",
    name: "European Free-tailed Bat",
    scientific_name: "Tadarida teniotis",
    family: "Molossidae",
    call_type: "QCF",
    freq_lo_hz: 10_000.0,
    freq_hi_hz: 18_000.0,
    description: "Europe's only free-tailed bat. Loud, low-frequency calls audible to humans. Fast, high-altitude forager. Roosts in cliff crevices and tall buildings.",
    echolocates: true,
};

pub const MINIOPTERUS_SCHREIBERSII: BatSpecies = BatSpecies {
    id: "miniopterus_schreibersii",
    name: "Common Bent-wing Bat",
    scientific_name: "Miniopterus schreibersii",
    family: "Miniopteridae",
    call_type: "FM",
    freq_lo_hz: 47_000.0,
    freq_hi_hz: 57_000.0,
    description: "Fast, agile cave-dweller found across southern Europe. Long, narrow wings for sustained flight. Large colonies; sensitive to cave disturbance.",
    echolocates: true,
};
