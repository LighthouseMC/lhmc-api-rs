#![feature(
    proc_macro_diagnostic,
    proc_macro_quote,
    let_chains
)]


use proc_macro::{ TokenStream, TokenTree, Ident, Span, quote };


#[proc_macro_attribute]
pub fn event(attr : TokenStream, item : TokenStream) -> TokenStream {
    let mut attr = attr.into_iter();

    // Get function.
    let mut item_iter = item.clone().into_iter().peekable();
    {
        // const
        if let Some(TokenTree::Ident(kw)) = item_iter.peek() && (kw.to_string() == "const") { item_iter.next(); }
        // async
        if let Some(TokenTree::Ident(kw)) = item_iter.peek() && (kw.to_string() == "async") { item_iter.next(); }
        // safe | unsafe
        if let Some(TokenTree::Ident(kw)) = item_iter.peek() && let kw = kw.to_string() && (kw == "safe" || kw == "unsafe") { item_iter.next(); }
        // extern
        if let Some(TokenTree::Ident(kw)) = item_iter.peek() && (kw.to_string() == "extern") {
            item_iter.next();
            // abi
            if let Some(TokenTree::Literal(_)) = item_iter.peek() { item_iter.next(); }
        }
        // fn
        if let Some(TokenTree::Ident(kw)) = item_iter.peek() && (kw.to_string() == "fn") { item_iter.next(); }
    }
    let Some(TokenTree::Ident(function_name)) = item_iter.next() else {
        Span::call_site()
            .error("`#[event(...)]` may only be applied to valid functions")
            .emit();
        return item;
    };

    // Get event name.
    let (Some(TokenTree::Ident(event_name)), None,) = (attr.next(), attr.next(),) else {
        Span::call_site()
            .error("a single event name ident is required.")
            .help("add an event name: `#[event(player_join)]`")
            .emit();
        return item;
    };
    let event_name_string = event_name.to_string();

    let add_item = match (event_name_string.as_str()) {


        "player_join" => {
            let player = Ident::new("player", event_name.span());
            quote!{ #[unsafe(no_mangle)] pub unsafe fn _vx_on_player_join(session_id : u64) -> () {
                let $player = ::lhmc_api_rs::player::Player::from_session_id(session_id);
                $function_name($player)
            } }
        },

        "player_leave" => {
            let player = Ident::new("player", event_name.span());
            quote!{ #[unsafe(no_mangle)] pub unsafe fn _vx_on_player_leave(session_id : u64) -> () {
                let $player = ::lhmc_api_rs::player::Player::from_session_id(session_id);
                $function_name($player)
            } }
        },

        "player_chat" => {
            let player = Ident::new("player", event_name.span());
            let msg    = Ident::new("msg",    event_name.span());
            quote!{ #[unsafe(no_mangle)] pub unsafe fn _vx_on_player_chat(session_id : u64, msg_ptr : u32, msg_size : u32) -> () {
                let $player = ::lhmc_api_rs::player::Player::from_session_id(session_id);
                let $msg    = unsafe { ::std::string::String::from_raw_parts(msg_ptr as *mut u8, msg_size as usize, msg_size as usize) };
                $function_name($player, $msg)
            } }
        },


        _ => {
            event_name.span()
                .error(format!("unknown event {:?}", event_name_string)) // TODO: Add docs link
                .emit();
            return item;
        }
    };

    quote!{ $item $add_item }
}
