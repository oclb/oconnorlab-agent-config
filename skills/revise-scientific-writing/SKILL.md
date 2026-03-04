---
name: revise-scientific-writing
description: Scientific writing revision using O'Connor lab principles. Invoke manually with /revise-scientific-writing when you want a manuscript, abstract, or section revised for clarity, structure, and persuasiveness. User-invoked only - never auto-trigger.
disable-model-invocation: true
version: 1.0.0
---

# Revise Scientific Writing Skill

This skill applies the O'Connor lab scientific writing principles to revise scientific papers and documents for clarity, structure, and persuasiveness.

## When This Skill Applies

Use this skill when the user wants to:
- Revise a scientific paper or manuscript
- Improve scientific writing clarity or structure
- Prepare a conference abstract

## Core Writing Philosophy

**Write for your readers.** Your paper should convince reviewers, satisfy aficionados, and engage casual readers.

## Revision Process

Follow this systematic approach:

### Step 1: Identify the Document Type

Determine what you're revising:
- **Full manuscript** (all sections)
- **Specific section** (Title, Abstract, Introduction, Results, Discussion, Methods)
- **Display items** (figures, tables, captions)
- **Conference abstract**
- **Supplement**

### Step 2: Apply General Principles

Before section-specific revision, check these universal principles:

#### Point-First Style

**Principle**: Don't save up for a big reveal; put the point first.

**Check for:**
- Does the reader understand WHY they're reading something BEFORE the details?
- Does each paragraph lead with its main point?
- Other acceptable ways to order content, especially sentences: by logical precedence (A implies B); by temporal precedence (we did A, then we did B); by motivation (A motivates B).

**Bad**: "We performed simulation with 1000 replicates under a neutral model. Effect size variance was 0.05. We found..."

**Good**: "We quantified the unbiasedness of our estimator in neutral simulations. We performed 1000 replicates with an effect size variance of 0.05. We found..."

#### Judicious Use of Emphasis

**Principle**: Emphasize important points; don't emphasize secondary, obvious, or self-serving content.

**Methods of emphasis:**
- **Position**: Put it first (sentence, paragraph, section)
- **Repetition**: Repeat key concepts
- **Naming**: Give concepts memorable names
- **Titles**: Section headers, figure titles
- **Punctuation**: Slows reader down (colons, semicolons, dashes)
- **Sentence structure**: Short or unusual sentence structures give emphasis

**Check for over-emphasis:**
- An entire paragraph devoted to a secondary point or caveat
- Repeating a point that has already been made and is either obvious, secondary, or dubious
- "We performed a sophisticated analysis..."
- Emphatic punctuation, like em-dashes or question marks, in more than a few places

**Check for under-emphasis:**
- Burying the main finding in the middle of a long paragraph
- Not naming a key concept that's referenced repeatedly
- The paper's three most important findings are all shoehorned into one of its three Results subsections

#### Cohesiveness

**Principle**: Sentences and paragraphs should connect logically.

**Sentence-level cohesiveness:**
- **Name previous concepts**: "convergence", "LD-dependant bias"
- **Use linking words**: "However," "Therefore," "In contrast," "Similarly"
- **Use pronouns (with clear antecedent)**: "this approach," "these results"

**Example**:
- "Genome-wide association studies (GWAS) have identified thousands of genetic associations. Common variants have small effect sizes individually, but they combine to explain a large fraction of common disease heritability"
- "Genome-wide association studies (GWAS) have identified thousands of common variants that are associated with common diseases and traits. Common variants have small effect sizes individually, but they combine to explain a large fraction of common disease heritability" (Repeated keyword: "common variants")

**Paragraph-level cohesiveness:**
- First sentence of paragraph should connect to the previous paragraph's point
- Paragraphs should follow a logical progression
- Transitional sentences link ideas
- In example above, the following paragraph begins, "To characterize common-variant architecture, ..."

**Check**:
- Can you follow the logical thread?
- Are there jarring topic changes?
- Does each paragraph justify its position in the sequence?

#### Connection to Literature

**Principle**: Embed your paper within the reader's existing mental model.

**Check for:**
- Are prior methods/findings cited throughout Results (not just Introduction)?
- Do you cite related literature in a manner that not only acknowledges but explains the connection?

