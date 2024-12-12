// src/context/auth.rs
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub user_id: Option<String>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            user_id: None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct AuthContext {
    pub state: AuthState,
    pub login: Callback<String>, // Takes user_id
    pub logout: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn AuthProvider(props: &AuthProviderProps) -> Html {
    let state = use_state(AuthState::default);

    let login = {
        let state = state.clone();
        Callback::from(move |user_id: String| {
            state.set(AuthState {
                is_authenticated: true,
                user_id: Some(user_id),
            });
        })
    };

    let logout = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(AuthState::default());
        })
    };

    let context = AuthContext {
        state: (*state).clone(),
        login,
        logout,
    };

    html! {
        <ContextProvider<Rc<AuthContext>> context={Rc::new(context)}>
            { for props.children.iter() }
        </ContextProvider<Rc<AuthContext>>>
    }
}

// fn get_user_id() -> String {
//     use crate::context::auth::AuthContext;
//     use yew::prelude::*;

//     let auth_ctx = use_context::<Rc<AuthContext>>().expect("Could not find AuthContext");
//     auth_ctx.state.user_id.clone().unwrap_or_else(|| "".to_string())
// }
