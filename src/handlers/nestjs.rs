use crate::{cli::Linter, templates::TemplateCopier};
use std::path::PathBuf;

pub fn handle_nestjs(cli: crate::cli::Cli) {
    let template_dir = match cli.linter {
        Some(Linter::Eslint) => "templates/nestjs/eslint",
        Some(Linter::Biome) => "templates/nestjs/biome",
        _ => "templates/nestjs/eslint",
    };

    let output_dir = PathBuf::from(&cli.output_path).join(&cli.project_name);
    let copier = TemplateCopier::new(
        template_dir,
        output_dir.to_str().expect("Invalid output path"),
    );

    copier.copy().expect("Failed to generate Nestjs project");
}