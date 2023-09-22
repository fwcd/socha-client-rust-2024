use std::net::TcpStream;
use std::io::{self, BufWriter, BufReader, Read, Write};
use log::{info, warn, debug, error};
use quick_xml::events::{Event as XmlEvent, BytesStart};
use quick_xml::{Reader, Writer};
use crate::game::{State, Team, Move};
use crate::protocol::{Request, Event, GameResult, EventPayload, RequestPayload};
use crate::util::{Result, Element, Error};

/// A handler that implements the game player's
/// behavior, usually employing some custom move
/// selection strategy.
pub trait GameClientDelegate {
    /// Invoked whenever the game state updates.
    fn state_updated(&mut self, _state: &State) {}
    
    /// Invoked when the game ends.
    fn game_ended(&mut self, _result: &GameResult) {}
    
    /// Invoked when the welcome message is received
    /// with the player's team.
    fn welcome_received(&mut self, _team: Team) {}
    
    /// Requests a move from the delegate. This method
    /// should implement the "main" game logic.
    fn pick_move(&mut self, state: &State, my_team: Team) -> Move;
}

/// A configuration that determines whether
/// the reader and/or the writer of a stream
/// should be swapped by stdio to ease debugging.
pub struct DebugMode {
    pub debug_reader: bool,
    pub debug_writer: bool,
}

/// The client which handles XML requests, manages
/// the game state and invokes the delegate.
pub struct GameClient<D> where D: GameClientDelegate {
    delegate: D,
    debug_mode: DebugMode,
    reservation_code: Option<String>,
    // TODO: Add game state
}

impl<D> GameClient<D> where D: GameClientDelegate {
    /// Creates a new client using the specified delegate.
    pub fn new(delegate: D, debug_mode: DebugMode, reservation_code: Option<String>) -> Self {
        Self { delegate, debug_mode, reservation_code }
    }
    
    /// Blocks the thread and begins reading XML messages
    /// from the provided address via TCP.
    pub fn connect(self, host: &str, port: u16) -> Result<GameResult> {
        let address = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&address)?;
        info!("Connected to {}", address);
        
        // Begin parsing game messages from the stream.
        // List all combinations of modes explicitly,
        // since they generate different generic instantiations
        // of `run_game`.

        let mode = &self.debug_mode;
        let game_result = if mode.debug_reader && !mode.debug_writer {
            self.run(io::stdin(), stream)?
        } else if !mode.debug_reader && mode.debug_writer {
            self.run(stream, io::stdout())?
        } else if mode.debug_reader && mode.debug_writer {
            self.run(io::stdin(), io::stdout())?
        } else {
            self.run(stream.try_clone()?, stream)?
        };
        
        Ok(game_result)
    }
    
    /// Blocks the thread and parses/handles game messages
    /// from the provided reader.
    fn run(mut self, read: impl Read, write: impl Write) -> Result<GameResult> {
        let mut buf = Vec::new();
        let mut reader = Reader::from_reader(BufReader::new(read));
        let mut writer = Writer::new(BufWriter::new(write));

        // Write <protocol>
        writer.write_event(XmlEvent::Start(BytesStart::new("protocol")))?;
        
        // Send join request
        let join_xml: Element = match self.reservation_code {
            Some(code) => Request::JoinPrepared { reservation_code: code.to_owned() },
            None => Request::Join,
        }.into();
        info!("Sending join request {}", &join_xml);
        join_xml.write_to(&mut writer)?;

        // Read <protocol>
        loop {
            match reader.read_event_into(&mut buf)? {
                XmlEvent::Start(ref start) if start.name().as_ref() == b"protocol" => {
                    info!("Performed handshake");
                    break
                },
                XmlEvent::Text(_) => (),
                XmlEvent::Eof => return Err(Error::Eof),
                e => warn!("Got unexpected event {:?}", e),
            }
        }

        // Handle events from the server
        let mut state: Option<State> = None;
        let mut game_result: Option<GameResult> = None;
        loop {
            let event_xml = Element::read_from(&mut reader)?;

            debug!("Got event {}", event_xml);
            match Event::try_from(&event_xml) {
                Ok(Event::Joined { room_id }) => {
                    info!("Joined room {}", room_id);
                },
                Ok(Event::Left { room_id }) => {
                    info!("Left room {}", room_id);
                    break;
                },
                Ok(Event::Room { room_id, payload }) => {
                    info!("Got {} in room {}", payload, room_id);
                    match payload {
                        EventPayload::Welcome(team) => self.delegate.welcome_received(team),
                        EventPayload::GameResult(result) => {
                            self.delegate.game_ended(&result);
                            game_result = Some(result);
                        },
                        EventPayload::Memento(new_state) => {
                            self.delegate.state_updated(&new_state);
                            state = Some(new_state);
                        },
                        EventPayload::MoveRequest => {
                            let state = state.as_ref().ok_or_else(|| Error::InvalidState("No state available at move request!".to_owned()))?;
                            let team = state.current_team();
                            let new_move = self.delegate.pick_move(state, team);
                            let request = Request::Room { room_id, payload: RequestPayload::Move(new_move) };
                            let request_xml = Element::from(request);
                            request_xml.write_to(&mut writer)?;
                        },
                    };
                },
                Err(Error::UnknownElement(element)) => {
                    warn!("Got unknown tag <{}>: {}", element.name(), element);
                },
                Err(Error::ServerError(message)) => {
                    error!("Server error: {}", message);
                },
                Err(e) => {
                    warn!("Error while parsing event: {:?}", e);
                },
            }
        }

        if let Some(result) = game_result {
            Ok(result)
        }else {
            Err(Error::InvalidState("Failed to receive game_result".to_string()))
        }
    }
}
