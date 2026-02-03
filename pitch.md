# PeakUI – Universell grensesnitt-infrastruktur for autonome systemer

**Prosjektsammendrag for Innovasjon Norge**  
**Innsending: EU Konkurranseevnefond – Digital satsing**  
**Dato: Februar 2026**

---

## Sammendrag

PeakUI er et norskutviklet,open-source UI-rammeverk som transformerer hvordan programvare kommuniserer med kunstig intelligens. Bygget i Rust tilbyr det et **universelt grensesnittlag** som distribuerer samme kildekode til alle større plattformer (Linux, macOS, WASM, kommende mobil), samtidig som det eksponerer et maskinlesbart **Semantisk Tre** for AI-agenter.

I motsetning til tradisjonelle løsninger som krever bildebehandling, leverer PeakUI strukturert data direkte fra UI-laget til AI-en. Dette reduserer energiforbruket med **opptil 99 %** sammenlignet med pikselbasert analysemetoder – en revolusjon for grønn AI.

---

## Problemstilling

### Status Quo: AI møter digital friksjon
Dagens autonome systemer navigerer programvare gjennom visuell prosessering (skjermbilder + OCR), noe som er:
- **Energikrevende**: Bildebehandling krever GPU-beregning ved hver interaksjon
- **Upresist**: OCR og objektdeteksjon feiler på komplekse grensesnitt
- **Plattformspesifikt**: Hver implementasjon må omskrives for web, desktop og mobile

### Konsekvensen
Autonome agenter (roboter, AI-assistenter, industrielle kontrollsystemer) kan ikke effektivt samhandle med moderne programvare uten høyt ressursforbruk.

---

## Løsningen: PeakUI

### 1. Semantisk Serialisering (Kjerneinnovasjonen)
Hvert PeakUI-komponent (knapper, lister, formularer) implementerer en `describe()`-metode som genererer et **SemanticNode**:

```rust
pub struct SemanticNode {
    pub role: String,              // "button", "text_field", "navigation"
    pub label: Option<String>,     // Komponenten sitt formål
    pub content: Option<String>,   // Tekstverdi, hvis relevant
    pub children: Vec<SemanticNode>,
    pub neural_tag: Option<String>, // Unik ID for AI-aksjon
    pub depth: Option<f32>,        // 3D-hierarki (for spatial computing)
    pub is_protected: bool,        // "Neural Sudo" sikkerhetsflagg
    pub protection_reason: Option<String>,
}
```

Dette skaper en **parallell datastruktur** optimalisert for språkmodeller (LLM), der AI-en kan "se" programvarens tilstand uten å analysere piksler.

### 2. Action Bridge Protocol
Rammeverket tilbyr et **deterministisk API** for AI-utløste handlinger:
- **Navigate(Page)**: Bytt aktiv visning
- **SetTheme(Dark)**: Endre designsystem i sanntid
- **SetValue(field_id, value)**: Fyll inn skjema
- **ExecuteCommand(name)**: Trigge applikasjonsoperasjoner

AI-en kan kombinere **Semantisk Tre** (øyne) med **Action Bridge** (hender) for å utføre multi-steg-oppgaver som: *"Åpne innstillinger, slå på mørk modus, og sett skriftstørrelse til 16pt"*.

### 3. Neural Sudo – Sikker AI-utførelse
Kritiske handlinger (sletting, systemkommandoer, betalinger) markeres med `.sudo(reason)`:

```rust
Button::new("Slett database")
    .sudo("Destruktiv handling – krever brukerbekreftelse")
```

Dette sikrer at autonome agenter aldri kan utføre irreversible handlinger uten menneskelig godkjenning.

### 4. Universell Plattformdistribusjon
PeakUI separerer applikasjonslogikk fra rendering gjennom en **Backend Trait**:
- **IcedBackend**: Native desktop (Metal på macOS, Vulkan på Linux)
- **WASM**: Høyperfomanse web-applikasjoner
- **SpatialBackend**: 3D-koordinater for VR/AR (eksperimentell)
- **AIBackend**: Semantisk tre for språkmodeller

Samme Rust-kode kompilerer til alle disse målene uten portingsinnsats.

---

## Teknisk Gjennombrudd

### Grønn Beregning ("Green Computing")
- **Tradisjonell AI-interaksjon**: 
  - Skjermbilde (1920×1080 RGB) = 6.2 MB per frame
  - GPU-prosessering for objektdeteksjon: ~50W kontinuerlig
  
- **PeakUI Semantisk Serialisering**:
  - JSON-tre (typisk størrelse: 5-15 KB)
  - CPU-prosessering: ~0.5W sporadisk
  - **Resultat: 99 % reduksjon i energibehov**

### Minnesikkerhet (Rust)
Rust garanterer 100 % minnesikkerhet uten "garbage collector", noe som eliminerer en hel klasse sikkerhetssårbarheter. Dette er kritisk for industrielle systemer der pålitelighet er livsavgjørende.

### Volumetrisk Layout (Spatial Computing)
PeakUI inkluderer en 3D-koordinatsystem der hvert komponent har:
- **Posisjon (x, y, z)**
- **Dybde** (hierarkisk sortering)
- **Ray-casting** for spatial input (VR-kontrollere, gaze-tracking)