**Good practice**: "In sequencing studies, most rare variants are observed in only one or a few individuals, motivating the use of burden tests that aggregate minor alleles within genes (citation). We defined burden heritability as the fraction of phenotypic variance explained by minor allele burden in each gene" (Connects to prior literature and clearly states the novel contribution)

### Step 3: Section-Specific Revision

Apply targeted revisions based on the section:

## TITLE

**Purpose**: Attract every reader interested in your content.

**Three approaches:**
1. **Describe subject matter**: "The common-variant effect size distribution"
2. **Describe methodological advance**: "Partitioning gene-mediated heritability without eQTLs"
3. **Describe key finding**: "Extreme polygenicity of complex traits is explained by negative selection"

**Including key details (optional):**
- Add one keyword or surprising detail to attract specific audiences
- "Extremely sparse models of linkage disequilibrium in **ancestrally diverse** association studies"
- "Partitioning gene-mediated heritability **without eQTLs**" (element of surprise)

**Revision checklist:**
- [ ] Does title attract target audience?
- [ ] Is it specific enough to differentiate from other papers?
- [ ] If includes key detail, does Introduction follow through?
- [ ] Is it concise (typically ≤15 words)?

## ABSTRACT

**Purpose**: Written for a broad audience; self-contained summary.

**Structure**
1. **Broad context** (1 sentence): Key background for the subject matter - something already known to target audience
2. **Specific gap and motivation** (1-2 sentences): More detailed background; what is specifically unknown?
3. **What you did** (1-2 sentences): Your approach (brief)
4. **What you found** (2-4 sentences): One sentence per major result; can be interspersed with (3)
5. **Implications** (1 sentence): Broader perspective

**Revision checklist:**
- [ ] First sentence broad enough for general readers?
- [ ] Gets to the point quickly (no meandering intro)?
- [ ] Methodology clear but not overly detailed?
- [ ] Avoids jargon where possible?
- [ ] Self-contained (understandable without reading paper)?

**Conference abstracts:**
- Can be 2-3 paragraphs (mini Introduction/Methods/Results)
- Still get to point immediately, no wind-up
- More detail than journal abstract
- Remember readers compare back-to-back with dozens of others

## INTRODUCTION

**Purpose**: Situate paper in literature, motivate question, make a promise about what the reader will learn by reading on.

**Standard structure** (short Introduction):

**Paragraph 1 - Problem statement:**
- Broad context
- Quickly get to the point: what is this paper about? ("Funnel shape")
- Often end with gap/question
- It should be fairly clear what the contribution of the paper is based on this paragraph plus the title

**Example**:
> "LD poses a challenge in genome-wide association studies (GWASs)
because disease-causing alleles reside on haplotypes with numerous tag
SNPs. In applications such as heritability partitioning, polygenic risk prediction and fine mapping, LD matrices from a reference panel are used to model summary association statistics from GWASs, especially when individual-level genotypes and phenotypes are unavailable. However, these LD correlation matries can be terabytes in size9, leading to computational bottlenecks."

**Optional Paragraph 2 - Significance:**
- Elaborate on importance of question
- Connect to "key detail" from title if present
- Rationale for approach

