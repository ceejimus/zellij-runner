use std::env;

use clap::Parser;

use crate::{action::Action, ui, zellij};

#[derive(Parser, Debug)]
#[clap(name = "zellij-runner")]
struct Args {
    #[arg(short, long)]
    session: Option<String>,
    #[arg(short, long)]
    layout: Option<String>,
}

pub(crate) fn init() {
    let args = Args::parse();

    let action = match zellij::list_sessions() {
        Err(error) => Action::Exit(Err(error)),
        Ok(sessions) => match (args.session, args.layout, sessions.as_slice()) {
            (None, layout, &[]) => {
                // TODO: add optional layout specification
                ui::new_session_prompt(sessions, layout)
            }
            (session, layout, &[]) => Action::CreateNewSession {
                session,
                layout,
                dir: None,
            },
            // TODO: make "prompt" arg
            // (None, None, &[]) => ui::new_session_prompt(sessions),
            (Some(session), layout, _) => {
                if sessions.contains(&session) {
                    // just ignore layout if attaching to session -- for now
                    // maybe there's a way to attach to a session then apply a new layout???
                    Action::AttachToSession(session)
                } else {
                    Action::CreateNewSession {
                        session: Some(session),
                        layout,
                        dir: None,
                    }
                }
            }
            (None, layout, _) => ui::action_selector(sessions, layout),
        },
    };

    action.exec()
}

pub(crate) fn switch() {
    let action = match zellij::list_sessions() {
        Err(error) => Action::Exit(Err(error)),
        Ok(sessions) => match sessions.as_slice() {
            // TODO: keep layout somewhere global and use?? prolly not
            &[] => ui::new_session_prompt(sessions, None),
            _ => ui::action_selector(sessions, None),
        },
    };

    action.exec()
}
