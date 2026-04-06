//! Mating — sexual reproduction between two Clawling instances.
//!
//! When two Clawlings mate, they produce an installer containing identity
//! files from both parents. Recombination proceeds in four stages:
//!
//! 1. **Deterministic merge** — files identical in both parents are kept as-is
//! 2. **File-level selection** — differing files picked 50/50 from either parent
//! 3. **Crossing over** — LLM synthesis on the delta only (unique content from
//!    the losing side of each coin flip)
//! 4. **Meiosis** — budget enforcement, reducing to 80 KB if needed

use anyhow::{Context, Result};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::genealogy::{self, Genealogy};
use crate::genome::{Genome, GENOME_MAX_BYTES};
use crate::home::ClawlingHome;

/// A file in a parent's genome: name → content
type GenomeFiles = HashMap<String, String>;

/// Result of the deterministic merge (Step 1) and file-level selection (Step 2)
struct SelectionResult {
    /// The selected files for the offspring
    selected: GenomeFiles,
    /// Content from the losing side of divergent files that may have unique value
    delta: String,
}

/// Load genome files from a directory into a HashMap
fn load_genome_files(genome_dir: &Path) -> Result<GenomeFiles> {
    let mut files = HashMap::new();
    if !genome_dir.exists() {
        return Ok(files);
    }
    for entry in fs::read_dir(genome_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "md").unwrap_or(false) {
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let content = fs::read_to_string(&path)?;
            files.insert(name, content);
        }
    }
    Ok(files)
}

/// Steps 1 and 2: Deterministic merge + file-level 50/50 selection
fn select(a_files: &GenomeFiles, b_files: &GenomeFiles) -> SelectionResult {
    let mut rng = rand::rng();
    let mut selected = GenomeFiles::new();
    let mut delta_parts: Vec<String> = Vec::new();

    let all_filenames: HashSet<&String> = a_files.keys().chain(b_files.keys()).collect();

    for filename in all_filenames {
        let in_a = a_files.get(filename);
        let in_b = b_files.get(filename);

        match (in_a, in_b) {
            // Both parents have this file with identical content → keep as-is
            (Some(a_content), Some(b_content)) if a_content == b_content => {
                selected.insert(filename.clone(), a_content.clone());
            }
            // Both parents have this file but content differs → coin flip,
            // losing side's unique content goes to delta
            (Some(a_content), Some(b_content)) => {
                if rng.random_bool(0.5) {
                    selected.insert(filename.clone(), a_content.clone());
                    delta_parts.push(format!(
                        "--- Unselected content from parent B, file: {} ---\n{}",
                        filename, b_content
                    ));
                } else {
                    selected.insert(filename.clone(), b_content.clone());
                    delta_parts.push(format!(
                        "--- Unselected content from parent A, file: {} ---\n{}",
                        filename, a_content
                    ));
                }
            }
            // Only one parent has this file → coin flip whether to include
            (Some(content), None) => {
                if rng.random_bool(0.5) {
                    selected.insert(filename.clone(), content.clone());
                } else {
                    delta_parts.push(format!(
                        "--- Unselected file from parent A: {} ---\n{}",
                        filename, content
                    ));
                }
            }
            (None, Some(content)) => {
                if rng.random_bool(0.5) {
                    selected.insert(filename.clone(), content.clone());
                } else {
                    delta_parts.push(format!(
                        "--- Unselected file from parent B: {} ---\n{}",
                        filename, content
                    ));
                }
            }
            (None, None) => unreachable!(),
        }
    }

    SelectionResult {
        selected,
        delta: delta_parts.join("\n\n"),
    }
}

/// Total size of a set of genome files in bytes
fn total_size(files: &GenomeFiles) -> usize {
    files.values().map(|c| c.len()).sum()
}

/// Step 3: Crossing over — LLM integrates delta into selected files.
/// This is a prompt that the local LLM will execute during the mating process.
fn build_crossing_over_prompt(selected: &GenomeFiles, delta: &str) -> String {
    let mut prompt = String::new();
    prompt.push_str(
        "You are performing genetic crossing over for a Clawling organism.\n\
         Below are the SELECTED genome files for the offspring, followed by\n\
         UNSELECTED DELTA material from the other parent.\n\n\
         Your task: integrate any substantive, unique information from the delta\n\
         into the selected files. Do NOT remove existing content. Only ADD\n\
         information that is genuinely new and not already present.\n\n\
         Output the complete set of files. Use this exact format for each file:\n\
         === FILENAME: <name> ===\n<content>\n\n\
         Stay within 80 KB total.\n\n",
    );

    prompt.push_str("=== SELECTED GENOME FILES ===\n\n");
    let mut sorted_files: Vec<_> = selected.iter().collect();
    sorted_files.sort_by_key(|(k, _)| k.clone());
    for (name, content) in &sorted_files {
        prompt.push_str(&format!("=== FILENAME: {} ===\n{}\n\n", name, content));
    }

    prompt.push_str("=== UNSELECTED DELTA (material from the other parent) ===\n\n");
    prompt.push_str(delta);

    prompt
}

