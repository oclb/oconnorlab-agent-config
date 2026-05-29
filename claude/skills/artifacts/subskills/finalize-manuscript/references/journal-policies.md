# Journal Data & Code Availability Policies

## AJHG (American Journal of Human Genetics)

### Required Section

"Data and Code Availability" — located within STAR Methods > Resource Availability. The Resource Availability section has three mandatory subsections:
1. **Lead Contact** — Name and email of corresponding author
2. **Materials Availability** — Statement on newly generated materials (required even if none)
3. **Data and Code Availability** — Three bullet points (all three required):

### Data and Code Availability: Three Bullets

**Bullet 1 — Datasets:**
- Describe where data are deposited
- All standardized data types must be in a Cell Press-recommended, type-specific repository before acceptance
- Accession numbers or permanent identifiers must be listed
- Any embargo must lift by publication date
- Recommended repos: SRA/GEO (sequencing), dbGaP/EGA (controlled-access human), ClinVar/dbSNP/dbVar (variation), Zenodo/Figshare/Dryad (general)

**Bullet 2 — Code:**
- All original code must be deposited in a DOI-minting repository (e.g., Zenodo) or included in supplemental info before acceptance
- DOI must be reported
- Novel programs must be publicly available by publication; URL listed in Web Resources section

**Bullet 3 — Additional information:**
- Standard text: "Any additional information required to reanalyze the data reported in this paper is available from the lead contact upon request."

### Additional AJHG Requirements
- **Web Resources section** (after Acknowledgments): List URLs for all web resources, databases, and software
- **Key Resources Table** (in STAR Methods): List all critical reagents, software, datasets with identifiers (accession numbers, RRIDs, DOIs)
- Data must be publicly accessible unless legal/ethical prohibition (editor approval required)

### Example Statement
> - Single-cell RNA-seq data have been deposited at GEO under accession number GSE85337 and are publicly available as of the date of publication. Original western blot images have been deposited at Mendeley Data at [DOI] and are publicly available as of the date of publication.
> - All original code has been deposited at Zenodo at [DOI] and is publicly available as of the date of publication.
> - Any additional information required to reanalyze the data reported in this paper is available from the lead contact upon request.

---

## Nature Genetics

### Required Sections

Two separate sections, both placed after Methods and before References:
1. **Data Availability**
2. **Code Availability**

### Data Availability

Mandatory deposition in community-endorsed repositories for these data types:

| Data Type | Required Repository |
|-----------|-------------------|
| DNA/RNA sequences, assemblies | GenBank, ENA, or DDBJ (INSDC) |
| Gene expression | GEO or ArrayExpress |
| Genetic variation <50bp (human) | dbSNP |
| Genetic variation >50bp (human) | dbVar or EVA |
| Sensitive human genomic data | EGA, dbGaP, or JGA (controlled-access) |
| Protein sequences | UniProt |
| Proteomics | ProteomeXchange member |
| Structures | wwPDB, BMRB, or EMDB |

- For types without a community repo: use Zenodo, Figshare, or Dryad
- Providing large datasets only as supplementary info is strongly discouraged
- Statement must include repository name, accession number, and functional URL
- Accession numbers should also appear in reference list with DataCite format

### Code Availability

- Custom code must be deposited in a DOI-minting repository (Zenodo, Code Ocean)
- **GitHub link alone is not sufficient** — must archive a snapshot with a DOI (GitHub integrates with Zenodo for this)
- DOI should be cited in the reference list
- Open-source license (OSI-approved) encouraged
- When code is central to the manuscript, it must be shared with editors/reviewers during peer review

### Example Statement
> Custom analysis scripts are available at [GitHub URL] and have been archived at Zenodo ([DOI]). [Software name] (v1.2) is available at [URL] under an MIT license.

---

## Comparison

| Aspect | AJHG | Nature Genetics |
|--------|------|-----------------|
| Section name | "Data and Code Availability" (combined) | Separate "Data Availability" and "Code Availability" |
| Placement | Within STAR Methods | After Methods, before References |
| Format | Three mandatory bullet points | Prose paragraphs |
| Code deposition | DOI-minting repo or supplemental info | DOI-minting repo (GitHub alone insufficient) |
| Additional sections | Web Resources; Key Resources Table | Code cited in reference list |
| Code peer review | Not explicitly required | Required when code is central |