**Example (cont'd from above)**:
> "The challenge is exacerbated in ancestrally diverse association
studies. Diversity carries crucial scientific benefits, but it also poses a methodological challenge, because LD patterns vary across ancestry groups. This variation makes it even more important to model LD in applications and it also increases the difficulty of doing so."

**Paragraphs 3-N - Prior literature:**
- Describe existing methods/approaches
- Related or parallel problems
- Can be single paragraph or several depending on literature density

**Example**:
> "To model LD efficiently and accurately, a possible approach is
to leverage the genealogical history that gave rise to LD in the first
place. New mutations arise on haplotypes carrying existing alleles
and become correlated as they increase in frequency1,16. With recent
breakthroughs, genome-wide genealogies of recombining organ-
isms can be inferred from large-scale genetic datasets and recorded
in succinct tree sequences17–20. Capitalizing on the limited number of common ancestral haplotypes at most loci, tree sequences provide a
compact representation of human genetic data19 ,21. Tree sequences,
and the closely related ancestral recombination graph, have enabled
powerful new methods for understanding ancestral relationships22–24
,
measuring selection20,25,26 and analyzing complex traits27 ,28
.

**Optional Paragraph N+1 - Challenge:**
- Why hasn't this been solved?
- Limitations of existing approaches

**Final Paragraph - Roadmap:**
- What you do in the paper
- Summary of methodological advance
- Optionally: brief summary of findings (emphasis on what you DID)

**Revision checklist:**
- [ ] First paragraph gets to point quickly?
- [ ] Clear problem statement (gap/question)?
- [ ] Command of relevant literature demonstrated?
- [ ] Subtle distinctions with prior work made clear?
- [ ] Reader knows whether to keep reading?
- [ ] Reader knows what findings to expect?
- [ ] Roadmap paragraph describes what you DID not just what you found?

**Long Introduction alternative:**
- Can build narrative (e.g., historical evolution of idea)
- 5-7 paragraphs tracing development of central concept
- Example: Boyle et al. 2017 "From polygenic to omnigenic"

## RESULTS

**Purpose**: The meat of the paper. Appeal to broad and close readers.

**High-level organization:**

**Subsection 1 - Study design or methodological innovation:**
- Overview, not detailed protocol
- First paragraph: what are you estimating/studying?
- Second paragraph: how do you estimate it?
- Third paragraph: key mathematical content (equations if needed)
- Fourth paragraph: methodological details (can reference Methods)
- Optional: conclude with 1-2 paragraphs on simulations

**Point-first structure**: Essential content first, details later.

**Optional Subsection 2 - Validation:**
- Simulations, sanity checks, QC benchmarks
- Replication of prior results
- Use for methods-oriented papers or surprising claims

**Subsection 3 - Topline results:**
- **Written for 100% of audience**
- First paragraph: describe data analyzed
- **Get to the point immediately**: What's the answer?
- No buildup or suspense

**Example**:
> "We analyzed 50 complex traits from UK Biobank (N = 500K). Heritability estimates ranged from 0.05 to 0.45 (mean = 0.20), consistent with prior studies."

**Subsections 4-N - Secondary questions:**
- Each addresses one secondary question
- Introduce one new concept, connect to main thrust
- Begin with mini-introduction
- Should NOT merely support previous section
- Should NOT require summary paragraph
- Last paragraph can list secondary analyses

**Optional Final Subsection - Coda:**
- Extends results in unexpected/orthogonal direction
- Different dataset, different field, creative application
- Adds flourish for high-impact papers
- Doesn't need close connection to preceding content

**Subsection titles:**
- Extremely short (single column width)
- Don't need to stand alone
- Useful for skimming readers
- Describe content, not conclusions

**Revision checklist:**
- [ ] 4-7 subsections (typical)?
- [ ] First subsection overviews method/design?
- [ ] Point-first style throughout?
- [ ] Topline results subsection accessible to all readers?
- [ ] Each subsection addresses one clear question?
- [ ] No subsection merely supports another?
- [ ] Subsection titles short and descriptive?
- [ ] Mini-introductions for secondary analyses?

## DISPLAY ITEMS (Figures & Tables)

**Purpose**: Each panel = one finding. Visually self-evident.

**Figure organization:**
- One main display item ≈ one Results subsection
- Each panel contains ONE finding
- Cite all subpanels in order in main text
- Show LESS data rather than more (supplement the rest)

**Visual clarity:**
- Point should be visually apparent
- Subplot titles (1-2 lines) explain what differs between similar plots
- Same x/y scale across similar plots (enables visual comparison)
- Consistent color axis across subplots
- Don't reuse colors for different meanings in same figure

**Production quality** (high-impact journals):
- Professional appearance (Illustrator-level quality)
- Subpanels wider than tall
- Uniform subpanel sizes (2-4 per row)
- Uniform text size and line thickness
- Visually pleasing color scheme
- 300 DPI for publication

**Figure captions:**
- **Title**: Half-sentence describing figure itself (not its punchline)
- **Body**: Short, factual description
- NO narrative ("We computed...")
- Describe axes, what differs among plots
- Plotting details: error bars, significance, abbreviations
- Reference supplementary table with numerical results

**Example title**:
- "Our method outperforms existing approaches"
- "Comparison of heritability estimation methods"

**Revision checklist:**
- [ ] One finding per panel?
- [ ] Visually self-evident?
- [ ] All text carefully chosen?
- [ ] Consistent scales/colors where appropriate?
- [ ] Professional production quality?
- [ ] Caption is factual, not narrative?
- [ ] Caption describes what's plotted, not conclusions?
- [ ] Numerical results available in supplement?

## DISCUSSION

**Purpose**: Say interesting things about your paper. Open-ended exploration.

**Structure:**

**Paragraph 1 - Review and contextualize:**
- Provide new/broader context (different from Introduction)
- Brief review of results (1-2 sentences MAX)
- Set stage for deeper discussion

**Paragraphs 2-N - Elaboration:**
- Implications: what can you now do/understand that you couldn't before?
- Limitations: what are the caveats? (addressed here or in separate paragraph)
- Conceptual underpinnings: what's the deeper meaning?
- Relation to specific prior work (too specific for Introduction)
- Generalization to other data types
- Parallels with other approaches/fields
- Important subtle distinctions
- Future directions

**Each paragraph**: Could prompt good journal club discussion.

**Last/second-to-last paragraph - Limitations** (optional separate paragraph):
- 3-5 limitations, each in 1-2 sentences
- Include mitigation or why limitation isn't too serious
- Can be technical or conceptual
- Alternative: embed limitations across multiple paragraphs

**Revision checklist:**
- [ ] First paragraph provides fresh context?
- [ ] Results summary very brief?
- [ ] Address questions you get when presenting?
- [ ] Make subtle distinctions clear?
- [ ] Acknowledge fundamental limitations?
- [ ] Each paragraph could prompt discussion?
- [ ] Limitations addressed (separate paragraph or throughout)?
- [ ] Not overly speculative?

## METHODS

**Purpose**: Enable replication. Written for reviewers, detail-oriented readers, and replicators.

**Three audiences:**
1. Skeptical/detail-oriented readers
2. Readers wanting deep method understanding
3. Readers wanting to replicate/build upon analyses

**Key principle**: Highly detailed. Knowledgeable reader should be able to replicate exactly.

**Structure:**
- Subsections correspond 1:1 with Results citations, in order
- Each subsection addresses one aspect of methodology

**Common subsections:**
- Data description (cite Data Availability, supplementary table)
- Selection criteria for data
- Mathematical model or quantity being estimated
- Algorithm or estimation procedure
- Hypothesis testing or standard error computation
- Mathematical derivations (short ones; long → Supplement)
- Key equations (auxiliary calculations used throughout)
- Simulation procedures (cite supplementary table with parameters)
- Use of existing methods (including parameter choices)

**Include "why" not just "how":**
- Explain methodological choices
- Head off reviewer questions
- Justify decisions

**Revision checklist:**
- [ ] Subsections match Results citation order?
- [ ] Detailed enough for exact replication?
- [ ] Methodological choices justified?
- [ ] Cites Data Availability and supplementary tables?
- [ ] Existing methods' parameters specified?
- [ ] Long derivations moved to Supplement?

## SUPPLEMENT

**Purpose**: Supporting material. Written for future self and detail-oriented readers.

**Reader in mind**: Your future self re-reading to recall analysis details.

**Supplementary Note:**
- Mathematical justifications (derivations, proofs)
- Can be formal (definitions → theorems → proofs)
- Logical precedence over point-first style (if mathematical)
- Too lengthy for Methods but important

**Supplementary Figures:**
- Secondary analyses
- Cite in order from main text (Results, Discussion, Methods)
- Professional but not main-figure production quality
- **Captions can be long** (unlike main figures)
- Detailed description AND interpretation of analyses
- Support terse sentences in main text
- Can be half-page long (if longer, create supplementary note)

**Extended Data Figures** (Nature family):
- Up to 10 allowed
- Slightly-more-important supplementary figures
- Same principles apply

**Supplementary Tables:**
- All numerical results in manuscript
- Raw data to reproduce main figures (or cite published dataset)
- Short captions (e.g., "Numerical results for Figure 2")
- Describe non-obvious column labels in caption
- Plain text in supplement OR separate Excel file
- Excel: name tabs appropriately, avoid Excel errors (gene→date conversion)
- Upload Excel as "dataset" not PDF (Nature family)

**Revision checklist:**
- [ ] Complete enough for future self?
- [ ] All supplementary items cited in order?
- [ ] Captions support terse main text sentences?
- [ ] Numerical results complete?
- [ ] Excel files error-free and well-organized?
- [ ] Formal mathematical content in supplementary note?

## Revision Workflow

When revising a document:

### Step 1: Read and Understand
- Read entire document first
- Identify what type of document (paper, section, abstract)
- Note initial impressions

### Step 2: Apply General Principles
Check each paragraph for:
- Point-first structure
- Appropriate emphasis
- Cohesiveness with surrounding content
- Connections to literature

### Step 3: Section-by-Section Review
For each section present:
- Apply section-specific checklist
- Provide concrete suggestions
- Preserve scientific content
- Maintain author's voice

### Step 4: Provide Structured Feedback

**Format**:
```
SECTION: [Name]

STRENGTHS:
- [What works well]

SUGGESTIONS:
1. [Specific issue]
   Current: "[Quote problematic text]"
   Revised: "[Proposed revision]"
   Reason: [Why this improves it]

2. [Next issue]
   ...

CHECKLIST:
- [ ] Point-first style applied
- [ ] Emphasis appropriate
- [ ] Cohesive structure
- [X] Section-specific criterion met
...
```

### Step 5: Prioritize Feedback

**High priority** (fix these first):
- Structural issues (wrong organization)
- Missing point-first style in key locations
- Over/under-emphasis of important content
- Lack of cohesiveness

**Medium priority**:
- Wordiness or awkward phrasing
- Missing connections to literature
- Incomplete section components

**Low priority** (polish):
- Minor word choices
- Stylistic preferences
- Format details

## Special Cases

### Revising an Abstract for High-Impact Journal
- Very broad opening sentence
- Get to point in 2 sentences max
- One sentence per major result
- Broader perspective paragraph
- No references

### Revising a Conference Abstract
- Can be 2-3 paragraphs
- More detail than journal abstract
- Mini Introduction/Methods/Results
- Get to point immediately (no broad intro)
- Consider it's read back-to-back with dozens of others

### Revising for Specific Audiences

**For reviewers** (suspicious readers):
- Justify all choices
- Address obvious questions preemptively
- Include validation analyses
- Detailed Methods

**For aficionados** (domain experts):
- Subtle distinctions with prior work
- Deep connection to literature
- Technical details in supplement

**For casual readers**:
- Clear topline results subsection
- Accessible abstract
- Self-explanatory figures
- Point-first style throughout

## Common Problems and Fixes

### Problem: Burying the lede
**Symptom**: Main finding appears in middle of paragraph or late in section.
**Fix**: Restructure to lead with conclusion, then support with evidence.

### Problem: Overemphasis of routine content
**Symptom**: "We performed a comprehensive analysis..." "It is important to note..."
**Fix**: Remove self-congratulatory language. Let findings speak for themselves.

### Problem: Lack of cohesion
**Symptom**: Jarring transitions, unclear logical flow.
**Fix**: Add linking sentences. Repeat keywords. Use "this/these" to reference previous concepts.

### Problem: Results subsection merely supports previous subsection
**Symptom**: Subsection doesn't introduce new question, just more evidence.
**Fix**: Merge subsections OR reframe to address distinct question.

### Problem: Introduction doesn't get to the point
**Symptom**: Multiple paragraphs of context before stating what paper is about.
**Fix**: Cut to the chase. State gap/question in first paragraph.

### Problem: Discussion just repeats Results
**Symptom**: No new insights, just summary.
**Fix**: Contextualize differently. Discuss implications, limitations, connections not in Intro.

### Problem: Figure caption is narrative
**Symptom**: "We computed X and found Y..."
**Fix**: "X values for Y dataset. Error bars represent Z."

### Problem: Method not reproducible
**Symptom**: Missing parameters, unclear procedure.
**Fix**: Add all details needed for exact replication. Justify choices.

## Final Reminders

1. **Write for your readers**: Convince reviewers, satisfy experts, engage casual readers
2. **Point-first**: Put conclusions before evidence
3. **Judicious emphasis**: Emphasize what matters, not what's obvious or self-serving
4. **Cohesiveness**: Connect sentences and paragraphs logically
5. **Embed in literature**: Throughout paper, not just Introduction
6. **Writing is thinking**: Use it as opportunity to think deeply about what you mean

## Output Format

When providing revision feedback:

1. **Summary** (3-5 sentences)
   - Overall assessment
   - Main strengths
   - Key areas for improvement

2. **Section-by-Section Feedback**
   - For each section present
   - Specific, actionable suggestions
   - Before/after examples where helpful

3. **Prioritized Action Items**
   - High/Medium/Low priority
   - Concrete next steps

4. **Checklist Status**
   - Section-specific criteria
   - What's working, what needs work
