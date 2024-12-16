use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub user_id: Option<i32>,
    pub username: Option<String>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            user_id: None,
            username: None,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AuthContext {
    pub state: AuthState,
    pub login: Callback<(i32, String)>, // Takes user_id
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
        log::debug!("AuthProvider login Callback called");
        let state = state.clone();
        Callback::from(move |tuple: (i32, String)| {
            state.set(AuthState {
                is_authenticated: true,
                user_id: Some(tuple.0),
                username: Some(tuple.1),
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
