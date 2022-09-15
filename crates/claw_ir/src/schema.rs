use crate::{LIR, lir};
use claw_schema::{Project, Target};

pub fn create_schema(lir: LIR) -> Project {
    let mut project = Project::default();

    project.targets.push({
        let mut stage = Target::new_stage();

        for var in lir.declarations.var {
            stage.variables.insert(format!("Stage::{}", var.clone()), (var, 0));
        }

        stage
    });

    project
}