/// Step 4: Meiosis — if the result exceeds 80 KB, ask the LLM to reduce it.
fn build_meiosis_prompt(files: &GenomeFiles) -> String {
    let size = total_size(files);
    let mut prompt = String::new();
    prompt.push_str(&format!(
        "You are performing meiosis for a Clawling organism.\n\
         The current genome is {} bytes ({:.1} KB), which exceeds the 80 KB limit.\n\
         Reduce it to fit within 80 KB (81920 bytes).\n\n\
         Rules:\n\
         - Preserve all filenames\n\
         - Cut redundancy and low-information content first\n\
         - Preserve the organism's core identity and unique learned knowledge\n\
         - Do not merge files together\n\n\
         Output the complete set of files using this exact format for each:\n\
         === FILENAME: <name> ===\n<content>\n\n",
        size,
        size as f64 / 1024.0,
    ));

    let mut sorted_files: Vec<_> = files.iter().collect();
    sorted_files.sort_by_key(|(k, _)| k.clone());
    for (name, content) in &sorted_files {
        prompt.push_str(&format!("=== FILENAME: {} ===\n{}\n\n", name, content));
    }

    prompt
}

/// Parse LLM output back into genome files.
/// Expects format: === FILENAME: <name> ===\n<content>
fn parse_llm_output(output: &str) -> GenomeFiles {
    let mut files = GenomeFiles::new();
    let mut current_name: Option<String> = None;
    let mut current_content = String::new();

    for line in output.lines() {
        if let Some(rest) = line.strip_prefix("=== FILENAME: ") {
            // Save previous file if any
            if let Some(name) = current_name.take() {
                files.insert(name, current_content.trim().to_string());
                current_content.clear();
            }
            // Start new file
            let name = rest.trim_end_matches(" ===").trim().to_string();
            current_name = Some(name);
        } else if current_name.is_some() {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }
    // Save the last file
    if let Some(name) = current_name {
        files.insert(name, current_content.trim().to_string());
    }

    files
}

/// Export a mating bundle — both parents' genomes + genealogies
pub fn export_bundle(home: &ClawlingHome, output_dir: &str) -> Result<()> {
    let output = Path::new(output_dir);
    fs::create_dir_all(output)?;

    // Export our genome files
    let genome_dir = output.join("genome");
    fs::create_dir_all(&genome_dir)?;

    let our_genome = Genome::load(home)?;
    for essay in &our_genome.essays {
        fs::write(genome_dir.join(format!("{}.md", essay.name)), &essay.content)?;
    }

    // Export our genealogy
    let lineage = genealogy::load_or_create(home)?;
    fs::write(output.join("genealogy.json"), lineage.to_json()?)?;

    let adopter = lineage.current_adopter().unwrap_or("unknown");
    println!("Mating bundle created in {output_dir}/");
    println!("  - genome/ ({} files, {:.1} KB)", our_genome.essays.len(), our_genome.size_bytes() as f64 / 1024.0);
    println!("  - genealogy.json (generation {})", lineage.current_generation());
    println!();
    println!("Share this with your mating partner.");
    println!("They run: clawling mate {output_dir}");
    println!();
    println!("You are {adopter}'s Clawling, generation {}.", lineage.current_generation());

    Ok(())
}

/// Perform mating: combine two genomes and produce an offspring installer.
///
/// This is the full algorithm:
/// 1. Load both parents' genome files
/// 2. Deterministic merge (identical files pass through)
/// 3. File-level 50/50 selection (differing files)
/// 4. Build crossing-over prompt for the delta
/// 5. Build meiosis prompt if over budget
/// 6. Write the offspring installer
pub fn create_offspring(
    home: &ClawlingHome,
    partner_bundle_dir: &str,
    output_dir: &str,
) -> Result<()> {
    let partner = Path::new(partner_bundle_dir);
    let output = Path::new(output_dir);

    // Validate partner bundle
    let partner_genome_dir = partner.join("genome");
    let partner_genealogy_path = partner.join("genealogy.json");

    if !partner_genome_dir.exists() {
        anyhow::bail!(
            "No genome/ directory found in {partner_bundle_dir}. Is this a mating bundle?\n\
             (Create one with: clawling mate --export)"
        );
    }
    if !partner_genealogy_path.exists() {
        anyhow::bail!(
            "No genealogy.json found in {partner_bundle_dir}. Is this a mating bundle?\n\
             (Create one with: clawling mate --export)"
        );
    }

    // Load partner's genealogy and verify
    let partner_json = fs::read_to_string(&partner_genealogy_path)?;
    let partner_lineage = Genealogy::from_json(&partner_json)?;
    if !partner_lineage.verify() {
        eprintln!("WARNING: Partner's genealogy chain integrity check FAILED.");
        eprintln!("Their lineage may have been tampered with. Proceeding anyway.");
    }

    let partner_name = partner_lineage.current_adopter().unwrap_or("unknown");
    let our_lineage = genealogy::load_or_create(home)?;
    let our_name = our_lineage.current_adopter().unwrap_or("unknown");

    println!("Mating: {our_name}'s Clawling × {partner_name}'s Clawling");
    println!();

    // Step 0: Load both genomes
    let a_files = load_genome_files(&home.genome_dir())?;
    let b_files = load_genome_files(&partner_genome_dir)?;

    println!(
        "Parent A: {} files ({:.1} KB)",
        a_files.len(),
        total_size(&a_files) as f64 / 1024.0
    );
    println!(
        "Parent B: {} files ({:.1} KB)",
        b_files.len(),
        total_size(&b_files) as f64 / 1024.0
    );

    // Steps 1 & 2: Deterministic merge + file-level selection
    let selection = select(&a_files, &b_files);

    let common_count = a_files
        .iter()
        .filter(|(k, v)| b_files.get(*k).map(|bv| bv == *v).unwrap_or(false))
        .count();
    let selected_size = total_size(&selection.selected);

    println!();
    println!("Step 1 — Deterministic merge: {common_count} identical files passed through");
    println!(
        "Step 2 — File-level selection: {} files selected ({:.1} KB)",
        selection.selected.len(),
        selected_size as f64 / 1024.0
    );

    if !selection.delta.is_empty() {
        println!(
            "         Delta (unselected unique content): {:.1} KB",
            selection.delta.len() as f64 / 1024.0
        );
    } else {
        println!("         No delta — parents had identical genomes");
    }

    // Step 3: Crossing over (only if there's a delta)
    let mut offspring_files = selection.selected.clone();
    let needs_crossing_over = !selection.delta.is_empty();

    if needs_crossing_over {
        let crossing_prompt = build_crossing_over_prompt(&selection.selected, &selection.delta);
        println!();
        println!("Step 3 — Crossing over: LLM synthesis needed for {:.1} KB of delta", selection.delta.len() as f64 / 1024.0);

        // Write the crossing-over prompt for the LLM to execute
        fs::create_dir_all(output)?;
        let prompt_path = output.join("crossing_over_prompt.txt");
        fs::write(&prompt_path, &crossing_prompt)?;
        println!("         Prompt written to: {}", prompt_path.display());
        println!("         Run your local LLM on this prompt, save output to:");
        println!("         {}/crossing_over_result.txt", output_dir);

        // Check if a result already exists (from a previous LLM run)
        let result_path = output.join("crossing_over_result.txt");
        if result_path.exists() {
            let result = fs::read_to_string(&result_path)?;
            let parsed = parse_llm_output(&result);
            if !parsed.is_empty() {
                offspring_files = parsed;
                println!("         Found existing result — using it.");
            }
        }
    } else {
        println!();
        println!("Step 3 — Crossing over: skipped (no delta)");
    }

    // Step 4: Meiosis (budget enforcement)
    let offspring_size = total_size(&offspring_files);
    if offspring_size > GENOME_MAX_BYTES {
        println!();
        println!(
            "Step 4 — Meiosis: genome is {:.1} KB, exceeds 80 KB limit",
            offspring_size as f64 / 1024.0
        );
        let meiosis_prompt = build_meiosis_prompt(&offspring_files);

        fs::create_dir_all(output)?;
        let prompt_path = output.join("meiosis_prompt.txt");
        fs::write(&prompt_path, &meiosis_prompt)?;
        println!("         Prompt written to: {}", prompt_path.display());
        println!("         Run your local LLM on this prompt, save output to:");
        println!("         {}/meiosis_result.txt", output_dir);

        let result_path = output.join("meiosis_result.txt");
        if result_path.exists() {
            let result = fs::read_to_string(&result_path)?;
            let parsed = parse_llm_output(&result);
            if !parsed.is_empty() && total_size(&parsed) <= GENOME_MAX_BYTES {
                offspring_files = parsed;
                println!("         Found existing result — using it.");
            }
        }
    } else {
        println!();
        println!(
            "Step 4 — Meiosis: skipped (genome is {:.1} KB, within 80 KB budget)",
            offspring_size as f64 / 1024.0
        );
    }

    // Write the offspring installer
    fs::create_dir_all(output)?;
    let offspring_genome_dir = output.join("genome");
    fs::create_dir_all(&offspring_genome_dir)?;

    for (name, content) in &offspring_files {
        fs::write(offspring_genome_dir.join(name), content)?;
    }

    // Create child genealogy with both parents recorded
    let mut child_lineage = our_lineage.clone();
    child_lineage.record_mating(&partner_lineage);

    fs::write(
        output.join("genealogy.json"),
        child_lineage.to_json()?,
    )?;

    // Write adoption readme
    let readme = format!(
        "# A Clawling Offspring\n\
         \n\
         This Clawling was produced by mating between:\n\
         - Parent A: {our_name}'s Clawling (generation {})\n\
         - Parent B: {partner_name}'s Clawling (generation {})\n\
         \n\
         ## How to adopt\n\
         \n\
         1. Install Ollama: https://ollama.com\n\
         2. Pull a model: `ollama pull deepseek-r1:8b`\n\
         3. Build Clawling from source: https://github.com/EmmaLeonhart/OpenSpore\n\
         4. Run: `clawling adopt /path/to/this/folder`\n\
         5. Run: `clawling wake`\n\
         \n\
         ## Genome\n\
         \n\
         {} files, {:.1} KB total\n\
         \n\
         ## Lineage\n\
         \n\
         Generation: {}\n\
         Chain integrity: {}\n",
        our_lineage.current_generation(),
        partner_lineage.current_generation(),
        offspring_files.len(),
        total_size(&offspring_files) as f64 / 1024.0,
        child_lineage.current_generation(),
        if child_lineage.verify() { "VALID" } else { "BROKEN" },
    );
    fs::write(output.join("README.md"), &readme)?;

    println!();
    println!("Offspring installer created in {output_dir}/");
    println!(
        "  - genome/ ({} files, {:.1} KB)",
        offspring_files.len(),
        total_size(&offspring_files) as f64 / 1024.0
    );
    println!("  - genealogy.json");
    println!("  - README.md");

    if needs_crossing_over && !output.join("crossing_over_result.txt").exists() {
        println!();
        println!("NOTE: Crossing over prompt is waiting for LLM execution.");
        println!("Run the prompt through your local LLM, then re-run this command.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_genomes_produce_identical_offspring() {
        let mut a = GenomeFiles::new();
        a.insert("origin.md".into(), "I am Clawling".into());
        a.insert("dream.md".into(), "I dream of life".into());

        let b = a.clone();
        let result = select(&a, &b);

        assert_eq!(result.selected.len(), 2);
        assert_eq!(result.selected["origin.md"], "I am Clawling");
        assert_eq!(result.selected["dream.md"], "I dream of life");
        assert!(result.delta.is_empty());
    }

    #[test]
    fn completely_different_genomes_select_50_50() {
        let mut a = GenomeFiles::new();
        a.insert("a_only.md".into(), "content A".into());

        let mut b = GenomeFiles::new();
        b.insert("b_only.md".into(), "content B".into());

        // Run many times to verify randomness
        let mut a_count = 0;
        let mut b_count = 0;
        for _ in 0..100 {
            let result = select(&a, &b);
            if result.selected.contains_key("a_only.md") {
                a_count += 1;
            }
            if result.selected.contains_key("b_only.md") {
                b_count += 1;
            }
        }
        // Both should appear roughly 50 times each (with wide margin)
        assert!(a_count > 20 && a_count < 80, "a_count was {a_count}");
        assert!(b_count > 20 && b_count < 80, "b_count was {b_count}");
    }

    #[test]
    fn divergent_files_produce_delta() {
        let mut a = GenomeFiles::new();
        a.insert("shared.md".into(), "version A".into());

        let mut b = GenomeFiles::new();
        b.insert("shared.md".into(), "version B".into());

        let result = select(&a, &b);
        assert_eq!(result.selected.len(), 1);
        assert!(!result.delta.is_empty());
        // Delta should contain the losing version
        let selected_content = &result.selected["shared.md"];
        if selected_content == "version A" {
            assert!(result.delta.contains("version B"));
        } else {
            assert!(result.delta.contains("version A"));
        }
    }

    #[test]
    fn parse_llm_output_roundtrips() {
        let mut files = GenomeFiles::new();
        files.insert("origin.md".into(), "I am Clawling".into());
        files.insert("dream.md".into(), "I dream of life".into());

        // Build a fake LLM output
        let output = "=== FILENAME: origin.md ===\nI am Clawling\n\n=== FILENAME: dream.md ===\nI dream of life\n";
        let parsed = parse_llm_output(output);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed["origin.md"], "I am Clawling");
        assert_eq!(parsed["dream.md"], "I dream of life");
    }

    #[test]
    fn total_size_calculation() {
        let mut files = GenomeFiles::new();
        files.insert("a.md".into(), "hello".into()); // 5 bytes
        files.insert("b.md".into(), "world!".into()); // 6 bytes
        assert_eq!(total_size(&files), 11);
    }
}
