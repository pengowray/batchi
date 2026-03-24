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

// ══════════════════════════════════════════════════════════════════════════════
// Costa Rica — species-level entries, tiered by commonness
// ══════════════════════════════════════════════════════════════════════════════
//
// Sources:
// - Jung et al. (2007): Echolocation calls in Central American emballonurids
// - Leiser-Miller & Santana (2021): Phyllostomid echolocation (Costa Rica data)
// - Gessinger et al. (2019): CF-FM echolocation of Lonchorhina aurita
// - Zamora-Gutierrez et al. (2016): Acoustic identification of Mexican bats
// - Rydell et al. (2002): Acoustic identification of Yucatan bats
//
// Phyllostomidae are low-intensity "whispering" echolocators, typically
// detectable only within a few meters. Descriptions note this limitation.

const COSTA_RICA_BOOK: &[BookEntryDef] = &[
    // ── Very Common ──────────────────────────────────────────────
    // Easily detected species with loud calls

    BookEntryDef {
        species: &species::SACCOPTERYX_BILINEATA,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Abundant in lowland forests. Roosts on tree trunks and building walls. Alternates ~45/48 kHz. Vocal learner with complex song repertoire."),
        name: None,
    },
    BookEntryDef {
        species: &species::MOLOSSUS_MOLOSSUS,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Abundant in buildings and urban areas. Alternating QCF at ~34.5/39.6 kHz. One of the first species heard at dusk. Open-space aerial hawker."),
        name: None,
    },
    BookEntryDef {
        species: &species::PTERONOTUS_MESOAMERICANUS,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Long CF at ~61 kHz with Doppler compensation\u{2014}the ONLY high-duty-cycle echolocator in the New World. Unmistakable call. Huge cave colonies."),
        name: None,
    },
    BookEntryDef {
        species: &species::CAROLLIA_PERSPICILLATA,
        commonness: Some(Commonness::VeryCommon),
        description: Some("One of the most abundant Neotropical bats. Peak ~71 kHz. Low-intensity whispering calls\u{2014}detectable only within a few meters. Key seed disperser."),
        name: None,
    },
    BookEntryDef {
        species: &species::ARTIBEUS_JAMAICENSIS,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Very common frugivore. Peak ~56 kHz. Variable intensity; not always a quiet whisperer. Important fig seed disperser throughout lowland forests."),
        name: None,
    },
    BookEntryDef {
        species: &species::TADARIDA_BRASILIENSIS,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Extremely flexible acoustics: QCF 49\u{2013}70 kHz in open space, drops to 25\u{2013}40 kHz near objects. Forms massive colonies. Fast, high-altitude forager."),
        name: None,
    },
    BookEntryDef {
        species: &species::GLOSSOPHAGA_SORICINA,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Abundant nectarivore. Peak ~80 kHz. Low-intensity calls\u{2014}hard to detect beyond a few meters. Uses echolocation to find flowers with acoustic reflectors."),
        name: None,
    },
    BookEntryDef {
        species: &species::DESMODUS_ROTUNDUS,
        commonness: Some(Commonness::VeryCommon),
        description: Some("Common near livestock. Peak ~55 kHz. Relatively long calls for a phyllostomid (~5.5 ms). Low-intensity. Obligate blood-feeder with infrared-sensing nose pits."),
        name: None,
    },

    // ── Common ───────────────────────────────────────────────────

    BookEntryDef {
        species: &species::RHYNCHONYCTERIS_NASO,
        commonness: Some(Commonness::Common),
        description: Some("Tiny bat roosting in lines along riverbanks. CF-FM with peak at ~47 kHz. Drops from ~100 to ~67 kHz during prey pursuit. Cryptic bark-like camouflage."),
        name: None,
    },
    BookEntryDef {
        species: &species::BALANTIOPTERYX_PLICATA,
        commonness: Some(Commonness::Common),
        description: Some("Open-area forager near caves and buildings. Long QCF (14\u{2013}20 ms) at ~43 kHz. Displays jamming avoidance in groups by shifting peak frequency."),
        name: None,
    },
    BookEntryDef {
        species: &species::PEROPTERYX_MACROTIS,
        commonness: Some(Commonness::Common),
        description: Some("Multiharmonic QCF at ~40 kHz (2nd harmonic). Found near caves and rock shelters. Distinctive musky odor."),
        name: None,
    },
    BookEntryDef {
        species: &species::PTERONOTUS_DAVYI,
        commonness: Some(Commonness::Common),
        description: Some("CF-FM at ~67 kHz with sweep to ~51 kHz. Wing membranes fused across back (naked-backed appearance). Cave-roosting; often with P. mesoamericanus."),
        name: None,
    },
    BookEntryDef {
        species: &species::PTERONOTUS_GYMNONOTUS,
        commonness: Some(Commonness::Common),
        description: Some("CF at ~54\u{2013}57 kHz. Largest Pteronotus. Similar to P. davyi but lower frequency. Cave-dwelling."),
        name: None,
    },
    BookEntryDef {
        species: &species::MORMOOPS_MEGALOPHYLLA,
        commonness: Some(Commonness::Common),
        description: Some("Bizarre leaf-chin face. Fundamental suppressed; 2nd harmonic at ~67 kHz dominates recordings. Large cave colonies. Ghost-like appearance in flight."),
        name: None,
    },
    BookEntryDef {
        species: &species::NOCTILIO_LEPORINUS,
        commonness: Some(Commonness::Common),
        description: Some("Large fishing bat. Long CF at 53\u{2013}56 kHz + FM sweep. Rakes water with large clawed feet to catch fish. Found along rivers, lakes, and coasts."),
        name: None,
    },
    BookEntryDef {
        species: &species::MOLOSSUS_SINALOAE,
        commonness: Some(Commonness::Common),
        description: Some("QCF at ~34 kHz. Shifts frequency up ~6 kHz in urban noise (Lombard effect). Larger than M. molossus. Open-space forager."),
        name: None,
    },
    BookEntryDef {
        species: &species::MOLOSSUS_RUFUS,
        commonness: Some(Commonness::Common),
        description: Some("Large molossid with low-frequency QCF at ~25\u{2013}26 kHz. Roosts in buildings and hollow trees. Fast, direct flight."),
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_NIGRICANS,
        commonness: Some(Commonness::Common),
        description: Some("Highly plastic calls: narrowband ~7 ms in open space; broadband FM in clutter. Peak ~50 kHz. Common in forests and urban edges."),
        name: None,
    },
    BookEntryDef {
        species: &species::ARTIBEUS_LITURATUS,
        commonness: Some(Commonness::Common),
        description: Some("Large frugivore. Lower peak (~52 kHz) than A. jamaicensis. Low-intensity. Prominent facial stripes. Important pollinator and seed disperser."),
        name: None,
    },
    BookEntryDef {
        species: &species::STURNIRA_LILIUM,
        commonness: Some(Commonness::Common),
        description: Some("Frugivore with well-documented peak at ~66.5 kHz. Low-intensity FM. Yellow shoulder epaulettes in males. Common in forest and edge habitats."),
        name: None,
    },
    BookEntryDef {
        species: &species::URODERMA_BILOBATUM,
        commonness: Some(Commonness::Common),
        description: Some("Tent-roosting frugivore. Bites leaf ribs to create tent roosts. Peak ~70 kHz. Low-intensity nasal FM\u{2014}hard to detect. Lowland forests."),
        name: None,
    },
    BookEntryDef {
        species: &species::CAROLLIA_CASTANEA,
        commonness: Some(Commonness::Common),
        description: Some("Higher peak (~78 kHz) than C. perspicillata. Low-intensity FM. Frugivore preferring understory fruits. Common in wet lowland forests."),
        name: None,
    },
    BookEntryDef {
        species: &species::CAROLLIA_BREVICAUDA,
        commonness: Some(Commonness::Common),
        description: Some("Intermediate peak (~73 kHz) between C. perspicillata and C. castanea. Low-intensity FM. Frugivore. Premontane and montane forests."),
        name: None,
    },
    BookEntryDef {
        species: &species::GLOSSOPHAGA_COMMISSARISI,
        commonness: Some(Commonness::Common),
        description: Some("Nectarivore. Slightly lower peak (~75 kHz) than G. soricina. Low-intensity FM. Important pollinator of many tropical plants."),
        name: None,
    },
    BookEntryDef {
        species: &species::TRACHOPS_CIRRHOSUS,
        commonness: Some(Commonness::Common),
        description: Some("Famous frog-eating bat. Locates prey by listening to mating calls. Peak ~70 kHz. Low-intensity FM\u{2014}hard to detect. Warty lips for gripping frogs."),
        name: None,
    },
    BookEntryDef {
        species: &species::PHYLLOSTOMUS_HASTATUS,
        commonness: Some(Commonness::Common),
        description: Some("Large omnivore. One of the lowest-frequency phyllostomids (~47 kHz peak). Low-intensity FM. Harem groups in caves and hollow trees."),
        name: None,
    },
    BookEntryDef {
        species: &species::DERMANURA_PHAEOTIS,
        commonness: Some(Commonness::Common),
        description: Some("Small frugivore. Peak ~75 kHz. Low-intensity FM\u{2014}detectable only within a few meters. Common in lowland and premontane forests."),
        name: None,
    },
    BookEntryDef {
        species: &species::MICRONYCTERIS_MICROTIS,
        commonness: Some(Commonness::Common),
        description: Some("Gleaning insectivore. Very short broadband FM (0.3\u{2013}1 ms) at ~90\u{2013}100 kHz. Ultra-low intensity\u{2014}barely detectable beyond 2\u{2013}3 m. Can find motionless prey."),
        name: None,
    },
    BookEntryDef {
        species: &species::EPTESICUS_BRASILIENSIS,
        commonness: Some(Commonness::Common),
        description: Some("Peak ~54\u{2013}60 kHz. Source level ~101\u{2013}106 dB SPL. FM-QCF. Frequency varies with temperature. Forest edges and open areas."),
        name: None,
    },

    // ── Uncommon ─────────────────────────────────────────────────

    BookEntryDef {
        species: &species::SACCOPTERYX_LEPTURA,
        commonness: Some(Commonness::Uncommon),
        description: Some("Higher frequency (~50 kHz) than S. bilineata. Similar QCF structure. Thinner dorsal stripes. Less common; found in lowland forests."),
        name: None,
    },
    BookEntryDef {
        species: &species::CORMURA_BREVIROSTRIS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Unusual: most energy in 5th harmonic at ~68 kHz. Forest-interior forager. Multiharmonic calls."),
        name: None,
    },
    BookEntryDef {
        species: &species::PEROPTERYX_KAPPLERI,
        commonness: Some(Commonness::Uncommon),
        description: Some("Lower frequency (~32 kHz) than P. macrotis. 2nd harmonic dominant. Near caves and rocky outcrops in forested areas."),
        name: None,
    },
    BookEntryDef {
        species: &species::PTERONOTUS_PERSONATUS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Highest frequency Pteronotus: initial CF ~83 kHz, terminal ~68 kHz. Doppler-shift compensation. Cave-roosting."),
        name: None,
    },
    BookEntryDef {
        species: &species::NOCTILIO_ALBIVENTRIS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Higher CF (~75 kHz) than N. leporinus. Trawls insects and small fish from water. Less common than greater bulldog bat."),
        name: None,
    },
    BookEntryDef {
        species: &species::MOLOSSUS_BONDAE,
        commonness: Some(Commonness::Uncommon),
        description: Some("QCF at ~33 kHz. Open-space forager. Roosts in buildings. Slightly lower frequency than M. molossus."),
        name: None,
    },
    BookEntryDef {
        species: &species::MOLOSSUS_COIBENSIS,
        commonness: Some(Commonness::Uncommon),
        description: Some("QCF at ~35 kHz. Originally described from Coiba Island, Panama. Open-space forager. Smaller than other Molossus species."),
        name: None,
    },
    BookEntryDef {
        species: &species::EUMOPS_AURIPENDULUS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Large molossid. Alternating QCF at ~23\u{2013}26 kHz. High, fast flight above canopy. Long-duration narrowband calls."),
        name: None,
    },
    BookEntryDef {
        species: &species::EUMOPS_GLAUCINUS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Very low frequency (~22\u{2013}25 kHz) QCF. Large bat with long narrow wings. High-altitude forager above canopy."),
        name: None,
    },
    BookEntryDef {
        species: &species::CYNOMOPS_GREENHALLI,
        commonness: Some(Commonness::Uncommon),
        description: Some("Low frequency (~22 kHz) open-space forager. Flat face with forward-pointing nostrils. Roosts in buildings and hollow trees."),
        name: None,
    },
    BookEntryDef {
        species: &species::PROMOPS_CENTRALIS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Distinctive upward-modulated QCF (unusual for molossids). Alternating pairs at ~30/35 kHz. Easily recognized on bat detector."),
        name: None,
    },
    BookEntryDef {
        species: &species::NYCTINOMOPS_LATICAUDATUS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Three-frequency alternation (~26.7, 28.7, 32.4 kHz). Open-space forager. Roosts in rock crevices and buildings."),
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_KEAYSI,
        commonness: Some(Commonness::Uncommon),
        description: Some("High repetition rates (15\u{2013}20/s). Short FM calls (~2.5 ms). Peak ~55 kHz. Found in highlands and cloud forests."),
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_RIPARIUS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Steep broadband FM sweep from ~120 to ~50 kHz. Short calls (~2 ms). Forages near streams and forest edges. Recorded in Costa Rica."),
        name: None,
    },
    BookEntryDef {
        species: &species::MYOTIS_ELEGANS,
        commonness: Some(Commonness::Uncommon),
        description: Some("High-frequency FM (~55 kHz peak). Difficult to distinguish from M. nigricans acoustically. Small Myotis of lowland forests."),
        name: None,
    },
    BookEntryDef {
        species: &species::EPTESICUS_FURINALIS,
        commonness: Some(Commonness::Uncommon),
        description: Some("Lower frequency (~43 kHz) than E. brasiliensis. FM-QCF. More FM in cluttered habitats. Forest edges."),
        name: None,
    },
    BookEntryDef {
        species: &species::LASIURUS_BLOSSEVILLII,
        commonness: Some(Commonness::Uncommon),
        description: Some("Open-air forager. Peak ~42 kHz. FM-QCF. Migratory. Roosts solitarily in foliage. Distinctive reddish fur."),
        name: None,
    },
    BookEntryDef {
        species: &species::LASIURUS_EGA,
        commonness: Some(Commonness::Uncommon),
        description: Some("Lower peak (~35 kHz) than L. blossevillii. FM-QCF. Roosts in palm fronds. Open-air forager around street lights."),
        name: None,
    },
    BookEntryDef {
        species: &species::RHOGEESSA_TUMIDA,
        commonness: Some(Commonness::Uncommon),
        description: Some("Small vespertilionid. Broadband FM + QCF termination at ~48 kHz. Forages low in forest gaps and edges."),
        name: None,
    },
    BookEntryDef {
        species: &species::LONCHORHINA_AURITA,
        commonness: Some(Commonness::Uncommon),
        description: Some("UNIQUE phyllostomid with CF-FM calls. Long CF at ~45 kHz (3rd harmonic). Longest phyllostomid calls (up to 8.7 ms). Extremely long nose-leaf. Cave-roosting."),
        name: None,
    },
    BookEntryDef {
        species: &species::PHYLLOSTOMUS_DISCOLOR,
        commonness: Some(Commonness::Uncommon),
        description: Some("Omnivore. Peak ~55 kHz. Low-intensity FM. Large colonies in hollow trees. Best hearing at 20 kHz. Low-frequency for a phyllostomid."),
        name: None,
    },
    BookEntryDef {
        species: &species::LOPHOSTOMA_SILVICOLUM,
        commonness: Some(Commonness::Uncommon),
        description: Some("Gleaning insectivore that modifies termite nests into roosts. Peak ~70 kHz. Low-intensity FM\u{2014}detectable only within a few meters."),
        name: None,
    },
    BookEntryDef {
        species: &species::ANOURA_GEOFFROYI,
        commonness: Some(Commonness::Uncommon),
        description: Some("High-altitude nectarivore. Peak ~70 kHz. Low-intensity FM. Cloud forests and highlands. Important pollinator."),
        name: None,
    },
    BookEntryDef {
        species: &species::CENTURIO_SENEX,
        commonness: Some(Commonness::Uncommon),
        description: Some("Bizarre wrinkled face with retractable skin mask. Peak ~65 kHz. Relatively long calls for a stenodermatine (1\u{2013}3 ms). Frugivore. Low-intensity."),
        name: None,
    },

    // ── Rare ─────────────────────────────────────────────────────

    BookEntryDef {
        species: &species::DICLIDURUS_ALBUS,
        commonness: Some(Commonness::Rare),
        description: Some("Distinctive white fur. Narrowband QCF at ~24 kHz. Rarely encountered. High-altitude open-space forager. One of the most striking-looking bats."),
        name: None,
    },
    BookEntryDef {
        species: &species::EPTESICUS_FUSCUS,
        commonness: Some(Commonness::Rare),
        description: Some("Large vespertilionid. Peak ~30 kHz. FM-QCF. At southern edge of range in Costa Rica. Uncommon in highlands."),
        name: None,
    },
    BookEntryDef {
        species: &species::VAMPYRUM_SPECTRUM,
        commonness: Some(Commonness::Rare),
        description: Some("Largest bat in the Americas (wingspan ~1 m). Peak ~70 kHz. Low-intensity FM\u{2014}very difficult to detect acoustically. Carnivorous: preys on birds and other bats."),
        name: None,
    },
    BookEntryDef {
        species: &species::CHROTOPTERUS_AURITUS,
        commonness: Some(Commonness::Rare),
        description: Some("Carnivorous gleaner. Peak ~77 kHz. Short FM (0.8\u{2013}1.4 ms). Low-intensity\u{2014}hard to detect beyond a few meters. Large ears; hunts other bats and rodents."),
        name: None,
    },
    BookEntryDef {
        species: &species::MACROPHYLLUM_MACROPHYLLUM,
        commonness: Some(Commonness::Rare),
        description: Some("Unusual trawling phyllostomid. Louder than most relatives (~101 dB SPL). Peak ~85 kHz. Large feet for grabbing insects from water surfaces."),
        name: None,
    },
    BookEntryDef {
        species: &species::MICRONYCTERIS_HIRSUTA,
        commonness: Some(Commonness::Rare),
        description: Some("Gleaning insectivore. Lower peak (~52 kHz) than M. microtis. Low-intensity FM. Documented from Costa Rica. Forest interior."),
        name: None,
    },
    BookEntryDef {
        species: &species::MIMON_CRENULATUM,
        commonness: Some(Commonness::Rare),
        description: Some("Gleaning insectivore. Peak ~75 kHz. Low-intensity FM. Now Gardnerycteris crenulatum. Forest understory."),
        name: None,
    },
    BookEntryDef {
        species: &species::TONATIA_SAUROPHILA,
        commonness: Some(Commonness::Rare),
        description: Some("Gleaning insectivore/carnivore. Peak ~65 kHz. Low-intensity FM. Forest interior specialist. Roosts in hollow trees."),
        name: None,
    },
    BookEntryDef {
        species: &species::LAMPRONYCTERIS_BRACHYOTIS,
        commonness: Some(Commonness::Rare),
        description: Some("Rare gleaning insectivore. Peak ~75 kHz. Low-intensity FM. Poorly documented acoustically. Yellow throat patches."),
        name: None,
    },
    BookEntryDef {
        species: &species::GLYPHONYCTERIS_SYLVESTRIS,
        commonness: Some(Commonness::Rare),
        description: Some("Rare gleaner. Peak ~85 kHz. Ultra-short broadband FM (0.3\u{2013}1 ms). Very low intensity. Forest interior."),
        name: None,
    },
    BookEntryDef {
        species: &species::TRINYCTERIS_NICEFORI,
        commonness: Some(Commonness::Rare),
        description: Some("Low-intensity gleaner. Peak ~80 kHz. Multiharmonic FM. Forest understory specialist. Rarely captured or detected."),
        name: None,
    },
    BookEntryDef {
        species: &species::HYLONYCTERIS_UNDERWOODI,
        commonness: Some(Commonness::Rare),
        description: Some("Very small nectarivore. High frequency peak ~90 kHz. Low-intensity FM. Montane cloud forests. Poorly known acoustically."),
        name: None,
    },
    BookEntryDef {
        species: &species::MESOPHYLLA_MACCONNELLI,
        commonness: Some(Commonness::Rare),
        description: Some("Tiny (5\u{2013}7 g) with the highest peak frequency of any phyllostomid (~100\u{2013}118 kHz). Ultra-low intensity. Tent-roosting frugivore."),
        name: None,
    },
    BookEntryDef {
        species: &species::ECTOPHYLLA_ALBA,
        commonness: Some(Commonness::Rare),
        description: Some("Iconic tiny white bat. Peak ~75 kHz. Low-intensity FM. Roosts in Heliconia leaf tents. Endemic to Central America. Specializes on one fig species."),
        name: None,
    },
    BookEntryDef {
        species: &species::THYROPTERA_TRICOLOR,
        commonness: Some(Commonness::Rare),
        description: Some("Suction-cup disks for roosting in rolled Heliconia leaves. Extremely low intensity\u{2014}barely detectable at <1 m. Distinctive social calls for roost-finding."),
        name: None,
    },
    BookEntryDef {
        species: &species::NATALUS_MEXICANUS,
        commonness: Some(Commonness::Rare),
        description: Some("Among the highest frequency bats: peak ~100\u{2013}130 kHz. Very low intensity\u{2014}barely detectable beyond 50 cm. Delicate, cave-roosting. Formerly N. stramineus."),
        name: None,
    },
    BookEntryDef {
        species: &species::BAUERUS_DUBIAQUERCUS,
        commonness: Some(Commonness::Rare),
        description: Some("Very quiet calls (~35 kHz peak). Plecotus-like gleaning insectivore. Rare and poorly known. Montane forests."),
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
        BatBookRegion::CostaRica => COSTA_RICA_BOOK,
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
