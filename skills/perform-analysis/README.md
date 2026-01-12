# Perform Analysis Plugin

A comprehensive Claude Code skill that provides a systematic framework for performing data analyses and experiments.

## What It Does

When you ask Claude to perform a data analysis or run an experiment, this skill automatically guides Claude through an 8-step rigorous process:

1. **Understand Motivation** - Clarify why the question is being asked
2. **Set Expectations** - Determine expected results and confidence level
3. **Verify Resources** - Ensure all data and tools are available
4. **Make a Plan** - Create a step-by-step analysis plan
5. **Perform Analysis** - Execute the plan with appropriate monitoring
6. **Display Results** - Create tables/figures highlighting key findings
7. **Document Choices** - Explain decisions and challenges
8. **List Files** - Provide paths to all created files

## When to Use

This skill activates when you ask to:
- "Perform an analysis"
- "Run an experiment"
- "Analyze this data"
- "Test if X is correlated with Y"
- "Compute statistics on this dataset"
- "Run a model to predict..."

## Key Features

### Systematic Approach

The skill ensures Claude doesn't just run code blindly, but instead:
- Understands the scientific context
- Sets expectations before running analyses
- Validates data and tools exist
- Creates a clear plan
- Documents the entire process

### Resource Verification

Before starting, Claude will:
- Verify all required datasets are accessible
- Check that necessary tools are installed
- Ask you for file paths if needed
- Invoke the **new-software** skill for unfamiliar tools
- Invoke the **new-data** skill for new data (when available)

### Time Estimation

For long-running analyses, Claude will:
- Estimate how long the analysis will take
- Use sleep commands to wait appropriately
- Monitor progress periodically
- Update you on status

### Cluster Integration

When running on the O2 cluster:
- Detects its environment and invokes `use-o2` skill
- Estimates resource requirements and submits SLURM jobs
- Monitors long-running jobs and reports progress periodically

### Complete Documentation

After analysis, you'll receive:
- Clear visualization (table or figure)
- The single most important finding highlighted
- Explanation of methodological choices
- Challenges encountered and solutions
- Complete list of all files created (scripts, data, figures)

## Best Practices

To get the most out of this skill, you should evaluate whether the analysis is straightforward. For challenging analyses: 
- Include in your prompt a sketch of what approach the model should take
- Include the motivation for the analysis, or the expected results
- Ask the to pilot the analysis on a small test dataset
- If a nontrivial new feature must be implemented to perform the analysis, ask for the feature first, verify that it works using unit tests, and only then ask for the analysis
- Ask Claude to discuss its approach before launching; use `/plan` mode
- Use Opus

For straightforward analyses, you can possibly get away with a 1 or 2 sentence prompt, but you should still think about what context Claude does or does not have. It might be helpful, for example, to point it to a previously written script that implements a similar analysis.

## Contributing

Feel free to make improvements to this skill. You are encouraged to contribute these improvements back by making a pull request.
