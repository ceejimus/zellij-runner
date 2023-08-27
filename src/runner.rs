use crate::{action::Action, ui, zellij, Args};

pub(crate) fn init(args: Args) {
    let action = match zellij::list_sessions() {
        Err(error) => Action::Exit(Err(error)),
        Ok(sessions) => match (args.session, args.layout, sessions.as_slice()) {
            (None, layout, &[]) => {
                // TODO: add optional layout specification
                ui::new_session_prompt(sessions, layout, args.chdir)
            }
            (session, layout, &[]) => Action::CreateNewSession {
                session,
                layout,
                dir: None,
            },
            // TODO: make "prompt" arg ?
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

pub(crate) fn switch(args: Args) {
    // Note: probably not the best way to do this, but it's fine
    let action = match zellij::list_sessions() {
        Err(error) => Action::Exit(Err(error)),
        Ok(sessions) => match sessions.as_slice() {
            // TODO: keep layout somewhere global and use?? prolly not
            &[] => ui::new_session_prompt(sessions, None, args.chdir),
            _ => ui::action_selector(sessions, None),
        },
    };

    action.exec()
}
