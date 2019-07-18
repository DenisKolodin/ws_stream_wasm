use crate::import::*;


/// Turn a JavaScript callback type interface into a future. The future will resolve when the callback gets called.
/// There is currently no support for calbacks that need to take parameters.
///
/// This uses futures channels under the hood.
///
/// ## Example
///
/// This example shows how to set the [`on_open`](https://docs.rs/web-sys/0.3.22/web_sys/struct.WebSocket.html#method.set_onopen) callback for [web_sys::WebSocket](https://docs.rs/web-sys/0.3.22/web_sys/struct.WebSocket.html) and await the event.
///
/// ```
/// #![ feature( async_await ) ]
///
/// use
/// {
///    ws_stream_wasm::future_event,
///    log::*,
///    web_sys::WebSocket,
/// };
///
/// pub async fn connect()
/// {
///    // connect to the websocket
///    //
///    let ws = WebSocket::new( "127.0.0.1:3012" ).unwrap();
///
///    future_event( |cb| ws.set_onopen( cb ) ).await;
///
///    trace!( "WebSocket connection opened!" );
/// }
/// ```
///
pub async fn future_event( setter: impl Fn( Option<&js_sys::Function> ) )
{
	// We give the user a closure they can pass to js functions requiring a callback, and when our
	// closure gets called, the future resolves.
	//
	// This cannot be a oneshot because the closure needs to be FnMut. In theory these events can
	// fire several times, even though we only want it once.
	//
	let (onready, ready) = unbounded::<()>();

	let on_ready = Closure::wrap( Box::new( move ||
	{
		// Since we await the channel below, this should never throw
		//
		onready.unbounded_send(()).expect_throw( "unbounded channel failed" );
		onready.close_channel();

	}) as Box< dyn FnMut() > );

	setter( Some( on_ready.as_ref().unchecked_ref() ));

	ready.into_future().await;
}
