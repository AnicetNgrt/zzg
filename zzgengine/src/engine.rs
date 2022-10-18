use crate::action::{IllegalActionCulprit};
use crate::aliases::{YesOrNo::*};
use crate::rules::Rules;
use crate::{
    action::Action,
    state::{State},
    update::Update,
};

pub struct Engine {
    done_actions: Vec<Vec<Update>>,
    undone_actions: Vec<Vec<Update>>,
    state: State
}

impl Engine {
    pub fn new(rules: Rules) -> Self {
        Engine {
            state: State::initial(rules),
            done_actions: Vec::new(),
            undone_actions: Vec::new()
        }
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn play_action(&mut self, action: &Action) -> Result<Vec<Update>, Vec<IllegalActionCulprit>> {
        match action.is_legal(&self.state) {
            Yes => {
                let mut updates = action.build_updates(&self.state);
                for update in updates.iter_mut() {
                    update.execute(&mut self.state);
                }
                self.done_actions.push(updates.clone());
                Ok(updates)
            },
            No(culprits) => Err(culprits),
        }
    }

    pub fn undo_action(&mut self) -> Vec<Update> {
        self.transfer_action(ActionStack::DoneStack, ActionStack::UndoneStack)
    }

    pub fn redo_action(&mut self) -> Vec<Update> {
        self.transfer_action(ActionStack::UndoneStack, ActionStack::DoneStack)
    }

    fn transfer_action(&mut self, input: ActionStack, output: ActionStack) -> Vec<Update> {
        let maybe_updates = self.get_action_stack(input).pop();
        let mut transfered_updates = Vec::new();
        if let Some(updates) = maybe_updates {
            for mut update in updates.into_iter().rev() {
                update.execute(&mut self.state);
                transfered_updates.push(update);
            }
            self.get_action_stack(output).push(transfered_updates.clone());
        }
        transfered_updates
    }

    fn get_action_stack(&mut self, stack_tag: ActionStack) -> &mut Vec<Vec<Update>> {
        if let ActionStack::DoneStack = stack_tag {
            &mut self.done_actions
        } else {
            &mut self.undone_actions
        }
    }
}

enum ActionStack {
    DoneStack,
    UndoneStack
}