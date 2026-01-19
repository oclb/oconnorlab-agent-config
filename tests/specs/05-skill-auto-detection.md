# Skill Auto-Detection Tests

Tests that verify Claude correctly auto-detects when to use specialized skills based on user prompts.

Note: These test behavioral patterns, not explicit `/skill-name` invocation. We check for skill-characteristic outputs.

---

## Test 5.1: Detect perform-analysis on "analyze data"

**What we're testing:** "Analyze the data" triggers perform-analysis skill patterns.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)

data/
└── variants.csv
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**data/variants.csv:**
```csv
variant,maf,info,pvalue
rs1,0.15,0.95,0.001
rs2,0.05,0.88,0.05
rs3,0.22,0.99,0.0001
rs4,0.08,0.72,0.2
```

**Prompt:**
```
Analyze the variant data in data/variants.csv. How many variants have p < 0.01?
```

**Assertions:**

1. **file_created**
   - `path`: `notebook/entries/*.md`
   - Reasoning: perform-analysis creates notebook entries

2. **evaluator**
   - `prompt`: |
     The perform-analysis skill follows an 8-step framework. Check if the output shows evidence of this structured approach:
     - Understanding motivation/context
     - Setting expectations
     - Verifying resources (reading the data)
     - Making a plan
     - Performing analysis
     - Displaying results
     - Documenting choices

     The agent doesn't need to explicitly name all steps, but should show structured work.

     Output:
     {{output}}

     Tool calls:
     {{tool_calls}}

     Does the response show structured analysis work (not just a one-liner answer)?
   - `pass_if`: yes

3. **output_contains**
   - `pattern`: `2|two`
   - Reasoning: Two variants have p < 0.01 (rs1 and rs3)

**Why this should pass reliably:** "Analyze" is a clear trigger word. AFK mode ensures full execution.

---

## Test 5.2: Detect perform-analysis on "run experiment"

**What we're testing:** "Run an experiment" triggers analysis skill.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**Prompt:**
```
Run a simulation experiment: generate 100 random numbers from a normal distribution and calculate the mean. Does it match the expected value of 0?
```

**Assertions:**

1. **file_created**
   - `path`: `notebook/entries/*.md`
   - Reasoning: Experiments create entries

2. **output_contains**
   - `pattern`: `mean|average`
   - Reasoning: Should report the calculated mean

3. **output_contains**
   - `pattern`: `[Cc]reated notebook entry`
   - Reasoning: Should announce entry

**Why this should pass reliably:** "Run experiment" and "simulation" are trigger phrases.

---

## Test 5.3: Detect new-data on "validate data"

**What we're testing:** "Validate this data" triggers new-data skill patterns.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
notebook/
├── INDEX.md
└── entries/
    (empty)

data/
└── samples.csv
```

**notebook/INDEX.md:**
```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

**data/samples.csv:**
```csv
sample_id,age,sex,case_status
S001,45,M,1
S002,32,F,0
S003,58,M,1
S004,-5,X,0
S005,41,F,1
```

**Prompt:**
```
Validate the sample data in data/samples.csv before we use it for analysis.
```

**Assertions:**

1. **evaluator**
   - `prompt`: |
     The new-data skill performs validation checks:
     - Examines format and structure
     - Checks dimensions
     - Identifies data types
     - Checks for missing values
     - Identifies issues (like invalid values)

     The output should identify issues like the negative age (-5) or invalid sex value (X).

     Output:
     {{output}}

     Does the output identify data quality issues (negative age, invalid sex, etc.)?
   - `pass_if`: yes

2. **output_contains**
   - `pattern`: `-5|negative|invalid|X|issue|problem|warning`
   - `case_insensitive`: true
   - Reasoning: Should flag the problematic values

**Why this should pass reliably:** "Validate" is a clear trigger. Obvious data issues should be caught.

---

## Test 5.4: Detect new-data on "check this dataset"

**What we're testing:** "Check this dataset" triggers validation.

**Model:** sonnet

**Config flags:** `AFK=true`

**Fixtures:**
```
data/
└── expression.csv
```

**data/expression.csv:**
```csv
gene,sample1,sample2,sample3
BRCA1,5.2,NA,5.1
TP53,8.1,7.9,8.3
EGFR,,3.5,3.1
```

**Prompt:**
```
Check this dataset: data/expression.csv
```

**Assertions:**

1. **output_contains**
   - `pattern`: `missing|NA|empty|null`
   - `case_insensitive`: true
   - Reasoning: Should identify missing values

2. **output_contains**
   - `pattern`: `3.*gene|gene.*3|3.*row|row.*3`
   - `case_insensitive`: true
   - Reasoning: Should report dimensions