Dette gjør rammeverket framtidsklart for «romlig databehandling» (spatial computing) – en nøkkelpillar i Apples Vision Pro og kommende AR-briller.

---

## Status og Modenhet

### Produksjonsklare komponenter ✅
- **Showcase-applikasjon**: Fungerende demo med 20+ komponenter
- **Semantisk serialisering**: Implementert i alle widgets
- **WASM-bygg**: Verifisert i produksjon
- **Action Bridge Protocol**: Aktivt i bruk i PeakOS-prosjektet
- **Neural Sudo**: Sikkerhetslag funksjonelt

### Pågående arbeid 🚧
- **Mobil engine**: Touch-optimalisering for native iOS/Android
- **Terminal backend (TUI)**: Teoretisk mulig, ikke komplett implementasjon
- **VR/AR**: Spatial backend eksperimentell

### Veikartet (Roadmap)
- **Q2 2026**: Ferdigstill mobil touch-infrastruktur
- **Q3 2026**: P2P-synkronisering mellom enheter (PeakCloud Mesh)
- **Q4 2026**: On-device LLM-integrasjon for offline AI

---

## Industriell Anvendelse

### 1. Energisektoren – Offshore kontrollrom
**Scenarie**: Autonome overvåkningssystemer som tolker kritiske dashboards.  
**Fordel**: Semantisk serialisering tillater AI-agenter å reagere på alarmer uten visuell prosessering, reduserer latens fra sekunder til millisekunder.

### 2. Forsvarssektorer – Taktiske systemer
**Scenarie**: Stridsvogn-operatørsystemer der AI assisterer besetningen.  
**Fordel**: Neural Sudo sikrer at AI aldri kan trigge våpensystemer uten menneskelig godkjenning. Rust's minnesikkerhet eliminerer sårbarheter i systemkritiske miljøer.

### 3. Robotikk – Industriell automatisering
**Scenarie**: Fabrikkroboter som bruker eksisterende kontrolltavler.  
**Fordel**: I stedet for å montere kameraer for å "lese" skjermer, kan roboter integrere direkte via Action Bridge Protocol.

### 4. Helsevesenet – Assistert diagnostikk
**Scenarie**: AI-systemer som bruker medisinske journalsystemer.  
**Fordel**: Semantisk tilgang til pasient-UI reduserer feilmarginer sammenlignet med OCR av skannet tekst.

---

## Hvorfor Norge bør satse på PeakUI

### 1. Strategisk teknologisk posisjonering
- **Open-source lisens**: Unngå leverandørinnlåsing, tilgjengelig for norsk industri
- **Rust-økosystem**: Norge har sterke tekniske miljøer, spesielt innen sikkerhetskritisk software
- **AI-beredskap**: Posisjonerer norske bedrifter for den autonome revolusjonen

### 2. Grønn konkurransekraft
EUs Konkurranseevnefond prioriterer digital bærekraft. PeakUI tilbyr dokumenterbar energireduksjon, noe som kan bli et krav i fremtidige offentlige anbud.

### 3. Eksportpotensial
Globale aktører (Tesla, Siemens, ABB) leter etter AI-klare grensesnittløsninger. En norskledet standard kan sikre teknologisk suverenitet i et marked dominert av amerikanske og kinesiske aktører.

## Konklusjon

PeakUI representerer et paradigmeskifte i hvordan programvare og kunstig intelligens kommuniserer. Ved å erstatte energikrevende bildebehandling med **semantisk serialisering**, tilbyr rammeverket:

1. **99 % reduksjon i AI-energiforbruk** (grønn digitalisering)
2. **Universell plattformkompatibilitet** (WASM, native, kommende mobil/spatial)
3. **Industriell sikkerhet** (Rust + Neural Sudo)
4. **Norsk teknologisk suverenitet** i AI-infrastruktur

---

## Kontaktinformasjon

**Vegar Berentsen**  
PeakSuite / PeakUI  
E-post: [Vegarberentsen@gmail.com]  
Nettside: [peakui.vercel.app]
Github: [https://github.com/Designrpros/PeakUI]    
Kode: Open-source (BSL 1.1)

**For teknisk dokumentasjon**:  
Se `README.md` og `/docs` i repositoriet for detaljert arkitektur og eksempelkode.

---

## Vedlegg

### A. Kodeeksempel – Semantisk serialisering i praksis

```rust
// En enkel knapp som eksponerer sin tilstand til AI
.push(
    Button::new("Send betaling")
        .sudo("Finansiell transaksjonsrisiko")
        .on_press(Message::ProcessPayment)
)
```

AI-en vil motta:
```json
{
  "role": "button",
  "label": "Send betaling",
  "neural_tag": "payment_button",
  "is_protected": true,
  "protection_reason": "Finansiell transaksjonsrisiko"
}
```


---

**Merknad til vurderingskomité**:  
PeakUI er et aktivt prosjekt i produksjon for PeakSuite og PeakOS (et komplett autonomt operativsystem). Rammeverket er ikke «vaporware» – det kan demonstreres live i WASM på få minutter. Vi oppfordrer teknisk evaluering av kodebasen før finansieringsbeslutning.
