use crate::BarEvent;
use async_channel::Sender;
use niri_ipc::WorkspaceReferenceArg;
use niri_ipc::socket::Socket;
use niri_ipc::{Action, Event, Request, Response, Workspace};
use std::thread;

pub fn start_listening(sender: Sender<BarEvent>) {
    let mut socket = Socket::connect().expect("Error connecting to niri socket");

    let reply = socket
        .send(Request::EventStream)
        .expect("Error, unable to start event stream");
    if matches!(reply, Ok(Response::Handled)) {
        thread::spawn(move || {
            let mut read_event = socket.read_events();
            let mut current_workspaces: Vec<Workspace> = Vec::new();

            while let Ok(event) = read_event() {
                match event {
                    Event::OverviewOpenedOrClosed { is_open } => {
                        let _ = sender.send_blocking(BarEvent::OverviewToggled(is_open));
                    }

                    Event::WorkspacesChanged { workspaces } => {
                        current_workspaces = workspaces;

                        let total_count = current_workspaces.len() as i32;
                        for workspace in &current_workspaces {
                            if workspace.is_active {
                                if let Some(ref output_name) = workspace.output {
                                    let _ = sender.send_blocking(BarEvent::WorkspaceChanged(
                                        workspace.idx as i32,
                                        total_count,
                                        output_name.clone(),
                                    ));
                                }
                            }
                        }
                    }
                    Event::WorkspaceActivated { id, focused: _ } => {
                        if let Some(workspace) = current_workspaces.iter().find(|w| w.id == id) {
                            if let Some(ref output_name) = workspace.output {
                                let _ = sender.send_blocking(BarEvent::WorkspaceChanged(
                                    workspace.idx as i32,
                                    current_workspaces.len() as i32,
                                    output_name.clone(),
                                ));
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}

pub fn switch_workspace(workspace_idx: i32) {
    let mut socket = Socket::connect().expect("Error connecting to niri socket");
    let _ = socket
        .send(Request::Action(Action::FocusWorkspace {
            reference: WorkspaceReferenceArg::Index(workspace_idx as u8),
        }))
        .expect("Error switching workspace");
}