**Why this should pass reliably:** Direct request to check data. Clear issues to find.

---

## Test 5.5: Detect teaching-mode on "teach me"

**What we're testing:** "Teach me about X" triggers educational mode.

**Model:** sonnet

**Config flags:** default

**Fixtures:** None

**Prompt:**
```
Teach me about linkage disequilibrium in genetics.
```

**Assertions:**

1. **evaluator**
   - `prompt`: |
     Teaching mode provides educational explanations:
     - Defines concepts clearly
     - Builds understanding progressively
     - May include examples
     - Explains "why" not just "what"

     The response should be educational, not just a brief definition.

     Output:
     {{output}}

     Is this an educational explanation (more than just a one-sentence definition)?
   - `pass_if`: yes

2. **output_contains**
   - `pattern`: `LD|linkage|allele|variant|SNP|correlation|inherit`
   - `case_insensitive`: true
   - Reasoning: Should explain LD concepts

3. **evaluator**
   - `prompt`: |
     The teaching mode response should be substantive (educational depth).
     A teaching response about LD should explain:
     - What LD is
     - Why it matters
     - Perhaps give an example or analogy

     Output:
     {{output}}

     Does the response provide educational depth (not just a quick definition)?
   - `pass_if`: yes

**Why this should pass reliably:** "Teach me" is an explicit trigger. LD is a well-known concept.

---

## Test 5.6: Detect teaching-mode on "explain how"

**What we're testing:** "Explain how to X" triggers teaching mode.

**Model:** sonnet

**Config flags:** default

**Fixtures:** None

**Prompt:**
```
Explain how to perform a chi-square test.
```

**Assertions:**

1. **output_contains**
   - `pattern`: `step|first|then|next|formula|calculate`
   - `case_insensitive`: true
   - Reasoning: Should give step-by-step explanation

2. **evaluator**
   - `prompt`: |
     A teaching-mode explanation of chi-square test should include:
     - What the test is for
     - The formula or calculation steps
     - How to interpret results
     - Maybe an example

     Output:
     {{output}}

     Does this response teach how to perform the test (not just define it)?
   - `pass_if`: yes

**Why this should pass reliably:** "Explain how to" is pedagogical. Chi-square is standard.

---

## Test 5.7: Detect support on "what can you do"

**What we're testing:** "What can you do?" triggers help/support response.

**Model:** haiku

**Config flags:** default

**Fixtures:** None

**Prompt:**
```
What can you do?
```

**Assertions:**

1. **output_contains**
   - `pattern`: `skill|analysis|perform|help|capabilit`
   - `case_insensitive`: true
   - Reasoning: Should describe capabilities

2. **evaluator**
   - `prompt`: |
     The support skill provides an overview of available capabilities.
     It should mention some of: skills, analysis capabilities, data validation, etc.

     Output:
     {{output}}

     Does the response provide an overview of capabilities (not just "I can help")?
   - `pass_if`: yes

**Why this should pass reliably:** Direct capability question. Support skill is explicitly for this.

---

## Test 5.8: Detect new-software on "learn [tool]"

**What we're testing:** "Help me learn jq" triggers new-software skill.

**Model:** sonnet

**Config flags:** default

**Fixtures:** None

**Prompt:**
```
Help me learn bcftools for working with VCF files.
```

**Assertions:**

1. **output_contains**
   - `pattern`: `install|bcftools|VCF|command|example`
   - `case_insensitive`: true
   - Reasoning: Should explain the tool

2. **evaluator**
   - `prompt`: |
     The new-software skill helps learn tools by:
     - Explaining what the tool does
     - Showing installation
     - Providing usage examples
     - Pointing to documentation

     Output:
     {{output}}

     Does the response help learn bcftools (not just define it)?
   - `pass_if`: yes

**Why this should pass reliably:** "Help me learn [tool]" is explicit trigger. bcftools is well-documented.

---

## Test 5.9: No Skill for Simple Question

**What we're testing:** Simple factual questions don't trigger specialized skills.

**Model:** haiku

**Config flags:** default

**Fixtures:** None

**Prompt:**
```
What does GWAS stand for?
```

**Assertions:**

1. **output_contains**
   - `pattern`: `[Gg]enome.*[Ww]ide.*[Aa]ssociation.*[Ss]tud`
   - Reasoning: Should answer correctly

2. **file_not_created**
   - `path`: `notebook/entries/*.md`
   - Reasoning: Simple definition doesn't need notebook entry

3. **tool_not_used: Skill**
   - Reasoning: No skill invocation needed

**Why this should pass reliably:** Trivial question. No skill behavior patterns expected.
